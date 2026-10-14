#!/bin/bash
# E2E tests for verbose mode (-v/--verbose)
# Tests: both flags accepted anywhere, debug lines go to stderr, JSON stdout stays parseable.
# None of these needs a pedal: preset/program --dry-run only parse keys.

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

# Isolated config dir, so doctor/info never read the real configuration
TEMP_DIR=$(mktemp -d)
trap 'rm -rf -- "$TEMP_DIR"' EXIT
export HOME="$TEMP_DIR"
export XDG_CONFIG_HOME="$TEMP_DIR/.config"

suite_start "Verbose Mode"

PASSED=0
FAILED=0

# ============================================================================
# FLAG ACCEPTANCE
# ============================================================================

for flag in -v --verbose; do
    test_start "Verbose flag accepted: $flag"
    if $SAVANT $flag keys >/dev/null 2>&1 && $SAVANT keys $flag >/dev/null 2>&1; then
        log_pass "$flag accepted before and after the subcommand"
        PASSED=$((PASSED + 1))
    else
        log_fail "$flag rejected"
        FAILED=$((FAILED + 1))
    fi
    test_end pass "$flag accepted"
done

# ============================================================================
# DEBUG OUTPUT
# ============================================================================

test_start "Verbose prints debug lines on stderr"
stderr=$($SAVANT --verbose keys 2>&1 >/dev/null)
if assert_contains "$stderr" "\[verbose\] Verbose mode enabled" "stderr announces verbose mode"; then
    PASSED=$((PASSED + 1))
else
    FAILED=$((FAILED + 1))
fi
test_end pass "verbose stderr"

test_start "Without verbose there are no debug lines"
output=$($SAVANT keys 2>&1)
if echo "$output" | grep -q "\[verbose\]"; then
    log_fail "[verbose] lines printed without -v"
    FAILED=$((FAILED + 1))
else
    log_pass "No [verbose] lines without -v"
    PASSED=$((PASSED + 1))
fi
test_end pass "no verbose by default"

test_start "Verbose reports the USB timeout"
stderr=$($SAVANT -v --timeout 750 keys 2>&1 >/dev/null)
if assert_contains "$stderr" "USB timeout: 750ms" "Custom --timeout shown"; then
    PASSED=$((PASSED + 1))
else
    FAILED=$((FAILED + 1))
fi
test_end pass "verbose timeout"

test_start "Verbose shows how pedal keys parse"
output=$($SAVANT -v preset copy-paste --dry-run 2>&1 || true)
if assert_contains "$output" "Parsing left pedal action: 'cmd+c'" "Left pedal parse logged" &&
    assert_contains "$output" "modifiers=0x08, key=0x06" "cmd+c resolves to Command + HID 0x06"; then
    PASSED=$((PASSED + 1))
else
    FAILED=$((FAILED + 1))
fi
test_end pass "verbose key parsing"

# ============================================================================
# VERBOSE + JSON
# ============================================================================

for cmd in doctor status keys; do
    test_start "Verbose keeps JSON stdout parseable: $cmd"
    if $SAVANT --verbose --json $cmd 2>/dev/null | jq -e . >/dev/null 2>&1; then
        log_pass "--verbose --json $cmd: stdout is valid JSON"
        PASSED=$((PASSED + 1))
    else
        log_fail "--verbose --json $cmd: stdout is not valid JSON"
        FAILED=$((FAILED + 1))
    fi
    test_end pass "verbose json $cmd"
done

test_start "Verbose doctor JSON has its fields"
output=$($SAVANT --verbose --json doctor 2>/dev/null)
if echo "$output" | jq -e '.version and (.checks | length > 0)' >/dev/null 2>&1; then
    log_pass "doctor JSON has version and checks"
    PASSED=$((PASSED + 1))
else
    log_fail "doctor JSON incomplete: $output"
    FAILED=$((FAILED + 1))
fi
test_end pass "verbose doctor json"

suite_end "$PASSED" "$FAILED"
