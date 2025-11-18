#!/bin/bash

# Start Frontend Development Server
# Usage: ./scripts/start-frontend.sh

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Project root
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
FRONTEND_DIR="$PROJECT_ROOT/frontend"

echo -e "${BLUE}🚀 Community Manager - Frontend Startup${NC}"
echo "================================================"

# Check if frontend directory exists
if [ ! -d "$FRONTEND_DIR" ]; then
    echo -e "${RED}❌ Error: frontend/ directory not found${NC}"
    exit 1
fi

# Check if .env.local exists
if [ ! -f "$FRONTEND_DIR/.env.local" ]; then
    echo -e "${YELLOW}⚠️  Warning: frontend/.env.local not found${NC}"
    echo -e "${YELLOW}💡 Copy ENV_TEMPLATE.md to frontend/.env.local and configure it${NC}"
    echo ""
fi

# Check if node_modules exists
if [ ! -d "$FRONTEND_DIR/node_modules" ]; then
    echo -e "${YELLOW}📦 Installing frontend dependencies...${NC}"
    cd "$FRONTEND_DIR"
    npm install
    echo -e "${GREEN}✅ Dependencies installed${NC}"
    echo ""
fi

cd "$FRONTEND_DIR"

echo -e "${GREEN}✅ Starting Next.js development server on port 3000...${NC}"
echo ""
echo "Frontend will be available at: http://localhost:3000"
echo ""

npm run dev
