#!/bin/bash

# Community Manager - Initial Setup Script
set -e

echo "🚀 Setting up Community Manager development environment..."

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check prerequisites
print_status "Checking prerequisites..."

if ! command_exists rust; then
    print_error "Rust is required but not installed."
    print_status "Install Rust from: https://rustup.rs/"
    exit 1
fi

if ! command_exists cargo; then
    print_error "Cargo is required but not installed."
    exit 1
fi

if ! command_exists node; then
    print_error "Node.js is required but not installed."
    print_status "Install Node.js from: https://nodejs.org/"
    exit 1
fi

if ! command_exists npm; then
    print_error "npm is required but not installed."
    exit 1
fi

if ! command_exists docker; then
    print_error "Docker is required but not installed."
    print_status "Install Docker from: https://docker.com/"
    exit 1
fi

print_success "All prerequisites are installed!"

# Install Rust tools
print_status "Installing Rust development tools..."

if ! command_exists cargo-lambda; then
    print_status "Installing cargo-lambda..."
    cargo install cargo-lambda
    print_success "cargo-lambda installed"
else
    print_status "cargo-lambda already installed"
fi

if ! command_exists sqlx; then
    print_status "Installing sqlx-cli..."
    cargo install sqlx-cli --no-default-features --features postgres
    print_success "sqlx-cli installed"
else
    print_status "sqlx-cli already installed"
fi

# Setup backend workspace
print_status "Setting up backend Rust workspace..."
cd backend

# Create Cargo.toml if it doesn't exist
if [ ! -f "Cargo.toml" ]; then
    print_status "Creating backend workspace..."
    cat > Cargo.toml << 'EOF'
[workspace]
members = [
    "api-gateway",
    "chat-service",
    "shared"
]
resolver = "2"

[workspace.dependencies]
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
uuid = { version = "1.0", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
anyhow = "1.0"
thiserror = "1.0"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
EOF
    print_success "Backend workspace created"
fi

# Try to build workspace (will show what's missing)
print_status "Checking backend workspace..."
if cargo check --workspace 2>/dev/null; then
    print_success "Backend workspace is ready"
else
    print_warning "Backend workspace needs service implementations (this is expected)"
fi

cd ..

# Setup frontend
print_status "Setting up frontend..."
cd frontend

if [ ! -f "package.json" ]; then
    print_status "Initializing Next.js project..."
    npm init -y

    # Update package.json with Next.js dependencies
    cat > package.json << 'EOF'
{
  "name": "community-manager-frontend",
  "version": "0.1.0",
  "private": true,
  "scripts": {
    "dev": "next dev",
    "build": "next build",
    "start": "next start",
    "lint": "next lint",
    "type-check": "tsc --noEmit"
  },
  "dependencies": {
    "next": "14.2.8",
    "react": "^18",
    "react-dom": "^18",
    "@mui/material": "^5.15.0",
    "@mui/icons-material": "^5.15.0",
    "@emotion/react": "^11.11.0",
    "@emotion/styled": "^11.11.0",
    "@auth0/nextjs-auth0": "^3.5.0",
    "@tanstack/react-query": "^5.0.0",
    "zustand": "^4.4.0",
    "react-leaflet": "^4.2.0",
    "leaflet": "^1.9.0"
  },
  "devDependencies": {
    "typescript": "^5",
    "@types/node": "^20",
    "@types/react": "^18",
    "@types/react-dom": "^18",
    "@types/leaflet": "^1.9.0",
    "eslint": "^8",
    "eslint-config-next": "14.2.8"
  }
}
EOF

    print_status "Installing frontend dependencies..."
    npm install
    print_success "Frontend dependencies installed"
else
    print_status "Frontend package.json exists, installing dependencies..."
    npm install
fi

cd ..

# Setup environment configuration
print_status "Setting up environment configuration..."
if [ ! -f ".env.local" ]; then
    cp .env.example .env.local
    print_warning "Created .env.local from .env.example"
    print_warning "Please update .env.local with your actual configuration values"
else
    print_status ".env.local already exists"
fi

# Create localstack directory for AWS simulation
mkdir -p localstack
print_status "Created localstack directory for AWS service simulation"

# Set up git hooks (if in git repo)
if [ -d ".git" ]; then
    print_status "Setting up git hooks..."
    # Add pre-commit hook for formatting
    mkdir -p .git/hooks
    cat > .git/hooks/pre-commit << 'EOF'
#!/bin/bash
# Format Rust code
cargo fmt --all --check
if [ $? -ne 0 ]; then
    echo "Please run 'cargo fmt --all' before committing"
    exit 1
fi

# Check Rust code
cargo clippy --all-targets --all-features -- -D warnings
if [ $? -ne 0 ]; then
    echo "Please fix clippy warnings before committing"
    exit 1
fi
EOF
    chmod +x .git/hooks/pre-commit
    print_success "Git hooks configured"
fi

print_success "Setup complete! 🎉"
echo ""
print_status "Next steps:"
echo "1. Update .env.local with your configuration values"
echo "2. Configure Auth0 application settings"
echo "3. Set up CockroachDB account (or use local PostgreSQL)"
echo "4. Run './scripts/dev.sh' to start development environment"
echo ""
print_status "Useful commands:"
echo "  ./scripts/dev.sh          - Start development environment"
echo "  ./scripts/deploy.sh dev   - Deploy to development"
echo "  docker-compose up -d      - Start local services only"
echo ""
print_warning "Remember to configure your Auth0, CockroachDB, and AWS credentials!"