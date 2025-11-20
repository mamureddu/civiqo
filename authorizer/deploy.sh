#!/bin/bash
set -e

echo "🚀 Deploying Lambda Authorizer..."

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if cargo-lambda is installed
if ! command -v cargo-lambda &> /dev/null; then
    echo -e "${YELLOW}cargo-lambda not found. Installing...${NC}"
    pip3 install cargo-lambda
fi

# Build
echo -e "${BLUE}Building...${NC}"
cargo lambda build --release --arm64

# Deploy
echo -e "${BLUE}Deploying to AWS...${NC}"
cargo lambda deploy authorizer \
    --iam-role arn:aws:iam::YOUR_ACCOUNT_ID:role/lambda-execution-role \
    --env-var RUST_LOG=info

echo -e "${GREEN}✅ Deployment complete!${NC}"
echo ""
echo "Next steps:"
echo "  1. Configure API Gateway to use this authorizer"
echo "  2. Set cache TTL to 3600 seconds (1 hour)"
echo "  3. Test with: curl -H 'Authorization: Bearer token' https://your-api.com/endpoint"
