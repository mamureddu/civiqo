#!/bin/bash

# 🚀 Community Manager - Complete Service Deployment
# Deploys both the API Server and Lambda Authorizer to AWS

set -e

# ============================================================================
# Colors & Formatting
# ============================================================================

RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# ============================================================================
# Configuration
# ============================================================================

# Default values (can be overridden by .env files)
SERVER_FUNCTION_NAME="community-manager-api"
SERVER_MEMORY=512
SERVER_TIMEOUT=60

AUTHORIZER_FUNCTION_NAME="community-manager-authorizer"
AUTHORIZER_MEMORY=256
AUTHORIZER_TIMEOUT=30

REGION="eu-central-1"
ROLE_NAME="lambda-execution-role"

# Environment variables (loaded from .env files)
STAGE=""
ENV_FILE=""

# ============================================================================
# Functions
# ============================================================================

print_header() {
    echo -e "\n${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${CYAN}$1${NC}"
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}\n"
}

print_step() {
    echo -e "${BLUE}▶ $1${NC}"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

# ============================================================================
# Load Environment Variables
# ============================================================================

load_env_file() {
    local env_file="$1"
    
    if [ ! -f "$env_file" ]; then
        print_error "Environment file not found: $env_file"
        exit 1
    fi
    
    print_step "Loading environment from: $env_file"
    
    # Source the env file, filtering out comments and empty lines
    set -a
    source <(grep -v '^#' "$env_file" | grep -v '^$')
    set +a
    
    print_success "Environment variables loaded"
}

validate_env_variables() {
    local required_vars=(
        "DATABASE_URL"
        "AUTH0_DOMAIN"
        "AUTH0_CLIENT_ID"
        "AUTH0_CLIENT_SECRET"
        "AUTH0_CALLBACK_URL"
        "SESSION_SECRET"
        "JWT_SECRET"
        "RUST_LOG"
    )
    
    print_step "Validating required environment variables..."
    
    local missing_vars=()
    for var in "${required_vars[@]}"; do
        if [ -z "${!var}" ]; then
            missing_vars+=("$var")
        fi
    done
    
    if [ ${#missing_vars[@]} -gt 0 ]; then
        print_error "Missing required environment variables:"
        for var in "${missing_vars[@]}"; do
            echo "  - $var"
        done
        exit 1
    fi
    
    print_success "All required environment variables are set"
}

# ============================================================================
# Stage Selection
# ============================================================================

select_stage() {
    print_header "Environment Stage Selection"
    
    echo "Available stages:"
    echo "  1) dev       (Development - for testing)"
    echo "  2) staging   (Staging - pre-production)"
    echo "  3) prod      (Production - live environment)"
    echo ""
    read -p "Select stage (1-3): " STAGE_SELECTION
    
    case $STAGE_SELECTION in
        1)
            STAGE="dev"
            STAGE_SUFFIX="-dev"
            ENV_FILE=".env.dev"
            ;;
        2)
            STAGE="staging"
            STAGE_SUFFIX="-staging"
            ENV_FILE=".env.staging"
            ;;
        3)
            STAGE="prod"
            STAGE_SUFFIX=""
            ENV_FILE=".env.production"
            print_warning "⚠️  You are deploying to PRODUCTION!"
            read -p "Are you sure? (type 'yes' to confirm): " CONFIRM
            if [ "$CONFIRM" != "yes" ]; then
                print_error "Deployment cancelled"
                exit 1
            fi
            ;;
        *)
            print_error "Invalid selection"
            exit 1
            ;;
    esac
    
    # Load environment variables from the selected stage file
    load_env_file "$ENV_FILE"
    
    # Validate all required variables are present
    validate_env_variables
    
    # Update function names with stage suffix
    SERVER_FUNCTION_NAME="community-manager-api${STAGE_SUFFIX}"
    AUTHORIZER_FUNCTION_NAME="community-manager-authorizer${STAGE_SUFFIX}"
    
    print_success "Selected stage: $STAGE"
    echo "  Environment file: $ENV_FILE"
    echo "  Server function: $SERVER_FUNCTION_NAME"
    echo "  Authorizer function: $AUTHORIZER_FUNCTION_NAME"
}

# ============================================================================
# AWS Account Selection
# ============================================================================

select_aws_account() {
    print_header "AWS Account Selection"
    
    # Get list of AWS profiles
    PROFILES=$(aws configure list-profiles 2>/dev/null || echo "")
    
    if [ -z "$PROFILES" ]; then
        print_error "No AWS profiles found. Please configure AWS credentials first."
        echo ""
        echo "Run: aws configure"
        exit 1
    fi
    
    # Convert to array
    PROFILE_ARRAY=($PROFILES)
    
    echo "Available AWS profiles:"
    for i in "${!PROFILE_ARRAY[@]}"; do
        PROFILE="${PROFILE_ARRAY[$i]}"
        ACCOUNT_ID=$(aws sts get-caller-identity --profile "$PROFILE" --query Account --output text 2>/dev/null || echo "unknown")
        printf "  %d) %s (Account: %s)\n" $((i+1)) "$PROFILE" "$ACCOUNT_ID"
    done
    
    echo ""
    read -p "Select profile (1-${#PROFILE_ARRAY[@]}): " SELECTION
    
    # Validate selection
    if ! [[ "$SELECTION" =~ ^[0-9]+$ ]] || [ "$SELECTION" -lt 1 ] || [ "$SELECTION" -gt "${#PROFILE_ARRAY[@]}" ]; then
        print_error "Invalid selection"
        exit 1
    fi
    
    SELECTED_PROFILE="${PROFILE_ARRAY[$((SELECTION-1))]}"
    export AWS_PROFILE="$SELECTED_PROFILE"
    
    # Get account details
    ACCOUNT_ID=$(aws sts get-caller-identity --query Account --output text)
    ACCOUNT_ARN=$(aws sts get-caller-identity --query Arn --output text)
    
    print_success "Selected profile: $SELECTED_PROFILE"
    echo "  Account ID: $ACCOUNT_ID"
    echo "  ARN: $ACCOUNT_ARN"
}

# ============================================================================
# Pre-flight Checks
# ============================================================================

check_prerequisites() {
    print_header "Pre-flight Checks"
    
    # Check cargo-lambda
    print_step "Checking cargo-lambda..."
    if ! command -v cargo-lambda &> /dev/null; then
        print_warning "cargo-lambda not found"
        read -p "Install cargo-lambda? (y/n): " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            print_step "Installing cargo-lambda..."
            pip3 install cargo-lambda
            print_success "cargo-lambda installed"
        else
            print_error "cargo-lambda is required"
            exit 1
        fi
    else
        print_success "cargo-lambda found"
    fi
    
    # Check AWS CLI
    print_step "Checking AWS CLI..."
    if ! command -v aws &> /dev/null; then
        print_error "AWS CLI not found. Please install it first."
        exit 1
    fi
    print_success "AWS CLI found"
    
    # Check Rust
    print_step "Checking Rust..."
    if ! command -v cargo &> /dev/null; then
        print_error "Rust/Cargo not found. Please install Rust first."
        exit 1
    fi
    print_success "Rust found"
    
    # Check AWS credentials
    print_step "Checking AWS credentials..."
    if ! aws sts get-caller-identity &> /dev/null; then
        print_error "AWS credentials not configured"
        exit 1
    fi
    print_success "AWS credentials valid"
}

# ============================================================================
# Build
# ============================================================================

build_services() {
    print_header "Building Services"
    
    # Build Server
    print_step "Building API Server for ARM64..."
    if cargo lambda build --release --arm64 -p server; then
        print_success "Server build successful"
        SERVER_SIZE=$(du -h target/lambda/server/bootstrap.zip 2>/dev/null | cut -f1)
        echo "  Binary size: $SERVER_SIZE"
    else
        print_error "Server build failed"
        exit 1
    fi
    
    # Build Authorizer
    print_step "Building Authorizer for ARM64..."
    if cargo lambda build --release --arm64 -p authorizer; then
        print_success "Authorizer build successful"
        AUTH_SIZE=$(du -h target/lambda/authorizer/bootstrap.zip 2>/dev/null | cut -f1)
        echo "  Binary size: $AUTH_SIZE"
    else
        print_error "Authorizer build failed"
        exit 1
    fi
}

# ============================================================================
# Verify IAM Role
# ============================================================================

verify_iam_role() {
    print_header "Verifying IAM Role"
    
    print_step "Checking if role exists: $ROLE_NAME..."
    
    if aws iam get-role --role-name "$ROLE_NAME" &> /dev/null; then
        print_success "Role exists"
        ROLE_ARN=$(aws iam get-role --role-name "$ROLE_NAME" --query 'Role.Arn' --output text)
        echo "  ARN: $ROLE_ARN"
    else
        print_warning "Role does not exist"
        read -p "Create role '$ROLE_NAME'? (y/n): " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            print_step "Creating role..."
            
            TRUST_POLICY='{
              "Version": "2012-10-17",
              "Statement": [{
                "Effect": "Allow",
                "Principal": {"Service": "lambda.amazonaws.com"},
                "Action": "sts:AssumeRole"
              }]
            }'
            
            aws iam create-role \
                --role-name "$ROLE_NAME" \
                --assume-role-policy-document "$TRUST_POLICY"
            
            print_step "Attaching CloudWatch Logs policy..."
            aws iam attach-role-policy \
                --role-name "$ROLE_NAME" \
                --policy-arn arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole
            
            print_success "Role created and configured"
            ROLE_ARN=$(aws iam get-role --role-name "$ROLE_NAME" --query 'Role.Arn' --output text)
            
            # Wait for role to be available
            print_step "Waiting for role to be available..."
            sleep 5
        else
            print_error "Role is required for deployment"
            exit 1
        fi
    fi
}

# ============================================================================
# Deploy Services
# ============================================================================

deploy_services() {
    print_header "Deploying Services to AWS Lambda"
    
    ROLE_ARN=$(aws iam get-role --role-name "$ROLE_NAME" --query 'Role.Arn' --output text)
    
    # Deploy Server
    print_step "Deploying API Server: $SERVER_FUNCTION_NAME..."
    
    if cargo lambda deploy "$SERVER_FUNCTION_NAME" \
        --iam-role "$ROLE_ARN" \
        --memory "$SERVER_MEMORY" \
        --timeout "$SERVER_TIMEOUT" \
        --env-var RUST_LOG="$RUST_LOG" \
        --env-var DATABASE_URL="$DATABASE_URL" \
        --env-var AUTH0_DOMAIN="$AUTH0_DOMAIN" \
        --env-var AUTH0_CLIENT_ID="$AUTH0_CLIENT_ID" \
        --env-var AUTH0_CLIENT_SECRET="$AUTH0_CLIENT_SECRET" \
        --env-var AUTH0_CALLBACK_URL="$AUTH0_CALLBACK_URL" \
        --env-var SESSION_SECRET="$SESSION_SECRET"; then
        
        print_success "API Server deployed successfully"
        
        SERVER_ARN=$(aws lambda get-function --function-name "$SERVER_FUNCTION_NAME" --query 'Configuration.FunctionArn' --output text)
        echo "  Function ARN: $SERVER_ARN"
        echo "  Memory: $SERVER_MEMORY MB"
        echo "  Timeout: $SERVER_TIMEOUT seconds"
    else
        print_error "API Server deployment failed"
        exit 1
    fi
    
    # Deploy Authorizer
    print_step "Deploying Authorizer: $AUTHORIZER_FUNCTION_NAME..."
    
    if cargo lambda deploy "$AUTHORIZER_FUNCTION_NAME" \
        --iam-role "$ROLE_ARN" \
        --memory "$AUTHORIZER_MEMORY" \
        --timeout "$AUTHORIZER_TIMEOUT" \
        --env-var RUST_LOG="$RUST_LOG" \
        --env-var AUTH0_DOMAIN="$AUTH0_DOMAIN" \
        --env-var JWT_SECRET="$JWT_SECRET"; then
        
        print_success "Authorizer deployed successfully"
        
        AUTH_ARN=$(aws lambda get-function --function-name "$AUTHORIZER_FUNCTION_NAME" --query 'Configuration.FunctionArn' --output text)
        echo "  Function ARN: $AUTH_ARN"
        echo "  Memory: $AUTHORIZER_MEMORY MB"
        echo "  Timeout: $AUTHORIZER_TIMEOUT seconds"
    else
        print_error "Authorizer deployment failed"
        exit 1
    fi
}

# ============================================================================
# Post-Deployment
# ============================================================================

post_deployment() {
    print_header "Post-Deployment Steps"
    
    echo "✅ All services deployed successfully!"
    echo ""
    echo "Next steps:"
    echo ""
    echo "1. Configure API Gateway:"
    echo "   - Create REST API"
    echo "   - Add Lambda Authorizer (community-manager-authorizer)"
    echo "   - Set Identity Source: method.request.header.Authorization"
    echo "   - Set Cache TTL: 3600 seconds"
    echo "   - Attach authorizer to all routes"
    echo ""
    echo "2. Create API Gateway integration:"
    echo "   - Add Lambda integration to $SERVER_FUNCTION_NAME"
    echo "   - Configure routes (/, /api/*, etc.)"
    echo "   - Deploy API"
    echo ""
    echo "3. Update environment variables (if needed):"
    echo "   aws lambda update-function-configuration \\"
    echo "     --function-name $SERVER_FUNCTION_NAME \\"
    echo "     --environment Variables={DATABASE_URL=...,AUTH0_DOMAIN=...}"
    echo ""
    echo "4. Test the services:"
    echo "   # Test server"
    echo "   curl https://your-api.com/health"
    echo ""
    echo "   # Test with authorization"
    echo "   curl -H 'Authorization: Bearer token' https://your-api.com/api/communities"
    echo ""
    echo "5. Monitor logs:"
    echo "   aws logs tail /aws/lambda/$SERVER_FUNCTION_NAME --follow"
    echo "   aws logs tail /aws/lambda/$AUTHORIZER_FUNCTION_NAME --follow"
    echo ""
}

# ============================================================================
# Main
# ============================================================================

main() {
    print_header "🚀 Community Manager - Complete Service Deployment"
    
    # Parse arguments
    SKIP_BUILD=false
    SKIP_CHECKS=false
    
    while [[ $# -gt 0 ]]; do
        case $1 in
            --skip-build)
                SKIP_BUILD=true
                shift
                ;;
            --skip-checks)
                SKIP_CHECKS=true
                shift
                ;;
            --help)
                echo "Usage: ./deploy.sh [OPTIONS]"
                echo ""
                echo "Deploys both the API Server and Lambda Authorizer to AWS"
                echo ""
                echo "Options:"
                echo "  --skip-build     Skip build step (use existing binaries)"
                echo "  --skip-checks    Skip pre-flight checks"
                echo "  --help           Show this help message"
                echo ""
                echo "Environment Configuration:"
                echo "  The script loads environment variables from stage-specific .env files:"
                echo "  - .env.dev         (Development environment)"
                echo "  - .env.staging     (Staging environment)"
                echo "  - .env.production  (Production environment)"
                echo ""
                echo "Required Variables (in .env files):"
                echo "  DATABASE_URL           Database connection string"
                echo "  AUTH0_DOMAIN           Auth0 tenant domain"
                echo "  AUTH0_CLIENT_ID        Auth0 client ID"
                echo "  AUTH0_CLIENT_SECRET    Auth0 client secret"
                echo "  AUTH0_CALLBACK_URL     Auth0 callback URL"
                echo "  SESSION_SECRET         Session encryption secret"
                echo "  JWT_SECRET             JWT signing secret"
                echo "  RUST_LOG               Log level (info, debug, trace)"
                echo ""
                echo "Example .env.dev:"
                echo "  DATABASE_URL=postgresql://user:pass@localhost/db"
                echo "  AUTH0_DOMAIN=your-tenant.auth0.com"
                echo "  AUTH0_CLIENT_ID=your-client-id"
                echo "  AUTH0_CLIENT_SECRET=your-client-secret"
                echo "  AUTH0_CALLBACK_URL=https://localhost:3000/auth/callback"
                echo "  SESSION_SECRET=your-session-secret"
                echo "  JWT_SECRET=your-jwt-secret"
                echo "  RUST_LOG=info"
                exit 0
                ;;
            *)
                print_error "Unknown option: $1"
                exit 1
                ;;
        esac
    done
    
    # Execute steps
    if [ "$SKIP_CHECKS" = false ]; then
        select_stage
        check_prerequisites
        select_aws_account
    fi
    
    if [ "$SKIP_BUILD" = false ]; then
        build_services
    fi
    
    verify_iam_role
    deploy_services
    post_deployment
    
    echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${GREEN}✅ Complete Service Deployment Successful!${NC}"
    echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}\n"
}

# Run main function
main "$@"
