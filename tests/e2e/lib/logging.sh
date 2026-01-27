#!/bin/bash
# Structured logging library for E2E tests
# Source this file in test scripts: source "$SCRIPT_DIR/lib/logging.sh"

# Configuration
LOG_LEVEL=${LOG_LEVEL:-INFO}
LOG_FILE=${LOG_FILE:-/tmp/savant-e2e-$(date +%Y%m%d-%H%M%S).log}

# Colors (disabled if not a terminal or NO_COLOR is set)
if [[ -t 1 ]] && [[ -z "${NO_COLOR:-}" ]]; then
    RED='\033[0;31m'
    GREEN='\033[0;32m'
    YELLOW='\033[1;33m'
    BLUE='\033[0;34m'
    CYAN='\033[0;36m'
    BOLD='\033[1m'
    DIM='\033[2m'
    NC='\033[0m'
else
    RED=''
    GREEN=''
    YELLOW=''
    BLUE=''
    CYAN=''
    BOLD=''
    DIM=''
    NC=''
fi

# Ensure log directory exists
mkdir -p "$(dirname "$LOG_FILE")" 2>/dev/null || true

# Internal: write to both console and log file
_log() {
    local level="$1"
    local color="$2"
    shift 2
    local timestamp
    timestamp="$(date '+%Y-%m-%d %H:%M:%S')"
    local msg="[$timestamp] [$level] $*"

    # Console output with color
    echo -e "${color}${msg}${NC}"

    # File output without color codes
    echo "[$timestamp] [$level] $*" >> "$LOG_FILE"
}

log_info() {
    _log "INFO" "$BLUE" "$@"
}

log_pass() {
    _log "PASS" "$GREEN" "$@"
}

log_fail() {
    _log "FAIL" "$RED" "$@"
}

log_warn() {
    _log "WARN" "$YELLOW" "$@"
}

log_debug() {
    if [[ "$LOG_LEVEL" == "DEBUG" ]]; then
        _log "DEBUG" "$DIM" "$@"
    fi
}

# Run a command with logging
# Usage: log_cmd command arg1 arg2 ...
# Returns the command's exit code
log_cmd() {
    log_debug "Running: $*"
    local output
    local exit_code

    # Capture both stdout and stderr
    output=$("$@" 2>&1)
    exit_code=$?

    # Log output to file
    if [[ -n "$output" ]]; then
        echo "$output" >> "$LOG_FILE"
    fi

    # Log exit code
    if [[ $exit_code -eq 0 ]]; then
        log_debug "Exit code: 0"
    else
        log_debug "Exit code: $exit_code"
    fi

    # Return output to caller
    echo "$output"
    return $exit_code
}

# Start a test case
# Usage: test_start "Test description"
test_start() {
    echo "" >> "$LOG_FILE"
    _log "TEST" "$CYAN" "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    _log "TEST" "${BOLD}${CYAN}" "$*"
    _log "TEST" "$CYAN" "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
}

# End a test case
# Usage: test_end pass "Description" or test_end fail "Description"
test_end() {
    local result="$1"
    local description="$2"

    if [[ "$result" == "pass" ]]; then
        log_pass "TEST PASSED: $description"
    else
        log_fail "TEST FAILED: $description"
    fi
}

# Assert that a condition is true
# Usage: assert "condition description" command args...
# Returns 0 on success, 1 on failure
assert() {
    local description="$1"
    shift

    if "$@"; then
        log_pass "ASSERT: $description"
        return 0
    else
        log_fail "ASSERT: $description"
        return 1
    fi
}

# Assert that output contains a string
# Usage: assert_contains "$output" "expected substring" "description"
assert_contains() {
    local output="$1"
    local expected="$2"
    local description="${3:-Output contains '$expected'}"

    if echo "$output" | grep -q "$expected"; then
        log_pass "ASSERT: $description"
        return 0
    else
        log_fail "ASSERT: $description"
        log_fail "  Expected to contain: $expected"
        log_fail "  Actual output: $output"
        return 1
    fi
}

# Assert that a command succeeds
# Usage: assert_success "description" command args...
assert_success() {
    local description="$1"
    shift

    if "$@" >/dev/null 2>&1; then
        log_pass "ASSERT: $description"
        return 0
    else
        log_fail "ASSERT: $description (exit code: $?)"
        return 1
    fi
}

# Assert that a command fails
# Usage: assert_failure "description" command args...
assert_failure() {
    local description="$1"
    shift

    if "$@" >/dev/null 2>&1; then
        log_fail "ASSERT: $description (expected failure, got success)"
        return 1
    else
        log_pass "ASSERT: $description"
        return 0
    fi
}

# Print test suite header
suite_start() {
    local suite_name="$1"
    log_info ""
    log_info "╔══════════════════════════════════════════════════════════════════╗"
    log_info "║  TEST SUITE: $suite_name"
    log_info "╚══════════════════════════════════════════════════════════════════╝"
    log_info ""
    log_info "Log file: $LOG_FILE"
    log_info "Started at: $(date)"
    log_info ""
}

# Print test suite summary
suite_end() {
    local passed="$1"
    local failed="$2"
    local total=$((passed + failed))

    log_info ""
    log_info "╔══════════════════════════════════════════════════════════════════╗"
    log_info "║  TEST SUMMARY"
    log_info "╚══════════════════════════════════════════════════════════════════╝"
    log_info ""
    log_info "Total tests: $total"
    log_pass "Passed: $passed"

    if [[ $failed -gt 0 ]]; then
        log_fail "Failed: $failed"
        log_info ""
        log_info "Log file: $LOG_FILE"
        return 1
    else
        log_pass "All tests passed!"
        log_info ""
        log_info "Log file: $LOG_FILE"
        return 0
    fi
}

# Initialize logging
log_debug "Logging initialized"
log_debug "Log file: $LOG_FILE"
log_debug "Log level: $LOG_LEVEL"
