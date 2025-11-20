#!/bin/bash
set -e

echo "🔨 Building Lambda Authorizer..."

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Build for AWS Lambda (ARM64)
echo -e "${BLUE}Building for ARM64...${NC}"
cargo lambda build --release --arm64

echo -e "${GREEN}✅ Build complete!${NC}"
echo ""
echo "Binary location: target/lambda/authorizer/bootstrap.zip"
echo ""
echo "Next steps:"
echo "  1. Test locally:  cargo lambda invoke authorizer --data-file test-event.json"
echo "  2. Deploy:        cargo lambda deploy authorizer"
echo "  3. Or use SAM:    sam build && sam deploy"
