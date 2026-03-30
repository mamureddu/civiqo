#!/bin/bash
# =============================================================================
# Civiqo Community Manager — VPS Setup Script
# =============================================================================
# Provisions a fresh Ubuntu VPS with everything needed to run the application.
# Idempotent: safe to re-run.
#
# Usage: ./setup-server.sh
# =============================================================================

set -euo pipefail

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

info()  { echo -e "${BLUE}[INFO]${NC} $*"; }
ok()    { echo -e "${GREEN}[OK]${NC} $*"; }
warn()  { echo -e "${YELLOW}[WARN]${NC} $*"; }
error() { echo -e "${RED}[ERROR]${NC} $*"; exit 1; }

# =============================================================================
# Phase 0: Gather user input
# =============================================================================

echo ""
echo -e "${BLUE}============================================${NC}"
echo -e "${BLUE}  Civiqo Community Manager — VPS Setup${NC}"
echo -e "${BLUE}============================================${NC}"
echo ""

read -rp "VPS IP address: " VPS_IP
read -rp "SSH user (default: root): " SSH_USER
SSH_USER=${SSH_USER:-root}
read -rp "SSH key path (default: ~/.ssh/id_ed25519): " SSH_KEY
SSH_KEY=${SSH_KEY:-~/.ssh/id_ed25519}
read -rp "Domain name (e.g., community.example.com): " DOMAIN
read -rp "Email for Let's Encrypt (for HTTPS certificates): " LE_EMAIL

# Expand tilde
SSH_KEY="${SSH_KEY/#\~/$HOME}"

echo ""
info "Configuration:"
echo "  VPS IP:    $VPS_IP"
echo "  SSH User:  $SSH_USER"
echo "  SSH Key:   $SSH_KEY"
echo "  Domain:    $DOMAIN"
echo "  LE Email:  $LE_EMAIL"
echo ""
read -rp "Continue? (y/N) " CONFIRM
[[ "$CONFIRM" =~ ^[Yy]$ ]] || exit 0

SSH_CMD="ssh -i $SSH_KEY -o StrictHostKeyChecking=accept-new $SSH_USER@$VPS_IP"

# =============================================================================
# Phase 1: Test SSH connection
# =============================================================================

info "Testing SSH connection..."
$SSH_CMD "echo 'SSH connection successful'" || error "Cannot connect to $VPS_IP"
ok "SSH connection established"

# Detect OS
OS_VERSION=$($SSH_CMD "cat /etc/os-release | grep VERSION_ID | cut -d= -f2 | tr -d '\"'")
info "Detected Ubuntu $OS_VERSION"
if [[ ! "$OS_VERSION" =~ ^(22\.04|24\.04)$ ]]; then
    warn "Expected Ubuntu 22.04 or 24.04, got $OS_VERSION. Proceeding anyway..."
fi

# =============================================================================
# Phase 2: System hardening
# =============================================================================

info "Updating system packages..."
$SSH_CMD "DEBIAN_FRONTEND=noninteractive apt-get update -qq && apt-get upgrade -y -qq"
ok "System updated"

info "Installing base packages..."
$SSH_CMD "DEBIAN_FRONTEND=noninteractive apt-get install -y -qq \
    ufw fail2ban unattended-upgrades curl gnupg lsb-release"
ok "Base packages installed"

info "Creating application user..."
$SSH_CMD "id -u community-manager &>/dev/null || useradd -r -s /usr/sbin/nologin -d /opt/community-manager community-manager"
ok "User community-manager created"

info "Configuring firewall (UFW)..."
$SSH_CMD "ufw allow OpenSSH && ufw allow 80/tcp && ufw allow 443/tcp && ufw --force enable"
ok "Firewall configured (SSH, HTTP, HTTPS)"

info "Configuring fail2ban..."
$SSH_CMD "systemctl enable fail2ban && systemctl start fail2ban"
ok "fail2ban enabled"

info "Enabling automatic security updates..."
$SSH_CMD "dpkg-reconfigure -plow unattended-upgrades 2>/dev/null || true"
ok "Automatic updates configured"

# =============================================================================
# Phase 3: PostgreSQL 18
# =============================================================================

info "Installing PostgreSQL 18..."
$SSH_CMD bash -s << 'PGEOF'
set -e
# Add PostgreSQL APT repo
if [ ! -f /etc/apt/sources.list.d/pgdg.list ]; then
    curl -fsSL https://www.postgresql.org/media/keys/ACCC4CF8.asc | gpg --dearmor -o /usr/share/keyrings/postgresql-keyring.gpg
    echo "deb [signed-by=/usr/share/keyrings/postgresql-keyring.gpg] http://apt.postgresql.org/pub/repos/apt $(lsb_release -cs)-pgdg main" > /etc/apt/sources.list.d/pgdg.list
    apt-get update -qq
fi
DEBIAN_FRONTEND=noninteractive apt-get install -y -qq postgresql-18 || {
    echo "PostgreSQL 18 not available yet, trying 17..."
    DEBIAN_FRONTEND=noninteractive apt-get install -y -qq postgresql-17
}
PGEOF
ok "PostgreSQL installed"

# Generate random DB password
DB_PASSWORD=$(openssl rand -hex 24)

info "Configuring PostgreSQL database..."
$SSH_CMD bash -s "$DB_PASSWORD" << 'DBEOF'
set -e
DB_PASSWORD="$1"
sudo -u postgres psql -tc "SELECT 1 FROM pg_roles WHERE rolname='community_manager'" | grep -q 1 || \
    sudo -u postgres psql -c "CREATE USER community_manager WITH PASSWORD '$DB_PASSWORD'"
sudo -u postgres psql -tc "SELECT 1 FROM pg_database WHERE datname='community_manager'" | grep -q 1 || \
    sudo -u postgres psql -c "CREATE DATABASE community_manager OWNER community_manager"
sudo -u postgres psql -c "GRANT ALL PRIVILEGES ON DATABASE community_manager TO community_manager"
DBEOF
ok "Database configured"

# =============================================================================
# Phase 4: Caddy (reverse proxy + auto-HTTPS)
# =============================================================================

info "Installing Caddy..."
$SSH_CMD bash -s << 'CADDYEOF'
set -e
if ! command -v caddy &>/dev/null; then
    curl -fsSL https://getcaddy.com -o /tmp/getcaddy.sh || true
    apt-get install -y -qq debian-keyring debian-archive-keyring apt-transport-https curl
    curl -1sLf 'https://dl.cloudsmith.io/public/caddy/stable/gpg.key' | gpg --dearmor -o /usr/share/keyrings/caddy-stable-archive-keyring.gpg
    curl -1sLf 'https://dl.cloudsmith.io/public/caddy/stable/debian.deb.txt' | tee /etc/apt/sources.list.d/caddy-stable.list
    apt-get update -qq
    apt-get install -y -qq caddy
fi
CADDYEOF
ok "Caddy installed"

info "Configuring Caddy..."
CADDYFILE=$(cat deploy/config/Caddyfile.template | sed "s/{DOMAIN}/$DOMAIN/g")
echo "$CADDYFILE" | $SSH_CMD "cat > /etc/caddy/Caddyfile"
$SSH_CMD "mkdir -p /var/log/caddy && systemctl enable caddy && systemctl restart caddy"
ok "Caddy configured for $DOMAIN"

# =============================================================================
# Phase 5: Application directory structure
# =============================================================================

info "Creating application directories..."
$SSH_CMD bash -s << 'DIREOF'
set -e
mkdir -p /opt/community-manager/{bin,config,data/uploads,data/backups/daily,data/backups/weekly,templates,static,locales,migrations}
chown -R community-manager:community-manager /opt/community-manager
chmod 750 /opt/community-manager
chmod 700 /opt/community-manager/config
DIREOF
ok "Application directories created"

# =============================================================================
# Phase 6: Systemd services
# =============================================================================

info "Installing systemd services..."
scp -i "$SSH_KEY" deploy/config/community-manager.service "$SSH_USER@$VPS_IP:/etc/systemd/system/"
scp -i "$SSH_KEY" deploy/config/community-manager-chat.service "$SSH_USER@$VPS_IP:/etc/systemd/system/"
$SSH_CMD "systemctl daemon-reload && systemctl enable community-manager community-manager-chat"
ok "Systemd services installed"

# =============================================================================
# Phase 7: Environment file
# =============================================================================

JWT_SECRET=$(openssl rand -hex 32)
SESSION_SECRET=$(openssl rand -hex 32)

info "Generating environment file..."
ENV_CONTENT=$(cat deploy/config/.env.template \
    | sed "s/__DB_PASSWORD__/$DB_PASSWORD/g" \
    | sed "s/__JWT_SECRET__/$JWT_SECRET/g" \
    | sed "s/__SESSION_SECRET__/$SESSION_SECRET/g" \
    | sed "s/__DOMAIN__/$DOMAIN/g")
echo "$ENV_CONTENT" | $SSH_CMD "cat > /opt/community-manager/config/.env && chmod 600 /opt/community-manager/config/.env && chown community-manager:community-manager /opt/community-manager/config/.env"
ok "Environment file generated"

# =============================================================================
# Phase 8: Backup cron
# =============================================================================

info "Setting up backup cron..."
scp -i "$SSH_KEY" deploy/config/pg-backup.sh "$SSH_USER@$VPS_IP:/opt/community-manager/config/"
$SSH_CMD "chmod +x /opt/community-manager/config/pg-backup.sh"
$SSH_CMD "(crontab -u community-manager -l 2>/dev/null | grep -v pg-backup; echo '0 3 * * * /opt/community-manager/config/pg-backup.sh >> /opt/community-manager/data/backups/backup.log 2>&1') | crontab -u community-manager -"
ok "Daily backup cron configured (03:00 UTC)"

# =============================================================================
# Done!
# =============================================================================

echo ""
echo -e "${GREEN}============================================${NC}"
echo -e "${GREEN}  VPS Setup Complete!${NC}"
echo -e "${GREEN}============================================${NC}"
echo ""
echo "Next steps:"
echo "  1. Point DNS for $DOMAIN to $VPS_IP"
echo "  2. Run ./deploy/deploy.sh to build and deploy the application"
echo "  3. Caddy will auto-provision HTTPS once DNS propagates"
echo ""
echo "Credentials saved in /opt/community-manager/config/.env on the VPS"
echo "  DB Password: $DB_PASSWORD"
echo "  JWT Secret:  $JWT_SECRET"
echo ""
warn "Save these credentials securely — they are NOT stored locally."
