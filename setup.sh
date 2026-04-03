#!/usr/bin/env bash
set -euo pipefail

# Civiqo - Interactive Setup Script
# Usage: ./setup.sh
# Or:    curl -sSL https://raw.githubusercontent.com/mamureddu/civiqo/main/setup.sh | bash

BOLD='\033[1m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

RUST_MIN_VERSION="1.75.0"
PG_MIN_VERSION="15"
NODE_MIN_VERSION="18"

print_header() {
    echo ""
    echo -e "${BLUE}${BOLD}========================================${NC}"
    echo -e "${BLUE}${BOLD}  Civiqo - Community Manager Setup${NC}"
    echo -e "${BLUE}${BOLD}========================================${NC}"
    echo ""
}

info()    { echo -e "${GREEN}[+]${NC} $1"; }
warn()    { echo -e "${YELLOW}[!]${NC} $1"; }
error()   { echo -e "${RED}[x]${NC} $1"; }
ask()     { echo -e -n "${BOLD}[?]${NC} $1 "; }

confirm() {
    ask "$1 [Y/n]"
    read -r reply
    [[ -z "$reply" || "$reply" =~ ^[Yy] ]]
}

# Compare semver: returns 0 if $1 >= $2
version_gte() {
    [ "$(printf '%s\n' "$1" "$2" | sort -V | head -n1)" = "$2" ]
}

detect_os() {
    if [[ "$OSTYPE" == "darwin"* ]]; then
        OS="macos"
        if command -v brew &>/dev/null; then
            PKG_MANAGER="brew"
        else
            PKG_MANAGER="none"
        fi
    elif [[ -f /etc/debian_version ]]; then
        OS="debian"
        PKG_MANAGER="apt"
    elif [[ -f /etc/redhat-release ]]; then
        OS="redhat"
        PKG_MANAGER="dnf"
    elif [[ -f /etc/arch-release ]]; then
        OS="arch"
        PKG_MANAGER="pacman"
    else
        OS="unknown"
        PKG_MANAGER="none"
    fi
    info "Detected OS: $OS (package manager: $PKG_MANAGER)"
}

# --- Standalone mode: clone repo if not in one ---
setup_repo() {
    if [[ ! -f "Cargo.toml" ]] || ! grep -q "civiqo\|community-manager" Cargo.toml 2>/dev/null; then
        info "Not inside the Civiqo repository. Cloning..."
        git clone https://github.com/mamureddu/civiqo.git
        cd civiqo
        info "Cloned into $(pwd)"
    fi
}

# --- Prerequisite checks and installation ---

check_rust() {
    info "Checking Rust..."
    if command -v rustc &>/dev/null; then
        local version
        version=$(rustc --version | grep -oE '[0-9]+\.[0-9]+\.[0-9]+')
        if version_gte "$version" "$RUST_MIN_VERSION"; then
            info "Rust $version found (>= $RUST_MIN_VERSION)"
            return 0
        else
            warn "Rust $version found but >= $RUST_MIN_VERSION is required"
        fi
    else
        warn "Rust is not installed"
    fi

    if confirm "Install/update Rust via rustup?"; then
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
        info "Rust $(rustc --version | grep -oE '[0-9]+\.[0-9]+\.[0-9]+') installed"
    else
        error "Rust >= $RUST_MIN_VERSION is required. Install from https://rustup.rs"
        exit 1
    fi
}

check_postgresql() {
    info "Checking PostgreSQL..."
    if command -v psql &>/dev/null; then
        local version
        version=$(psql --version | grep -oE '[0-9]+' | head -1)
        if version_gte "$version" "$PG_MIN_VERSION"; then
            info "PostgreSQL $version found (>= $PG_MIN_VERSION)"
            return 0
        else
            warn "PostgreSQL $version found but >= $PG_MIN_VERSION is required"
        fi
    else
        warn "PostgreSQL is not installed"
    fi

    if confirm "Install PostgreSQL?"; then
        case $PKG_MANAGER in
            brew)
                brew install postgresql@18
                brew services start postgresql@18
                ;;
            apt)
                sudo apt update && sudo apt install -y postgresql postgresql-contrib
                sudo systemctl start postgresql
                sudo systemctl enable postgresql
                ;;
            dnf)
                sudo dnf install -y postgresql-server postgresql-contrib
                sudo postgresql-setup --initdb
                sudo systemctl start postgresql
                sudo systemctl enable postgresql
                ;;
            pacman)
                sudo pacman -S --noconfirm postgresql
                sudo -u postgres initdb -D /var/lib/postgres/data
                sudo systemctl start postgresql
                sudo systemctl enable postgresql
                ;;
            *)
                error "Cannot auto-install PostgreSQL on this system."
                error "Install PostgreSQL >= $PG_MIN_VERSION manually and re-run this script."
                exit 1
                ;;
        esac
        info "PostgreSQL installed and started"
    else
        error "PostgreSQL >= $PG_MIN_VERSION is required. Install it manually and re-run."
        exit 1
    fi
}

check_node() {
    info "Checking Node.js (for Tailwind CSS)..."
    if command -v node &>/dev/null; then
        local version
        version=$(node --version | grep -oE '[0-9]+' | head -1)
        if version_gte "$version" "$NODE_MIN_VERSION"; then
            info "Node.js v$version found (>= $NODE_MIN_VERSION)"
            return 0
        else
            warn "Node.js v$version found but >= $NODE_MIN_VERSION is required"
        fi
    else
        warn "Node.js is not installed"
    fi

    if confirm "Install Node.js?"; then
        case $PKG_MANAGER in
            brew)    brew install node ;;
            apt)     sudo apt update && sudo apt install -y nodejs npm ;;
            dnf)     sudo dnf install -y nodejs npm ;;
            pacman)  sudo pacman -S --noconfirm nodejs npm ;;
            *)
                error "Cannot auto-install Node.js. Install from https://nodejs.org"
                exit 1
                ;;
        esac
        info "Node.js $(node --version) installed"
    else
        error "Node.js >= $NODE_MIN_VERSION is required for Tailwind CSS. Install it manually."
        exit 1
    fi
}

# --- Database setup ---

setup_database() {
    info "Setting up database..."
    local db_name="civiqo_dev"
    local db_user

    # Detect current system user for PostgreSQL peer auth
    db_user=$(whoami)

    # Check if database exists
    if psql -lqt 2>/dev/null | cut -d \| -f 1 | grep -qw "$db_name"; then
        info "Database '$db_name' already exists"
    else
        if confirm "Create database '$db_name'?"; then
            createdb "$db_name" 2>/dev/null || sudo -u postgres createdb "$db_name" 2>/dev/null || {
                error "Could not create database. Create it manually:"
                echo "  createdb $db_name"
                exit 1
            }
            info "Database '$db_name' created"
        fi
    fi

    echo "postgresql://${db_user}@localhost:5432/${db_name}"
}

# --- Environment configuration ---

setup_env() {
    local db_url="$1"

    if [[ -f src/.env ]]; then
        warn "src/.env already exists"
        if ! confirm "Overwrite it?"; then
            info "Keeping existing src/.env"
            return 0
        fi
    fi

    # Generate secure JWT secret
    local jwt_secret
    if command -v openssl &>/dev/null; then
        jwt_secret=$(openssl rand -base64 32)
    else
        jwt_secret=$(head -c 32 /dev/urandom | base64)
    fi

    cp src/.env.example src/.env

    # Replace values in .env
    if [[ "$OSTYPE" == "darwin"* ]]; then
        sed -i '' "s|^DATABASE_URL=.*|DATABASE_URL=${db_url}|" src/.env
        sed -i '' "s|^JWT_SECRET=.*|JWT_SECRET=${jwt_secret}|" src/.env
    else
        sed -i "s|^DATABASE_URL=.*|DATABASE_URL=${db_url}|" src/.env
        sed -i "s|^JWT_SECRET=.*|JWT_SECRET=${jwt_secret}|" src/.env
    fi

    info "Environment configured in src/.env"
    info "JWT secret generated (32 bytes, base64)"
}

# --- Build ---

build_project() {
    info "Building Civiqo..."
    (cd src && cargo build -p server 2>&1)
    info "Build complete"
}

# --- Systemd / Launchd service setup ---

setup_service() {
    echo ""
    if ! confirm "Configure Civiqo as a system service (auto-start on boot)?"; then
        echo ""
        info "To start Civiqo manually:"
        echo "  cd src && cargo run -p server"
        echo "  Server will be at http://localhost:9001"
        return 0
    fi

    local project_dir
    project_dir=$(pwd)

    if [[ "$OS" == "macos" ]]; then
        setup_launchd "$project_dir"
    else
        setup_systemd "$project_dir"
    fi
}

setup_systemd() {
    local project_dir="$1"
    local service_file="/etc/systemd/system/civiqo.service"
    local bin_path
    bin_path="${project_dir}/target/debug/server"

    if [[ -f "${project_dir}/target/release/server" ]]; then
        bin_path="${project_dir}/target/release/server"
    fi

    info "Creating systemd service..."

    sudo tee "$service_file" > /dev/null <<UNIT
[Unit]
Description=Civiqo Community Manager
After=network.target postgresql.service
Requires=postgresql.service

[Service]
Type=simple
User=$(whoami)
WorkingDirectory=${project_dir}/src
ExecStart=${bin_path}
EnvironmentFile=${project_dir}/src/.env
Restart=on-failure
RestartSec=5
StandardOutput=journal
StandardError=journal
SyslogIdentifier=civiqo

[Install]
WantedBy=multi-user.target
UNIT

    sudo systemctl daemon-reload
    sudo systemctl enable civiqo
    sudo systemctl start civiqo

    echo ""
    info "Civiqo is running as a systemd service!"
    echo ""
    echo "  Useful commands:"
    echo "    sudo systemctl start civiqo     # Start"
    echo "    sudo systemctl stop civiqo      # Stop"
    echo "    sudo systemctl restart civiqo   # Restart"
    echo "    sudo systemctl status civiqo    # Status"
    echo "    journalctl -u civiqo -f         # View logs"
    echo ""
    echo "  Config file: ${project_dir}/src/.env"
    echo "  Service file: ${service_file}"
}

setup_launchd() {
    local project_dir="$1"
    local plist_path="$HOME/Library/LaunchAgents/com.civiqo.server.plist"
    local bin_path
    bin_path="${project_dir}/target/debug/server"

    if [[ -f "${project_dir}/target/release/server" ]]; then
        bin_path="${project_dir}/target/release/server"
    fi

    info "Creating launchd agent..."

    cat > "$plist_path" <<PLIST
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.civiqo.server</string>
    <key>ProgramArguments</key>
    <array>
        <string>${bin_path}</string>
    </array>
    <key>WorkingDirectory</key>
    <string>${project_dir}/src</string>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
    <key>StandardOutPath</key>
    <string>/tmp/civiqo.log</string>
    <key>StandardErrorPath</key>
    <string>/tmp/civiqo.error.log</string>
</dict>
</plist>
PLIST

    launchctl load "$plist_path"

    echo ""
    info "Civiqo is running as a launchd agent!"
    echo ""
    echo "  Useful commands:"
    echo "    launchctl start com.civiqo.server    # Start"
    echo "    launchctl stop com.civiqo.server     # Stop"
    echo "    launchctl unload ${plist_path}       # Disable"
    echo "    tail -f /tmp/civiqo.log              # View logs"
    echo ""
    echo "  Config file: ${project_dir}/src/.env"
    echo "  Plist file: ${plist_path}"
}

# --- Main ---

main() {
    print_header
    setup_repo
    detect_os

    echo ""
    info "Phase 1: Checking prerequisites..."
    echo ""
    check_rust
    check_postgresql
    check_node

    echo ""
    info "Phase 2: Database & environment..."
    echo ""
    local db_url
    db_url=$(setup_database)
    setup_env "$db_url"

    echo ""
    info "Phase 3: Building project..."
    echo ""
    build_project

    echo ""
    info "Phase 4: Service configuration..."
    setup_service

    echo ""
    echo -e "${GREEN}${BOLD}========================================${NC}"
    echo -e "${GREEN}${BOLD}  Civiqo setup complete!${NC}"
    echo -e "${GREEN}${BOLD}========================================${NC}"
    echo ""
    echo "  Open http://localhost:9001 in your browser"
    echo ""
}

main "$@"
