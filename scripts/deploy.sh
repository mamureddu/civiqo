#!/bin/bash

# Community Manager - Deployment Script
set -e

# Default values
ENVIRONMENT=${1:-"dev"}
SERVICE=${2:-"all"}

echo "🚀 Deploying Community Manager to $ENVIRONMENT environment..."

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

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

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Validate environment
validate_environment() {
    case $ENVIRONMENT in
        "dev"|"development")
            ENVIRONMENT="dev"
            ;;
        "staging"|"stage")
            ENVIRONMENT="staging"
            ;;
        "prod"|"production")
            ENVIRONMENT="prod"
            ;;
        *)
            print_error "Invalid environment: $ENVIRONMENT"
            print_status "Valid environments: dev, staging, prod"
            exit 1
            ;;
    esac
}

# Check prerequisites
check_prerequisites() {
    print_status "Checking deployment prerequisites..."

    local missing=false

    if ! command_exists cargo-lambda; then
        print_error "cargo-lambda not found. Run 'cargo install cargo-lambda'"
        missing=true
    fi

    if ! command_exists aws; then
        print_error "AWS CLI not found. Install AWS CLI and configure credentials"
        missing=true
    fi

    if [ "$missing" = true ]; then
        exit 1
    fi

    # Check AWS credentials
    if ! aws sts get-caller-identity >/dev/null 2>&1; then
        print_error "AWS credentials not configured. Run 'aws configure'"
        exit 1
    fi

    print_success "Prerequisites check passed"
}

# Deploy a Rust service
deploy_service() {
    local service=$1
    local env=$2

    print_status "Deploying $service to $env environment..."

    cd backend/$service

    # Check if deployment config exists
    if [ ! -f "deploy-$env.toml" ]; then
        print_error "Deployment config deploy-$env.toml not found for $service"
        cd ../..
        return 1
    fi

    # Build the service
    print_status "Building $service..."
    if ! cargo lambda build --release; then
        print_error "Failed to build $service"
        cd ../..
        return 1
    fi

    # Deploy with cargo-lambda
    print_status "Deploying $service..."
    if cargo lambda deploy --config-file deploy-$env.toml; then
        print_success "$service deployed successfully to $env"
    else
        print_error "$service deployment failed"
        cd ../..
        return 1
    fi

    cd ../..
    return 0
}

# Deploy frontend
deploy_frontend() {
    local env=$1

    print_status "Deploying frontend to $env environment..."

    cd frontend

    # Check if vercel is available
    if ! command_exists vercel; then
        print_warning "Vercel CLI not found. Installing..."
        npm install -g vercel
    fi

    # Build the frontend
    print_status "Building frontend..."
    if ! npm run build; then
        print_error "Frontend build failed"
        cd ..
        return 1
    fi

    # Deploy based on environment
    case $env in
        "dev")
            print_status "Deploying to Vercel preview..."
            vercel deploy
            ;;
        "staging"|"prod")
            print_status "Deploying to Vercel production..."
            vercel deploy --prod
            ;;
    esac

    if [ $? -eq 0 ]; then
        print_success "Frontend deployed successfully to $env"
    else
        print_error "Frontend deployment failed"
        cd ..
        return 1
    fi

    cd ..
    return 0
}

# Main deployment logic
main() {
    validate_environment
    check_prerequisites

    case $SERVICE in
        "api"|"api-gateway")
            deploy_service "api-gateway" $ENVIRONMENT
            ;;
        "chat"|"chat-service")
            deploy_service "chat-service" $ENVIRONMENT
            ;;
        "frontend"|"fe")
            deploy_frontend $ENVIRONMENT
            ;;
        "backend"|"be")
            deploy_service "api-gateway" $ENVIRONMENT
            deploy_service "chat-service" $ENVIRONMENT
            ;;
        "all")
            print_status "Deploying all services to $ENVIRONMENT..."

            # Deploy backend services
            deploy_service "api-gateway" $ENVIRONMENT
            deploy_service "chat-service" $ENVIRONMENT

            # Deploy frontend
            deploy_frontend $ENVIRONMENT

            print_success "All services deployed to $ENVIRONMENT! 🎉"
            ;;
        *)
            print_error "Invalid service: $SERVICE"
            echo ""
            print_status "Usage: $0 [ENVIRONMENT] [SERVICE]"
            echo ""
            print_status "Environments:"
            echo "  dev, development    - Development environment"
            echo "  staging, stage      - Staging environment"
            echo "  prod, production    - Production environment"
            echo ""
            print_status "Services:"
            echo "  api, api-gateway    - REST API service"
            echo "  chat, chat-service  - WebSocket chat service"
            echo "  frontend, fe        - Next.js frontend"
            echo "  backend, be         - All backend services"
            echo "  all                 - All services (default)"
            echo ""
            print_status "Examples:"
            echo "  $0 dev all          # Deploy everything to dev"
            echo "  $0 prod api         # Deploy API to production"
            echo "  $0 staging frontend # Deploy frontend to staging"
            exit 1
            ;;
    esac

    print_success "Deployment completed! 🎉"
}

# Run main function
main