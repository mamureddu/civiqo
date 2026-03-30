#!/bin/bash
# =============================================================================
# Civiqo Community Manager — Build & Deploy Script
# =============================================================================
# Cross-compiles from macOS, transfers to VPS, restarts services.
#
# Usage: ./deploy/deploy.sh
# Prerequisites: cargo-zigbuild, zig, rustup target x86_64-unknown-linux-gnu
# =============================================================================

set -euo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

info()  { echo -e "${BLUE}[INFO]${NC} $*"; }
ok()    { echo -e "${GREEN}[OK]${NC} $*"; }
warn()  { echo -e "${YELLOW}[WARN]${NC} $*"; }
error() { echo -e "${RED}[ERROR]${NC} $*"; exit 1; }

TARGET="x86_64-unknown-linux-gnu"
PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"

# =============================================================================
# Phase 0: Configuration
# =============================================================================

echo ""
echo -e "${BLUE}============================================${NC}"
echo -e "${BLUE}  Civiqo Community Manager — Deploy${NC}"
echo -e "${BLUE}============================================${NC}"
echo ""

read -rp "VPS IP address: " VPS_IP
read -rp "SSH user (default: root): " SSH_USER
SSH_USER=${SSH_USER:-root}
read -rp "SSH key path (default: ~/.ssh/id_ed25519): " SSH_KEY
SSH_KEY=${SSH_KEY:-~/.ssh/id_ed25519}
SSH_KEY="${SSH_KEY/#\~/$HOME}"

SSH_CMD="ssh -i $SSH_KEY $SSH_USER@$VPS_IP"
RSYNC_CMD="rsync -avz --progress -e \"ssh -i $SSH_KEY\""

# =============================================================================
# Phase 1: Pre-flight checks
# =============================================================================

info "Running pre-flight checks..."

# Check cargo-zigbuild
command -v cargo-zigbuild &>/dev/null || error "cargo-zigbuild not found. Install: cargo install cargo-zigbuild"

# Check zig
command -v zig &>/dev/null || error "zig not found. Install: brew install zig"

# Check target
rustup target list --installed | grep -q "$TARGET" || {
    info "Installing Rust target $TARGET..."
    rustup target add "$TARGET"
}

# Test SSH
$SSH_CMD "echo ok" &>/dev/null || error "Cannot connect to $VPS_IP"

ok "Pre-flight checks passed"

# =============================================================================
# Phase 2: Build
# =============================================================================

info "Building server for $TARGET..."
cd "$PROJECT_ROOT"
cargo zigbuild --release --target "$TARGET" -p server 2>&1 | tail -5
ok "Server built"

info "Building chat-service for $TARGET..."
cargo zigbuild --release --target "$TARGET" -p chat-service 2>&1 | tail -5
ok "Chat service built"

# Show binary sizes
SERVER_SIZE=$(du -h "target/$TARGET/release/server" | cut -f1)
CHAT_SIZE=$(du -h "target/$TARGET/release/chat-service" | cut -f1)
info "Binary sizes: server=$SERVER_SIZE, chat-service=$CHAT_SIZE"

# =============================================================================
# Phase 3: Transfer
# =============================================================================

info "Transferring binaries..."
scp -i "$SSH_KEY" "target/$TARGET/release/server" "$SSH_USER@$VPS_IP:/opt/community-manager/bin/server.new"
scp -i "$SSH_KEY" "target/$TARGET/release/chat-service" "$SSH_USER@$VPS_IP:/opt/community-manager/bin/chat-service.new"
ok "Binaries transferred"

info "Transferring templates and assets..."
rsync -avz --delete -e "ssh -i $SSH_KEY" \
    src/server/templates/ "$SSH_USER@$VPS_IP:/opt/community-manager/templates/"
rsync -avz --delete -e "ssh -i $SSH_KEY" \
    src/server/static/ "$SSH_USER@$VPS_IP:/opt/community-manager/static/"
rsync -avz --delete -e "ssh -i $SSH_KEY" \
    src/server/locales/ "$SSH_USER@$VPS_IP:/opt/community-manager/locales/"
rsync -avz --delete -e "ssh -i $SSH_KEY" \
    src/migrations/ "$SSH_USER@$VPS_IP:/opt/community-manager/migrations/"
ok "Assets transferred"

# =============================================================================
# Phase 4: Swap binaries and restart
# =============================================================================

info "Swapping binaries and restarting services..."
$SSH_CMD bash -s << 'RESTARTEOF'
set -e
# Swap binaries atomically
mv /opt/community-manager/bin/server.new /opt/community-manager/bin/server
mv /opt/community-manager/bin/chat-service.new /opt/community-manager/bin/chat-service
chmod +x /opt/community-manager/bin/server /opt/community-manager/bin/chat-service
chown community-manager:community-manager /opt/community-manager/bin/*

# Fix ownership on transferred files
chown -R community-manager:community-manager /opt/community-manager/templates /opt/community-manager/static /opt/community-manager/locales /opt/community-manager/migrations

# Restart services
systemctl restart community-manager
systemctl restart community-manager-chat

# Wait for startup
sleep 2

# Check status
systemctl is-active --quiet community-manager && echo "API server: running" || echo "API server: FAILED"
systemctl is-active --quiet community-manager-chat && echo "Chat service: running" || echo "Chat service: FAILED"
RESTARTEOF
ok "Services restarted"

# =============================================================================
# Phase 5: Health check
# =============================================================================

info "Running health check..."
sleep 3

DOMAIN=$($SSH_CMD "grep CORS_ORIGINS /opt/community-manager/config/.env | cut -d= -f2 | sed 's|https://||'")

if curl -sf --max-time 10 "http://$VPS_IP:9001/health" &>/dev/null; then
    ok "Health check passed (direct)"
elif [ -n "$DOMAIN" ] && curl -sf --max-time 10 "https://$DOMAIN/health" &>/dev/null; then
    ok "Health check passed (via domain)"
else
    warn "Health check failed — checking logs..."
    $SSH_CMD "journalctl -u community-manager -n 20 --no-pager"
fi

echo ""
echo -e "${GREEN}============================================${NC}"
echo -e "${GREEN}  Deploy Complete!${NC}"
echo -e "${GREEN}============================================${NC}"
echo ""
