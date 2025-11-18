#!/bin/bash

# Start All Services (Backend + Frontend)
# Usage: ./scripts/start-all.sh

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Project root
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

echo -e "${CYAN}"
echo "╔════════════════════════════════════════════════════════════╗"
echo "║                                                            ║"
echo "║        🚀 Community Manager - Full Stack Startup 🚀       ║"
echo "║                                                            ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo -e "${NC}"

# Check prerequisites
echo -e "${BLUE}🔍 Checking prerequisites...${NC}"

# Check Rust
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}❌ Error: Rust/Cargo not found${NC}"
    echo -e "${YELLOW}💡 Install from: https://rustup.rs/${NC}"
    exit 1
fi
echo -e "${GREEN}✅ Rust/Cargo found${NC}"

# Check cargo-lambda
if ! command -v cargo-lambda &> /dev/null; then
    echo -e "${RED}❌ Error: cargo-lambda not found${NC}"
    echo -e "${YELLOW}💡 Install with: cargo install cargo-lambda${NC}"
    exit 1
fi
echo -e "${GREEN}✅ cargo-lambda found${NC}"

# Check Node.js
if ! command -v node &> /dev/null; then
    echo -e "${RED}❌ Error: Node.js not found${NC}"
    echo -e "${YELLOW}💡 Install from: https://nodejs.org/${NC}"
    exit 1
fi
echo -e "${GREEN}✅ Node.js found${NC}"

# Check environment files
if [ ! -f "$PROJECT_ROOT/backend/.env" ]; then
    echo -e "${RED}❌ Error: backend/.env not found${NC}"
    echo -e "${YELLOW}💡 Copy ENV_TEMPLATE.md to backend/.env and configure it${NC}"
    exit 1
fi
echo -e "${GREEN}✅ backend/.env found${NC}"

if [ ! -f "$PROJECT_ROOT/frontend/.env.local" ]; then
    echo -e "${YELLOW}⚠️  Warning: frontend/.env.local not found${NC}"
    echo -e "${YELLOW}💡 Copy ENV_TEMPLATE.md to frontend/.env.local and configure it${NC}"
fi

echo ""
echo -e "${GREEN}✅ All prerequisites met!${NC}"
echo ""

# Check if tmux is available
if command -v tmux &> /dev/null; then
    echo -e "${BLUE}📺 Using tmux for multi-service management${NC}"
    echo ""
    
    # Kill existing session if it exists
    tmux kill-session -t community-manager 2>/dev/null || true
    
    # Create new session with API Gateway
    tmux new-session -d -s community-manager -n api-gateway
    tmux send-keys -t community-manager:api-gateway "cd $PROJECT_ROOT/backend && cargo lambda watch api-gateway --env-file .env --invoke-port 9001" C-m
    
    # Create window for Chat Service
    tmux new-window -t community-manager -n chat-service
    tmux send-keys -t community-manager:chat-service "cd $PROJECT_ROOT/backend && cargo lambda watch chat-service --env-file .env --invoke-port 9002" C-m
    
    # Create window for Frontend
    tmux new-window -t community-manager -n frontend
    tmux send-keys -t community-manager:frontend "cd $PROJECT_ROOT/frontend && npm run dev" C-m
    
    # Select first window
    tmux select-window -t community-manager:api-gateway
    
    echo -e "${GREEN}✅ All services started in tmux session 'community-manager'${NC}"
    echo ""
    echo "Services:"
    echo -e "  ${CYAN}API Gateway:${NC}   http://localhost:9001"
    echo -e "  ${CYAN}Chat Service:${NC}  ws://localhost:9002"
    echo -e "  ${CYAN}Frontend:${NC}      http://localhost:3000"
    echo ""
    echo "Tmux commands:"
    echo "  - Switch window: Ctrl+B then 0/1/2 (or N for next, P for previous)"
    echo "  - Detach: Ctrl+B then D"
    echo "  - Reattach: tmux attach -t community-manager"
    echo "  - Kill all: tmux kill-session -t community-manager"
    echo ""
    echo -e "${YELLOW}💡 Attaching to tmux session in 3 seconds...${NC}"
    sleep 3
    tmux attach -t community-manager
    
else
    echo -e "${YELLOW}⚠️  tmux not found${NC}"
    echo -e "${YELLOW}💡 Install tmux for better multi-service management:${NC}"
    echo -e "${YELLOW}   macOS: brew install tmux${NC}"
    echo -e "${YELLOW}   Linux: sudo apt-get install tmux${NC}"
    echo ""
    echo -e "${BLUE}Starting services individually...${NC}"
    echo ""
    echo -e "${YELLOW}Note: You'll need to run each service in a separate terminal:${NC}"
    echo ""
    echo "Terminal 1 - API Gateway:"
    echo "  cd $PROJECT_ROOT && ./scripts/start-backend.sh api"
    echo ""
    echo "Terminal 2 - Chat Service:"
    echo "  cd $PROJECT_ROOT && ./scripts/start-backend.sh chat"
    echo ""
    echo "Terminal 3 - Frontend:"
    echo "  cd $PROJECT_ROOT && ./scripts/start-frontend.sh"
    echo ""
    exit 1
fi
