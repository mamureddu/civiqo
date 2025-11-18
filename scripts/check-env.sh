#!/bin/bash

# Check Environment Configuration
# Usage: ./scripts/check-env.sh

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Project root
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

echo -e "${BLUE}🔍 Community Manager - Environment Check${NC}"
echo "================================================"
echo ""

ERRORS=0
WARNINGS=0

# Check backend/.env
echo -e "${BLUE}Checking backend/.env...${NC}"
if [ ! -f "$PROJECT_ROOT/backend/.env" ]; then
    echo -e "${RED}❌ backend/.env not found${NC}"
    echo -e "${YELLOW}💡 Copy ENV_TEMPLATE.md to backend/.env${NC}"
    ERRORS=$((ERRORS + 1))
else
    echo -e "${GREEN}✅ backend/.env exists${NC}"
    
    # Check required variables
    REQUIRED_VARS=("DATABASE_URL" "AUTH0_DOMAIN" "AUTH0_AUDIENCE" "AUTH0_CLIENT_ID" "AUTH0_CLIENT_SECRET")
    
    for VAR in "${REQUIRED_VARS[@]}"; do
        if grep -q "^${VAR}=" "$PROJECT_ROOT/backend/.env"; then
            VALUE=$(grep "^${VAR}=" "$PROJECT_ROOT/backend/.env" | cut -d'=' -f2-)
            if [[ "$VALUE" == *"your-"* ]] || [[ "$VALUE" == "" ]]; then
                echo -e "${YELLOW}⚠️  $VAR needs configuration${NC}"
                WARNINGS=$((WARNINGS + 1))
            else
                echo -e "${GREEN}  ✓ $VAR configured${NC}"
            fi
        else
            echo -e "${RED}  ✗ $VAR missing${NC}"
            ERRORS=$((ERRORS + 1))
        fi
    done
fi
echo ""

# Check frontend/.env.local
echo -e "${BLUE}Checking frontend/.env.local...${NC}"
if [ ! -f "$PROJECT_ROOT/frontend/.env.local" ]; then
    echo -e "${YELLOW}⚠️  frontend/.env.local not found${NC}"
    echo -e "${YELLOW}💡 Copy ENV_TEMPLATE.md to frontend/.env.local${NC}"
    WARNINGS=$((WARNINGS + 1))
else
    echo -e "${GREEN}✅ frontend/.env.local exists${NC}"
    
    # Check required variables
    REQUIRED_VARS=("NEXTAUTH_URL" "NEXTAUTH_SECRET" "AUTH0_CLIENT_ID" "AUTH0_CLIENT_SECRET" "AUTH0_DOMAIN" "NEXT_PUBLIC_API_URL" "NEXT_PUBLIC_WS_URL")
    
    for VAR in "${REQUIRED_VARS[@]}"; do
        if grep -q "^${VAR}=" "$PROJECT_ROOT/frontend/.env.local"; then
            VALUE=$(grep "^${VAR}=" "$PROJECT_ROOT/frontend/.env.local" | cut -d'=' -f2-)
            if [[ "$VALUE" == *"your-"* ]] || [[ "$VALUE" == "" ]]; then
                echo -e "${YELLOW}⚠️  $VAR needs configuration${NC}"
                WARNINGS=$((WARNINGS + 1))
            else
                echo -e "${GREEN}  ✓ $VAR configured${NC}"
            fi
        else
            echo -e "${RED}  ✗ $VAR missing${NC}"
            ERRORS=$((ERRORS + 1))
        fi
    done
fi
echo ""

# Check backend/.env.test
echo -e "${BLUE}Checking backend/.env.test...${NC}"
if [ ! -f "$PROJECT_ROOT/backend/.env.test" ]; then
    echo -e "${YELLOW}⚠️  backend/.env.test not found${NC}"
    echo -e "${YELLOW}💡 Copy ENV_TEMPLATE.md to backend/.env.test for testing${NC}"
    WARNINGS=$((WARNINGS + 1))
else
    echo -e "${GREEN}✅ backend/.env.test exists${NC}"
fi
echo ""

# Summary
echo "================================================"
if [ $ERRORS -eq 0 ] && [ $WARNINGS -eq 0 ]; then
    echo -e "${GREEN}✅ All environment files are properly configured!${NC}"
    echo ""
    echo "You can now start development with:"
    echo "  ./scripts/start-all.sh"
    exit 0
elif [ $ERRORS -eq 0 ]; then
    echo -e "${YELLOW}⚠️  Environment check completed with $WARNINGS warning(s)${NC}"
    echo ""
    echo "You can start development, but some features may not work:"
    echo "  ./scripts/start-all.sh"
    exit 0
else
    echo -e "${RED}❌ Environment check failed with $ERRORS error(s) and $WARNINGS warning(s)${NC}"
    echo ""
    echo "Please fix the errors above before starting development."
    echo "See ENV_TEMPLATE.md for configuration examples."
    exit 1
fi
