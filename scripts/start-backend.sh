#!/bin/bash

# Start Backend Services
# Usage: ./scripts/start-backend.sh [service]
#   service: api | chat | all (default: all)

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Project root
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BACKEND_DIR="$PROJECT_ROOT/backend"

echo -e "${BLUE}🚀 Community Manager - Backend Startup${NC}"
echo "================================================"

# Check if .env exists
if [ ! -f "$BACKEND_DIR/.env" ]; then
    echo -e "${RED}❌ Error: backend/.env file not found${NC}"
    echo -e "${YELLOW}💡 Copy ENV_TEMPLATE.md to backend/.env and configure it${NC}"
    exit 1
fi

# Check cargo-lambda
if ! command -v cargo-lambda &> /dev/null; then
    echo -e "${RED}❌ Error: cargo-lambda not found${NC}"
    echo -e "${YELLOW}💡 Install with: cargo install cargo-lambda${NC}"
    exit 1
fi

# Determine which service to start
SERVICE="${1:-all}"

cd "$BACKEND_DIR"

case "$SERVICE" in
    api)
        echo -e "${GREEN}✅ Starting API Gateway on port 9001...${NC}"
        cargo lambda watch api-gateway --env-file .env --invoke-port 9001
        ;;
    
    chat)
        echo -e "${GREEN}✅ Starting Chat Service on port 9002...${NC}"
        cargo lambda watch chat-service --env-file .env --invoke-port 9002
        ;;
    
    all)
        echo -e "${GREEN}✅ Starting all backend services...${NC}"
        
        # Check if tmux is available
        if command -v tmux &> /dev/null; then
            echo -e "${BLUE}📺 Using tmux for multi-service management${NC}"
            
            # Kill existing session if it exists
            tmux kill-session -t community-manager-backend 2>/dev/null || true
            
            # Create new session
            tmux new-session -d -s community-manager-backend -n api-gateway
            
            # Start API Gateway in first window
            tmux send-keys -t community-manager-backend:api-gateway "cd $BACKEND_DIR && cargo lambda watch api-gateway --env-file .env --invoke-port 9001" C-m
            
            # Create new window for Chat Service
            tmux new-window -t community-manager-backend -n chat-service
            tmux send-keys -t community-manager-backend:chat-service "cd $BACKEND_DIR && cargo lambda watch chat-service --env-file .env --invoke-port 9002" C-m
            
            echo ""
            echo -e "${GREEN}✅ Backend services started in tmux session 'community-manager-backend'${NC}"
            echo ""
            echo "Tmux commands:"
            echo "  - Attach: tmux attach -t community-manager-backend"
            echo "  - Detach: Ctrl+B then D"
            echo "  - Switch window: Ctrl+B then N (next) or P (previous)"
            echo "  - Kill session: tmux kill-session -t community-manager-backend"
            echo ""
            echo -e "${YELLOW}💡 Attaching to tmux session...${NC}"
            sleep 2
            tmux attach -t community-manager-backend
        else
            echo -e "${YELLOW}⚠️  tmux not found. Starting services sequentially.${NC}"
            echo -e "${YELLOW}💡 Install tmux for better multi-service management: brew install tmux${NC}"
            echo ""
            echo -e "${BLUE}Starting API Gateway...${NC}"
            echo -e "${YELLOW}Note: Chat Service won't start until you stop API Gateway (Ctrl+C)${NC}"
            echo ""
            cargo lambda watch api-gateway --env-file .env --invoke-port 9001
        fi
        ;;
    
    *)
        echo -e "${RED}❌ Error: Unknown service '$SERVICE'${NC}"
        echo "Usage: $0 [api|chat|all]"
        exit 1
        ;;
esac
