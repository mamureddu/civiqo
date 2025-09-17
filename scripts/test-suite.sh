#!/bin/bash
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

SCRIPT_DIR="$(dirname "$0")"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
REPORT_FILE="$PROJECT_ROOT/test-report-$(date +%Y%m%d-%H%M%S).md"

echo "🧪 Community Manager Comprehensive Test Suite"
echo "=============================================="
echo "📅 Test Run: $(date)"
echo "📍 Project: $PROJECT_ROOT"
echo "📄 Report: $REPORT_FILE"
echo ""

# Initialize test report
cat > "$REPORT_FILE" << EOF
# Community Manager Test Suite Report

**Test Run Date:** $(date)
**Project Location:** $PROJECT_ROOT
**Test Suite Version:** 1.0

## Test Summary

EOF

# Counters for test results
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0
SKIPPED_TESTS=0

# Function to log test results
log_test() {
    local test_name="$1"
    local status="$2"
    local details="$3"

    TOTAL_TESTS=$((TOTAL_TESTS + 1))

    case $status in
        "PASS")
            PASSED_TESTS=$((PASSED_TESTS + 1))
            echo -e "✅ ${GREEN}PASS${NC} - $test_name"
            echo "- ✅ **PASS** - $test_name" >> "$REPORT_FILE"
            ;;
        "FAIL")
            FAILED_TESTS=$((FAILED_TESTS + 1))
            echo -e "❌ ${RED}FAIL${NC} - $test_name"
            echo "- ❌ **FAIL** - $test_name" >> "$REPORT_FILE"
            if [ -n "$details" ]; then
                echo "  Details: $details" >> "$REPORT_FILE"
            fi
            ;;
        "SKIP")
            SKIPPED_TESTS=$((SKIPPED_TESTS + 1))
            echo -e "⏭️  ${YELLOW}SKIP${NC} - $test_name"
            echo "- ⏭️ **SKIP** - $test_name" >> "$REPORT_FILE"
            if [ -n "$details" ]; then
                echo "  Reason: $details" >> "$REPORT_FILE"
            fi
            ;;
    esac

    if [ -n "$details" ]; then
        echo "    $details"
    fi
}

# Function to run a command and capture output
run_test_cmd() {
    local cmd="$1"
    local test_name="$2"

    echo ""
    echo -e "${BLUE}Running:${NC} $test_name"
    echo "Command: $cmd"

    if eval "$cmd" >/dev/null 2>&1; then
        log_test "$test_name" "PASS"
        return 0
    else
        local error_output=$(eval "$cmd" 2>&1 | tail -5)
        log_test "$test_name" "FAIL" "$error_output"
        return 1
    fi
}

echo ""
echo "🔍 Phase 1: Prerequisites Check"
echo "==============================="

echo "" >> "$REPORT_FILE"
echo "## Prerequisites Check" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

# Check Docker
echo ""
echo -e "${BLUE}Checking:${NC} Docker Installation"
if command -v docker >/dev/null 2>&1; then
    docker_version=$(docker --version)
    log_test "Docker Installation" "PASS" "$docker_version"
else
    log_test "Docker Installation" "FAIL" "Docker not found in PATH"
fi

# Check Docker Daemon
echo ""
echo -e "${BLUE}Checking:${NC} Docker Daemon"
if docker info >/dev/null 2>&1; then
    log_test "Docker Daemon Running" "PASS"
else
    log_test "Docker Daemon Running" "FAIL" "Docker daemon not running"
    echo "❌ Docker is required for testing. Please start Docker Desktop."
    exit 1
fi

# Check Rust/Cargo
echo ""
echo -e "${BLUE}Checking:${NC} Rust Installation"
if command -v cargo >/dev/null 2>&1; then
    rust_version=$(cargo --version)
    log_test "Rust/Cargo Installation" "PASS" "$rust_version"
else
    log_test "Rust/Cargo Installation" "FAIL" "Cargo not found in PATH"
    exit 1
fi

# Check cargo-lambda
echo ""
echo -e "${BLUE}Checking:${NC} cargo-lambda"
if command -v cargo-lambda >/dev/null 2>&1; then
    lambda_version=$(cargo lambda --version)
    log_test "cargo-lambda Installation" "PASS" "$lambda_version"
else
    log_test "cargo-lambda Installation" "FAIL" "cargo-lambda not found"
fi

echo ""
echo "🐳 Phase 2: Docker Services"
echo "==========================="

echo "" >> "$REPORT_FILE"
echo "## Docker Services" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

# Start Docker services if not running
echo ""
echo -e "${BLUE}Starting:${NC} Docker Services"
cd "$PROJECT_ROOT"

if ! docker-compose ps | grep -q "Up"; then
    echo "Starting Docker services..."
    docker-compose up -d postgres redis localstack adminer

    # Wait for services to be ready
    echo "Waiting for services to be healthy..."
    sleep 10
fi

# Test PostgreSQL
echo ""
echo -e "${BLUE}Testing:${NC} PostgreSQL Connection"
if docker-compose exec -T postgres pg_isready -U dev -d community_manager >/dev/null 2>&1; then
    log_test "PostgreSQL Connection" "PASS"

    # Test database query
    if docker-compose exec -T postgres psql -U dev -d community_manager -c "SELECT 1;" >/dev/null 2>&1; then
        log_test "PostgreSQL Query Test" "PASS"
    else
        log_test "PostgreSQL Query Test" "FAIL"
    fi
else
    log_test "PostgreSQL Connection" "FAIL"
fi

# Test Redis
echo ""
echo -e "${BLUE}Testing:${NC} Redis Connection"
if docker-compose exec -T redis redis-cli ping >/dev/null 2>&1; then
    log_test "Redis Connection" "PASS"
else
    log_test "Redis Connection" "FAIL"
fi

# Test LocalStack
echo ""
echo -e "${BLUE}Testing:${NC} LocalStack Health"
if curl -s http://localhost:4566/_localstack/health >/dev/null 2>&1; then
    log_test "LocalStack Health Check" "PASS"

    # Test S3 endpoint
    if curl -s http://localhost:4566 >/dev/null 2>&1; then
        log_test "LocalStack S3 Endpoint" "PASS"
    else
        log_test "LocalStack S3 Endpoint" "FAIL"
    fi
else
    log_test "LocalStack Health Check" "FAIL"
fi

# Test Adminer
echo ""
echo -e "${BLUE}Testing:${NC} Adminer Web Interface"
if curl -s http://localhost:8080 >/dev/null 2>&1; then
    log_test "Adminer Web Interface" "PASS"
else
    log_test "Adminer Web Interface" "FAIL"
fi

echo ""
echo "🔧 Phase 3: Backend Compilation & Tests"
echo "======================================="

echo "" >> "$REPORT_FILE"
echo "## Backend Tests" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

cd "$PROJECT_ROOT/backend"

# Test compilation
echo ""
run_test_cmd "cargo check --quiet" "Backend Compilation Check"

# Test cargo-lambda build
echo ""
run_test_cmd "cargo lambda build --quiet" "cargo-lambda Build"

# Run unit tests
echo ""
echo -e "${BLUE}Running:${NC} Unit Tests"
if cargo test --lib --quiet 2>/dev/null; then
    log_test "Unit Tests (Shared Library)" "PASS"
else
    test_output=$(cargo test --lib 2>&1 | tail -10)
    log_test "Unit Tests (Shared Library)" "FAIL" "$test_output"
fi

# Test specific modules
echo ""
echo -e "${BLUE}Running:${NC} Auth Module Tests"
if cargo test --lib shared::auth --quiet 2>/dev/null; then
    log_test "Auth Module Tests" "PASS"
else
    log_test "Auth Module Tests" "FAIL"
fi

echo ""
echo -e "${BLUE}Running:${NC} Database Module Tests"
if cargo test --lib shared::database --quiet 2>/dev/null; then
    log_test "Database Module Tests" "PASS"
else
    log_test "Database Module Tests" "FAIL"
fi

echo ""
echo -e "${BLUE}Running:${NC} Error Handling Tests"
if cargo test --lib shared::error --quiet 2>/dev/null; then
    log_test "Error Handling Tests" "PASS"
else
    log_test "Error Handling Tests" "FAIL"
fi

echo ""
echo "🌐 Phase 4: Environment & Configuration"
echo "======================================="

echo "" >> "$REPORT_FILE"
echo "## Environment Configuration" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

# Test environment files
echo ""
echo -e "${BLUE}Checking:${NC} Environment Files"
if [ -f "$PROJECT_ROOT/backend/.env" ]; then
    log_test ".env File Exists" "PASS"

    # Check required environment variables
    if grep -q "DATABASE_URL.*dev:dev123" "$PROJECT_ROOT/backend/.env"; then
        log_test "Database URL Configuration" "PASS"
    else
        log_test "Database URL Configuration" "FAIL"
    fi

    if grep -q "AUTH0_DOMAIN.*auth0.com" "$PROJECT_ROOT/backend/.env"; then
        log_test "Auth0 Configuration" "PASS"
    else
        log_test "Auth0 Configuration" "FAIL"
    fi
else
    log_test ".env File Exists" "FAIL"
fi

if [ -f "$PROJECT_ROOT/backend/.env.test" ]; then
    log_test ".env.test File Exists" "PASS"
else
    log_test ".env.test File Exists" "FAIL"
fi

echo ""
echo "📊 Phase 5: Development Scripts"
echo "==============================="

echo "" >> "$REPORT_FILE"
echo "## Development Scripts" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

cd "$PROJECT_ROOT"

# Test scripts exist and are executable
scripts=("dev-start.sh" "dev-stop.sh" "dev-status.sh" "dev-logs.sh" "dev-reset.sh")
for script in "${scripts[@]}"; do
    echo ""
    echo -e "${BLUE}Checking:${NC} $script"
    if [ -f "scripts/$script" ] && [ -x "scripts/$script" ]; then
        log_test "Script: $script" "PASS"
    else
        log_test "Script: $script" "FAIL" "Missing or not executable"
    fi
done

# Test dev-status script
echo ""
echo -e "${BLUE}Testing:${NC} Development Status Script"
if ./scripts/dev-status.sh >/dev/null 2>&1; then
    log_test "dev-status.sh Execution" "PASS"
else
    log_test "dev-status.sh Execution" "FAIL"
fi

# Generate final report
echo "" >> "$REPORT_FILE"
echo "## Test Results Summary" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"
echo "- **Total Tests:** $TOTAL_TESTS" >> "$REPORT_FILE"
echo "- **Passed:** $PASSED_TESTS" >> "$REPORT_FILE"
echo "- **Failed:** $FAILED_TESTS" >> "$REPORT_FILE"
echo "- **Skipped:** $SKIPPED_TESTS" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

if [ $FAILED_TESTS -eq 0 ]; then
    echo "- **Overall Status:** ✅ **ALL TESTS PASSED**" >> "$REPORT_FILE"
    test_status="PASSED"
else
    echo "- **Overall Status:** ❌ **$FAILED_TESTS TESTS FAILED**" >> "$REPORT_FILE"
    test_status="FAILED"
fi

echo "" >> "$REPORT_FILE"
echo "## Environment Details" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"
echo "- **Docker Version:** $(docker --version)" >> "$REPORT_FILE"
echo "- **Rust Version:** $(cargo --version)" >> "$REPORT_FILE"
if command -v cargo-lambda >/dev/null 2>&1; then
    echo "- **cargo-lambda Version:** $(cargo lambda --version)" >> "$REPORT_FILE"
fi
echo "- **Operating System:** $(uname -a)" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

# Final output
echo ""
echo "📋 Test Suite Complete!"
echo "======================="
echo -e "📊 Total Tests: $TOTAL_TESTS"
echo -e "✅ Passed: ${GREEN}$PASSED_TESTS${NC}"
echo -e "❌ Failed: ${RED}$FAILED_TESTS${NC}"
echo -e "⏭️  Skipped: ${YELLOW}$SKIPPED_TESTS${NC}"
echo ""

if [ $FAILED_TESTS -eq 0 ]; then
    echo -e "🎉 ${GREEN}ALL TESTS PASSED!${NC}"
    echo "✅ Docker development stack is fully functional"
    echo "✅ Ready for development work"
else
    echo -e "⚠️  ${RED}$FAILED_TESTS TESTS FAILED${NC}"
    echo "❌ Please review the issues above before proceeding"
fi

echo ""
echo "📄 Detailed report saved to: $REPORT_FILE"
echo ""
echo "💡 Next steps:"
echo "   • Review the test report for any issues"
echo "   • Fix any failed tests before proceeding"
echo "   • Run ./scripts/dev-start.sh to begin development"

# Exit with appropriate code
if [ $FAILED_TESTS -eq 0 ]; then
    exit 0
else
    exit 1
fi