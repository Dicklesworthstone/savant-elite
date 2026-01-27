#!/bin/bash
# Master E2E test runner for savant-elite
# Usage: ./run_all.sh [options]
#
# Options:
#   --skip-hardware    Skip hardware tests even if device is present
#   --verbose          Enable verbose logging (LOG_LEVEL=DEBUG)
#   --help             Show this help message

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
source "$SCRIPT_DIR/lib/logging.sh"

# Parse command line arguments
SKIP_HARDWARE=false
while [[ $# -gt 0 ]]; do
    case $1 in
        --skip-hardware)
            SKIP_HARDWARE=true
            shift
            ;;
        --verbose)
            export LOG_LEVEL=DEBUG
            shift
            ;;
        --help)
            echo "Usage: $0 [options]"
            echo ""
            echo "Options:"
            echo "  --skip-hardware    Skip hardware tests even if device is present"
            echo "  --verbose          Enable verbose logging"
            echo "  --help             Show this help message"
            echo ""
            echo "Environment variables:"
            echo "  SAVANT             Path to savant binary (default: searches PATH)"
            echo "  LOG_FILE           Path to log file (default: /tmp/savant-e2e-TIMESTAMP.log)"
            echo "  LOG_LEVEL          Log level: INFO or DEBUG (default: INFO)"
            echo "  NO_COLOR           Disable colored output"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Find savant binary
SAVANT="${SAVANT:-$(which savant 2>/dev/null || echo "")}"
if [[ -z "$SAVANT" ]]; then
    # Try to find in target directory (development)
    if [[ -x "$SCRIPT_DIR/../../target/release/savant" ]]; then
        SAVANT="$SCRIPT_DIR/../../target/release/savant"
    elif [[ -x "$SCRIPT_DIR/../../target/debug/savant" ]]; then
        SAVANT="$SCRIPT_DIR/../../target/debug/savant"
    else
        log_fail "Could not find savant binary. Set SAVANT environment variable or add to PATH."
        exit 1
    fi
fi
export SAVANT

suite_start "Savant Elite E2E Tests"

log_info "Savant binary: $SAVANT"
log_info "Savant version: $($SAVANT --version 2>&1 || echo 'unknown')"
log_info ""

# Test counters
TESTS_PASSED=0
TESTS_FAILED=0

# Run a test script and track results
run_test() {
    local test_script="$1"
    local test_path="$SCRIPT_DIR/$test_script"

    if [[ ! -f "$test_path" ]]; then
        log_warn "Test script not found: $test_script (skipping)"
        return 0
    fi

    log_info "────────────────────────────────────────────────────────────────────"
    log_info "Running: $test_script"
    log_info "────────────────────────────────────────────────────────────────────"

    if bash "$test_path"; then
        ((TESTS_PASSED++))
        log_pass "SUITE: $test_script completed successfully"
    else
        ((TESTS_FAILED++))
        log_fail "SUITE: $test_script failed"
    fi

    log_info ""
}

# Run all test scripts
run_test "test_help_version.sh"
run_test "test_keys.sh"
run_test "test_presets.sh"
run_test "test_config.sh"
run_test "test_doctor.sh"
run_test "test_verbose.sh"
run_test "test_completions.sh"

# Hardware tests (optional)
if [[ "$SKIP_HARDWARE" != "true" ]]; then
    if [[ -f "$SCRIPT_DIR/test_hardware.sh" ]]; then
        run_test "test_hardware.sh"
    fi
else
    log_info "Skipping hardware tests (--skip-hardware)"
fi

# Print summary and exit with appropriate code
suite_end "$TESTS_PASSED" "$TESTS_FAILED"
