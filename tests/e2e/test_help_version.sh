#!/bin/bash
# E2E tests for help and version commands
# Tests all --help flags and version output

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
source "$SCRIPT_DIR/lib/logging.sh"

# Find savant binary (prefer local builds for testing new features)
if [[ -n "${SAVANT:-}" ]]; then
    : # Use provided SAVANT
elif [[ -x "$SCRIPT_DIR/../../target/release/savant" ]]; then
    SAVANT="$SCRIPT_DIR/../../target/release/savant"
elif [[ -x "$SCRIPT_DIR/../../target/debug/savant" ]]; then
    SAVANT="$SCRIPT_DIR/../../target/debug/savant"
else
    SAVANT="savant"
fi

suite_start "Help and Version Commands"

PASSED=0
FAILED=0

# Test: Main --help
test_start "Main --help output"
output=$($SAVANT --help 2>&1)
if assert_contains "$output" "Kinesis Savant Elite" "Shows product name"; then
    PASSED=$((PASSED + 1))
else
    FAILED=$((FAILED + 1))
fi
test_end pass "Main --help"

# Test: --version
test_start "Version output"
output=$($SAVANT --version 2>&1)
if echo "$output" | grep -qE "savant [0-9]+\.[0-9]+"; then
    log_pass "Version shows semver format"
    PASSED=$((PASSED + 1))
else
    log_fail "Version format incorrect: $output"
    FAILED=$((FAILED + 1))
fi
test_end pass "--version"

# Test: All subcommand help
SUBCOMMANDS="program monitor status info probe keys preset config doctor completions"
for cmd in $SUBCOMMANDS; do
    test_start "Subcommand help: $cmd"
    if $SAVANT $cmd --help >/dev/null 2>&1; then
        log_pass "$cmd --help succeeds"
        PASSED=$((PASSED + 1))
    else
        log_fail "$cmd --help failed"
        FAILED=$((FAILED + 1))
    fi
    test_end pass "$cmd --help"
done

# Test: Config subcommand help
CONFIG_SUBCOMMANDS="save load list show delete check history restore"
for subcmd in $CONFIG_SUBCOMMANDS; do
    test_start "Config subcommand help: $subcmd"
    if $SAVANT config $subcmd --help >/dev/null 2>&1; then
        log_pass "config $subcmd --help succeeds"
        PASSED=$((PASSED + 1))
    else
        log_fail "config $subcmd --help failed"
        FAILED=$((FAILED + 1))
    fi
    test_end pass "config $subcmd --help"
done

# Test: Help shows global flags
test_start "Help shows global flags"
output=$($SAVANT --help 2>&1)
if echo "$output" | grep -q "\-\-json" && echo "$output" | grep -q "\-\-verbose"; then
    log_pass "Global flags shown in help"
    PASSED=$((PASSED + 1))
else
    log_fail "Missing global flags in help"
    FAILED=$((FAILED + 1))
fi
test_end pass "Global flags in help"

suite_end "$PASSED" "$FAILED"
