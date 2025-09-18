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

# Check for port conflicts
check_port_conflicts() {
    print_status "Checking for port conflicts..."

    # Check if PostgreSQL port 5433 is already in use
    if lsof -Pi :5433 -sTCP:LISTEN -t >/dev/null 2>&1; then
        print_warning "Port 5433 is already in use!"
        print_status "Processes using port 5433:"
        lsof -Pi :5433 -sTCP:LISTEN
        echo ""
        print_warning "This might cause database connection issues."
        print_status "Consider stopping other services using this port."
        echo ""
    fi

    # Check if system PostgreSQL is running on 5432 (informational)
    if lsof -Pi :5432 -sTCP:LISTEN -t >/dev/null 2>&1; then
        print_status "System PostgreSQL detected on port 5432 (this is fine - we use 5433)"
        local postgres_processes=$(lsof -Pi :5432 -sTCP:LISTEN 2>/dev/null | grep -v "COMMAND")
        if [ -n "$postgres_processes" ]; then
            echo "$postgres_processes" | head -2
        fi
        echo ""
    fi
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
    print_status "Starting Docker services (PostgreSQL 15 with PostGIS 3.4 extension)..."

    if ! docker compose up -d; then
        print_warning "Docker services startup had issues, checking individual services..."
        docker compose ps
    fi

    print_status "Waiting for services to be ready..."

    # Wait for PostgreSQL to be ready (Docker does all the setup automatically)
    print_status "Waiting for PostgreSQL initialization to complete..."
    local retries=60

    # Use the health check that matches docker-compose.yml
    while ! docker compose exec postgres pg_isready -U dev -d community_manager >/dev/null 2>&1; do
        retries=$((retries-1))
        if [ $retries -eq 0 ]; then
            print_error "PostgreSQL failed to initialize properly"
            docker compose logs postgres | tail -15
            exit 1
        fi
        sleep 1
    done

    # Additional wait to ensure database is fully ready for connections
    print_status "Ensuring database is ready for external connections..."
    sleep 5

    # Test direct connection from host
    local db_retries=30
    while ! PGPASSWORD=dev123 psql -h localhost -p 5433 -U dev -d community_manager -c "SELECT 1;" >/dev/null 2>&1; do
        db_retries=$((db_retries-1))
        if [ $db_retries -eq 0 ]; then
            print_error "Cannot connect to PostgreSQL from host on port 5433"
            print_status "Container status:"
            docker compose ps postgres
            print_status "Container logs:"
            docker compose logs postgres | tail -10
            exit 1
        fi
        print_status "Waiting for database external connectivity... ($db_retries retries left)"
        sleep 2
    done

    print_success "PostgreSQL is ready for external connections"

    # Verify both databases are accessible (they should be created by init scripts)
    print_status "Verifying database setup..."

    if docker compose exec postgres psql -U dev -d community_manager -c "SELECT current_user;" >/dev/null 2>&1; then
        print_success "Main database accessible"

        if docker compose exec postgres psql -U dev -d community_manager_test -c "SELECT current_user;" >/dev/null 2>&1; then
            print_success "Test database accessible"
            print_success "PostgreSQL setup complete"
        else
            print_warning "Test database not accessible - check init-test-db.sql"
        fi
    else
        print_error "Main database not accessible"
        docker compose logs postgres | tail -15
        exit 1
    fi

    # Wait for LocalStack
    print_status "Checking LocalStack availability..."
    local retries=15
    while ! curl -sf http://localhost:4566/_localstack/health >/dev/null 2>&1; do
        retries=$((retries-1))
        if [ $retries -eq 0 ]; then
            print_warning "LocalStack failed to start (continuing anyway)"
            break
        fi
        sleep 1
    done

    print_success "Docker services check completed"
}

# Install SQLx CLI if needed
install_sqlx_cli() {
    if ! command_exists sqlx; then
        print_status "Installing SQLx CLI for database migrations..."
        cargo install sqlx-cli --no-default-features --features postgres >/dev/null 2>&1 &
        SQLX_PID=$!

        # Show progress indicator while installing
        local spinner="/-\|"
        local i=0
        while kill -0 $SQLX_PID 2>/dev/null; do
            printf "\r${BLUE}[INFO]${NC} Installing SQLx CLI... %c" "${spinner:$((i % ${#spinner})):1}"
            sleep 0.5
            ((i++))
        done
        printf "\r${GREEN}[SUCCESS]${NC} SQLx CLI installed successfully\n"
    fi
}

# Setup database migrations
setup_database() {
    print_status "Setting up database schema..."

    if [ ! -d "backend" ]; then
        print_warning "Backend directory not found, skipping database setup"
        return 0
    fi

    cd backend

    # Set up environment variables for database connections
    export DATABASE_URL="postgresql://dev:dev123@localhost:5433/community_manager"
    export TEST_DATABASE_URL="postgresql://dev:dev123@localhost:5433/community_manager_test"
    export SQLX_CONNECT_TIMEOUT=30

    print_status "Database ready for migrations and SQLx operations"
    print_status "Using DATABASE_URL: $DATABASE_URL"

    # Test SQLx connectivity before proceeding
    if command_exists sqlx; then
        print_status "Testing SQLx database connectivity..."

        if sqlx database create 2>/dev/null; then
            print_success "SQLx database initialized"
        else
            print_status "Database already exists"
        fi

        # Verify SQLx can connect and query
        if sqlx migrate info >/dev/null 2>&1; then
            print_success "SQLx connection verified"
        else
            print_warning "SQLx connection issues detected, but continuing..."
            print_status "SQLx will attempt to connect during migration/prepare steps"
        fi
    fi

    # Check if migrations have been run
    local tables_exist=false
    if docker compose exec postgres psql -U dev -d community_manager -t -c "SELECT COUNT(*) FROM information_schema.tables WHERE table_schema = 'public';" 2>/dev/null | grep -q "[1-9]"; then
        tables_exist=true
    fi

    if [ "$tables_exist" = false ]; then
        print_status "Running database migrations..."

        # Try to run migrations with sqlx
        if command_exists sqlx; then
            print_status "Running database migrations with SQLx..."

            if sqlx migrate run; then
                print_success "Database migrations completed with SQLx"
            else
                print_warning "SQLx migrations failed, falling back to direct SQL execution..."
                run_migrations_directly
            fi
        else
            print_warning "SQLx CLI not available, running migrations directly..."
            run_migrations_directly
        fi
    else
        print_success "Database schema already exists"
    fi

    # Create .env file for development with proper database connection
    print_status "Creating development environment file..."
    cat > .env << EOF
# Database Configuration (using port 5433 to avoid conflict with system PostgreSQL)
DATABASE_URL=postgresql://dev:dev123@localhost:5433/community_manager?sslmode=prefer&connect_timeout=30
TEST_DATABASE_URL=postgresql://dev:dev123@localhost:5433/community_manager_test?sslmode=prefer&connect_timeout=30

# SQLx Configuration
SQLX_OFFLINE=false

# Auth0 Configuration
AUTH0_DOMAIN=community-manager-dev.eu.auth0.com
AUTH0_AUDIENCE=community-manager-dev
AUTH0_CLIENT_ID=LySggaHFqRlFnQR5i8EPShPEM42coLZm
AUTH0_CLIENT_SECRET=9AqELvuSzgzDwwPkyIF37yIDguouWWqSJ8h5dwSbfn69xnpYcmpNFVJv_bw82TOs

# AWS Configuration
AWS_ENDPOINT_URL=http://localhost:4566
S3_BUCKET=community-manager-uploads
SQS_QUEUE_URL=http://localhost:4566/000000000000/chat-queue
EOF
    print_success "Development environment configured"

    # Prepare SQLx query cache for offline compilation (only if SQLx is available)
    if command_exists sqlx; then
        print_status "Preparing SQLx query cache for compilation..."

        # Test connection first
        if sqlx migrate info >/dev/null 2>&1; then
            print_status "SQLx connection verified, preparing query cache..."
            if cargo sqlx prepare --workspace 2>/dev/null; then
                print_success "SQLx query cache prepared successfully"
            else
                print_warning "SQLx query cache preparation failed, using online mode"
            fi
        else
            print_warning "SQLx connection issues, skipping query cache preparation"
            print_status "SQLx will use online mode during compilation"
        fi
    fi

    cd ..
}

# Run migrations directly using psql
run_migrations_directly() {
    print_status "Applying migrations directly to database..."

    # Apply migrations in order
    local migrations=(
        "001_initial.sql"
        "002_business.sql"
        "003_governance.sql"
        "004_chat.sql"
        "005_seed_data.sql"
    )

    for migration_name in "${migrations[@]}"; do
        if [ -f "migrations/$migration_name" ]; then
            print_status "Running migration: $migration_name"

            # Run migration using the mounted /migrations directory in the container
            if docker compose exec postgres psql -U dev -d community_manager -f "/migrations/$migration_name" >/dev/null 2>&1; then
                print_success "Applied migration: $migration_name"
            else
                print_warning "Migration $migration_name had issues (might already be applied)"
            fi
        else
            print_warning "Migration file not found: $migration_name"
        fi
    done

    print_success "Direct migration process completed"
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

    if [ ! -d "backend" ]; then
        print_warning "Backend directory not found, skipping backend startup"
        return 0
    fi

    cd backend

    # Start API Gateway
    print_status "Starting API Gateway service..."
    if [ -d "api-gateway" ]; then
        if [ -f "api-gateway/lambda-env/dev.env" ]; then
            cargo lambda watch api-gateway --env-file api-gateway/lambda-env/dev.env --port 9001 > ../logs/api-gateway.log 2>&1 &
        else
            cargo lambda watch api-gateway --port 9001 > ../logs/api-gateway.log 2>&1 &
        fi
        API_PID=$!
        echo $API_PID > ../logs/api-gateway.pid
        print_success "API Gateway service started (PID: $API_PID)"
    else
        print_warning "API Gateway directory not found, skipping"
    fi

    # Start Chat Service
    print_status "Starting Chat WebSocket service..."
    if [ -d "chat-service" ]; then
        if [ -f "chat-service/lambda-env/dev.env" ]; then
            cargo lambda watch chat-service --env-file chat-service/lambda-env/dev.env --port 9002 > ../logs/chat-service.log 2>&1 &
        else
            cargo lambda watch chat-service --port 9002 > ../logs/chat-service.log 2>&1 &
        fi
        CHAT_PID=$!
        echo $CHAT_PID > ../logs/chat-service.pid
        print_success "Chat WebSocket service started (PID: $CHAT_PID)"
    else
        print_warning "Chat service directory not found, skipping"
    fi

    cd ..

    # Give services time to start
    sleep 3
}

# Start frontend
start_frontend() {
    print_status "Starting Next.js frontend..."

    if [ ! -d "frontend" ]; then
        print_warning "Frontend directory not found, skipping frontend startup"
        return 0
    fi

    cd frontend
    if [ ! -d "node_modules" ]; then
        print_status "Installing frontend dependencies..."
        if ! npm install; then
            print_warning "Frontend dependency installation failed, skipping frontend startup"
            cd ..
            return 0
        fi
    fi

    npm run dev > ../logs/frontend.log 2>&1 &
    FRONTEND_PID=$!
    if [ $? -eq 0 ]; then
        echo $FRONTEND_PID > ../logs/frontend.pid
        print_success "Frontend started"
    else
        print_warning "Frontend startup failed, continuing without frontend"
    fi

    cd ..
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

    # Check for port conflicts before starting services
    check_port_conflicts

    # Start Docker services
    start_docker_services

    # Install SQLx CLI if needed (run in background to not block)
    install_sqlx_cli &
    SQLX_INSTALL_PID=$!

    # Setup database migrations
    setup_database

    # Setup LocalStack (if available)
    if command_exists aws; then
        setup_localstack
    else
        print_warning "AWS CLI not found, skipping LocalStack setup"
    fi

    # Wait for SQLx CLI installation to complete if it was running
    if [ -n "$SQLX_INSTALL_PID" ]; then
        wait $SQLX_INSTALL_PID 2>/dev/null || true
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
    echo "  🗄️  Database (PostgreSQL): localhost:5433 (PostGIS 3.4 enabled)"
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