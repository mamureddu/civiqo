#!/bin/bash

# Community Manager - Testing Script
set -e

echo "🧪 Running Community Manager tests..."

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

# Test backend
test_backend() {
    print_status "Running backend tests..."

    cd backend

    # Run cargo tests
    if cargo test --workspace --verbose; then
        print_success "Backend tests passed"
    else
        print_error "Backend tests failed"
        cd ..
        return 1
    fi

    # Run clippy for linting
    print_status "Running clippy lints..."
    if cargo clippy --workspace --all-targets --all-features -- -D warnings; then
        print_success "Clippy checks passed"
    else
        print_error "Clippy checks failed"
        cd ..
        return 1
    fi

    # Check formatting
    print_status "Checking code formatting..."
    if cargo fmt --all --check; then
        print_success "Code formatting is correct"
    else
        print_error "Code formatting issues found. Run 'cargo fmt --all'"
        cd ..
        return 1
    fi

    cd ..
    return 0
}

# Test frontend
test_frontend() {
    print_status "Running frontend tests..."

    cd frontend

    # Check if dependencies are installed
    if [ ! -d "node_modules" ]; then
        print_status "Installing frontend dependencies..."
        npm install
    fi

    # Run TypeScript type checking
    print_status "Running TypeScript type check..."
    if npm run type-check; then
        print_success "TypeScript type check passed"
    else
        print_error "TypeScript type check failed"
        cd ..
        return 1
    fi

    # Run ESLint
    print_status "Running ESLint..."
    if npm run lint; then
        print_success "ESLint checks passed"
    else
        print_error "ESLint checks failed"
        cd ..
        return 1
    fi

    # Run tests if they exist
    if npm run test --if-present >/dev/null 2>&1; then
        print_status "Running frontend tests..."
        if npm run test; then
            print_success "Frontend tests passed"
        else
            print_error "Frontend tests failed"
            cd ..
            return 1
        fi
    else
        print_warning "No frontend tests configured yet"
    fi

    cd ..
    return 0
}

# Integration tests
test_integration() {
    print_status "Running integration tests..."

    # Start Docker services for testing
    print_status "Starting test services..."
    docker compose -f docker-compose.yml up -d postgres

    # Wait for PostgreSQL
    local retries=30
    while ! docker compose exec postgres pg_isready -U dev -d community_manager >/dev/null 2>&1; do
        retries=$((retries-1))
        if [ $retries -eq 0 ]; then
            print_error "PostgreSQL failed to start for testing"
            docker compose down
            return 1
        fi
        sleep 1
    done

    # Run database migrations for testing
    cd backend
    if command -v sqlx >/dev/null 2>&1; then
        print_status "Running database migrations..."
        DATABASE_URL="postgresql://dev:dev123@localhost:5432/community_manager" sqlx migrate run --source migrations
    else
        print_warning "sqlx-cli not found, skipping migrations"
    fi
    cd ..

    # Add integration tests here when they exist
    print_warning "Integration tests not implemented yet"

    # Cleanup
    docker compose down

    return 0
}

# Build tests
test_build() {
    print_status "Testing build processes..."

    # Test backend build
    print_status "Testing backend build..."
    cd backend
    if cargo lambda build; then
        print_success "Backend build successful"
    else
        print_error "Backend build failed"
        cd ..
        return 1
    fi
    cd ..

    # Test frontend build
    print_status "Testing frontend build..."
    cd frontend
    if [ ! -d "node_modules" ]; then
        npm install
    fi

    if npm run build; then
        print_success "Frontend build successful"
    else
        print_error "Frontend build failed"
        cd ..
        return 1
    fi
    cd ..

    return 0
}

# Main function
main() {
    local test_type=${1:-"all"}
    local failed=false

    case $test_type in
        "backend"|"be")
            test_backend || failed=true
            ;;
        "frontend"|"fe")
            test_frontend || failed=true
            ;;
        "integration"|"int")
            test_integration || failed=true
            ;;
        "build")
            test_build || failed=true
            ;;
        "all")
            print_status "Running all tests..."

            test_backend || failed=true
            test_frontend || failed=true
            test_build || failed=true
            test_integration || failed=true
            ;;
        *)
            print_error "Invalid test type: $test_type"
            echo ""
            print_status "Usage: $0 [TEST_TYPE]"
            echo ""
            print_status "Test types:"
            echo "  backend, be         - Backend Rust tests"
            echo "  frontend, fe        - Frontend TypeScript tests"
            echo "  integration, int    - Integration tests"
            echo "  build              - Build tests"
            echo "  all                - All tests (default)"
            echo ""
            print_status "Examples:"
            echo "  $0 backend         # Run only backend tests"
            echo "  $0 frontend        # Run only frontend tests"
            echo "  $0 all             # Run all tests"
            exit 1
            ;;
    esac

    if [ "$failed" = true ]; then
        print_error "Some tests failed!"
        exit 1
    else
        print_success "All tests passed! 🎉"
    fi
}

# Run main function
main "$@"