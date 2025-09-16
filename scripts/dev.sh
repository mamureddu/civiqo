#!/bin/bash

# Community Manager - Development Environment Script
set -e

echo "🔧 Starting Community Manager development environment..."

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

# Check prerequisites
check_prerequisites() {
    print_status "Checking prerequisites..."

    local missing=false

    if ! command_exists cargo-lambda; then
        print_error "cargo-lambda not found. Run './scripts/setup.sh' first."
        missing=true
    fi

    if ! command_exists docker; then
        print_error "Docker not found. Please install Docker."
        missing=true
    fi

    if ! command_exists node; then
        print_error "Node.js not found. Please install Node.js."
        missing=true
    fi

    if [ "$missing" = true ]; then
        exit 1
    fi

    print_success "All prerequisites are available"
}

# Start Docker services
start_docker_services() {
    print_status "Starting Docker services..."

    if ! docker compose up -d; then
        print_error "Failed to start Docker services"
        exit 1
    fi

    print_status "Waiting for services to be ready..."

    # Wait for PostgreSQL
    local retries=30
    while ! docker compose exec postgres pg_isready -U dev -d community_manager >/dev/null 2>&1; do
        retries=$((retries-1))
        if [ $retries -eq 0 ]; then
            print_error "PostgreSQL failed to start"
            docker compose logs postgres
            exit 1
        fi
        sleep 1
    done

    # Wait for LocalStack
    local retries=30
    while ! curl -sf http://localhost:4566/_localstack/health >/dev/null 2>&1; do
        retries=$((retries-1))
        if [ $retries -eq 0 ]; then
            print_warning "LocalStack failed to start (continuing anyway)"
            break
        fi
        sleep 1
    done

    print_success "Docker services are ready"
}

# Setup LocalStack AWS services
setup_localstack() {
    print_status "Setting up LocalStack AWS services..."

    # Wait a bit more for LocalStack to be fully ready
    sleep 5

    # Create SQS queues and SNS topics
    aws --endpoint-url=http://localhost:4566 sqs create-queue --queue-name chat-queue --region us-east-1 >/dev/null 2>&1 || true
    aws --endpoint-url=http://localhost:4566 sns create-topic --name chat-notifications --region us-east-1 >/dev/null 2>&1 || true

    print_success "LocalStack services configured"
}

# Start backend services
start_backend_services() {
    print_status "Starting Rust backend services..."

    # Start API Gateway
    print_status "Starting API Gateway service..."
    cd backend
    if [ -f "api-gateway/lambda-env/dev.env" ]; then
        cargo lambda watch api-gateway --env-file api-gateway/lambda-env/dev.env --port 9001 > ../logs/api-gateway.log 2>&1 &
    else
        cargo lambda watch api-gateway --port 9001 > ../logs/api-gateway.log 2>&1 &
    fi
    API_PID=$!
    echo $API_PID > ../logs/api-gateway.pid

    # Start Chat Service
    print_status "Starting Chat WebSocket service..."
    if [ -f "chat-service/lambda-env/dev.env" ]; then
        cargo lambda watch chat-service --env-file chat-service/lambda-env/dev.env --port 9002 > ../logs/chat-service.log 2>&1 &
    else
        cargo lambda watch chat-service --port 9002 > ../logs/chat-service.log 2>&1 &
    fi
    CHAT_PID=$!
    echo $CHAT_PID > ../logs/chat-service.pid

    cd ..

    # Give services time to start
    sleep 3

    print_success "Backend services started"
}

# Start frontend
start_frontend() {
    print_status "Starting Next.js frontend..."

    cd frontend
    if [ ! -d "node_modules" ]; then
        print_status "Installing frontend dependencies..."
        npm install
    fi

    npm run dev > ../logs/frontend.log 2>&1 &
    FRONTEND_PID=$!
    echo $FRONTEND_PID > ../logs/frontend.pid

    cd ..

    print_success "Frontend started"
}

# Cleanup function
cleanup() {
    echo ""
    print_status "Stopping development environment..."

    # Kill background processes
    if [ -f "logs/api-gateway.pid" ]; then
        kill "$(cat logs/api-gateway.pid)" 2>/dev/null || true
        rm -f logs/api-gateway.pid
    fi

    if [ -f "logs/chat-service.pid" ]; then
        kill "$(cat logs/chat-service.pid)" 2>/dev/null || true
        rm -f logs/chat-service.pid
    fi

    if [ -f "logs/frontend.pid" ]; then
        kill "$(cat logs/frontend.pid)" 2>/dev/null || true
        rm -f logs/frontend.pid
    fi

    # Stop Docker services
    docker compose down

    print_success "Development environment stopped"
    exit 0
}

# Main execution
main() {
    # Create logs directory
    mkdir -p logs

    # Set up signal handlers
    trap cleanup INT TERM

    # Check prerequisites
    check_prerequisites

    # Start Docker services
    start_docker_services

    # Setup LocalStack (if available)
    if command_exists aws; then
        setup_localstack
    else
        print_warning "AWS CLI not found, skipping LocalStack setup"
    fi

    # Start backend services
    start_backend_services

    # Start frontend
    start_frontend

    # Display running services
    echo ""
    print_success "🎉 Development environment is running!"
    echo ""
    echo "Services:"
    echo "  📱 Frontend (Next.js):     http://localhost:3000"
    echo "  🔌 API Gateway (Rust):     http://localhost:9001"
    echo "  💬 Chat Service (Rust):    http://localhost:9002"
    echo "  🗄️  Database (Postgres):   localhost:5432"
    echo "  📊 Adminer (DB UI):        http://localhost:8080"
    echo "  ☁️  LocalStack (AWS):       http://localhost:4566"
    echo ""
    echo "Logs:"
    echo "  API Gateway: logs/api-gateway.log"
    echo "  Chat Service: logs/chat-service.log"
    echo "  Frontend: logs/frontend.log"
    echo ""
    print_status "Press Ctrl+C to stop all services"

    # Wait for user interrupt
    while true; do
        sleep 1
    done
}

# Run main function
main