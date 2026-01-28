#!/bin/bash
# E2E tests for savant keys command
# Tests: key listing, modifiers, JSON output

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

suite_start "Savant Keys Command"

PASSED=0
FAILED=0

# ============================================================================
# BASIC KEYS COMMAND TESTS
# ============================================================================

test_start "Keys: command runs successfully"
if $SAVANT keys >/dev/null 2>&1; then
    log_pass "Keys command runs"
    PASSED=$((PASSED + 1))
else
    log_fail "Keys command failed"
    FAILED=$((FAILED + 1))
fi
test_end pass "keys runs"

test_start "Keys: shows help with --help"
output=$($SAVANT keys --help 2>&1)
if echo "$output" | grep -qi "key\|valid\|list"; then
    log_pass "Keys help shown"
    PASSED=$((PASSED + 1))
else
    log_fail "Keys help missing: $output"
    FAILED=$((FAILED + 1))
fi
test_end pass "keys --help"

# ============================================================================
# MODIFIER LISTING TESTS
# ============================================================================

test_start "Keys: shows cmd modifier"
output=$($SAVANT keys 2>&1)
if echo "$output" | grep -qi "cmd\|command"; then
    log_pass "Found cmd modifier"
    PASSED=$((PASSED + 1))
else
    log_fail "Missing cmd modifier"
    FAILED=$((FAILED + 1))
fi
test_end pass "keys shows cmd"

test_start "Keys: shows ctrl modifier"
output=$($SAVANT keys 2>&1)
if echo "$output" | grep -qi "ctrl\|control"; then
    log_pass "Found ctrl modifier"
    PASSED=$((PASSED + 1))
else
    log_fail "Missing ctrl modifier"
    FAILED=$((FAILED + 1))
fi
test_end pass "keys shows ctrl"

test_start "Keys: shows alt modifier"
output=$($SAVANT keys 2>&1)
if echo "$output" | grep -qi "alt\|option"; then
    log_pass "Found alt modifier"
    PASSED=$((PASSED + 1))
else
    log_fail "Missing alt modifier"
    FAILED=$((FAILED + 1))
fi
test_end pass "keys shows alt"

test_start "Keys: shows shift modifier"
output=$($SAVANT keys 2>&1)
if echo "$output" | grep -qi "shift"; then
    log_pass "Found shift modifier"
    PASSED=$((PASSED + 1))
else
    log_fail "Missing shift modifier"
    FAILED=$((FAILED + 1))
fi
test_end pass "keys shows shift"

# ============================================================================
# LETTER KEY TESTS
# ============================================================================

test_start "Keys: shows letter keys"
output=$($SAVANT keys 2>&1)
# Check for a few common letters
if echo "$output" | grep -q "a" && echo "$output" | grep -q "z"; then
    log_pass "Found letter keys"
    PASSED=$((PASSED + 1))
else
    log_fail "Missing letter keys"
    FAILED=$((FAILED + 1))
fi
test_end pass "keys shows letters"

# ============================================================================
# NUMBER KEY TESTS
# ============================================================================

test_start "Keys: shows number keys"
output=$($SAVANT keys 2>&1)
if echo "$output" | grep -q "0" && echo "$output" | grep -q "9"; then
    log_pass "Found number keys"
    PASSED=$((PASSED + 1))
else
    log_fail "Missing number keys"
    FAILED=$((FAILED + 1))
fi
test_end pass "keys shows numbers"

# ============================================================================
# FUNCTION KEY TESTS
# ============================================================================

test_start "Keys: shows function keys"
output=$($SAVANT keys 2>&1)
if echo "$output" | grep -qi "f1" && echo "$output" | grep -qi "f12"; then
    log_pass "Found function keys"
    PASSED=$((PASSED + 1))
else
    log_fail "Missing function keys"
    FAILED=$((FAILED + 1))
fi
test_end pass "keys shows function keys"

# ============================================================================
# SPECIAL KEY TESTS
# ============================================================================

test_start "Keys: shows enter key"
output=$($SAVANT keys 2>&1)
if echo "$output" | grep -qi "enter\|return"; then
    log_pass "Found enter key"
    PASSED=$((PASSED + 1))
else
    log_fail "Missing enter key"
    FAILED=$((FAILED + 1))
fi
test_end pass "keys shows enter"

test_start "Keys: shows escape key"
output=$($SAVANT keys 2>&1)
if echo "$output" | grep -qi "esc"; then
    log_pass "Found escape key"
    PASSED=$((PASSED + 1))
else
    log_fail "Missing escape key"
    FAILED=$((FAILED + 1))
fi
test_end pass "keys shows escape"

test_start "Keys: shows space key"
output=$($SAVANT keys 2>&1)
if echo "$output" | grep -qi "space"; then
    log_pass "Found space key"
    PASSED=$((PASSED + 1))
else
    log_fail "Missing space key"
    FAILED=$((FAILED + 1))
fi
test_end pass "keys shows space"

test_start "Keys: shows tab key"
output=$($SAVANT keys 2>&1)
if echo "$output" | grep -qi "tab"; then
    log_pass "Found tab key"
    PASSED=$((PASSED + 1))
else
    log_fail "Missing tab key"
    FAILED=$((FAILED + 1))
fi
test_end pass "keys shows tab"

test_start "Keys: shows backspace key"
output=$($SAVANT keys 2>&1)
if echo "$output" | grep -qi "backspace"; then
    log_pass "Found backspace key"
    PASSED=$((PASSED + 1))
else
    log_fail "Missing backspace key"
    FAILED=$((FAILED + 1))
fi
test_end pass "keys shows backspace"

# ============================================================================
# ARROW KEY TESTS
# ============================================================================

test_start "Keys: shows arrow keys"
output=$($SAVANT keys 2>&1)
if echo "$output" | grep -qi "up" && echo "$output" | grep -qi "down" && \
   echo "$output" | grep -qi "left" && echo "$output" | grep -qi "right"; then
    log_pass "Found arrow keys"
    PASSED=$((PASSED + 1))
else
    log_fail "Missing arrow keys"
    FAILED=$((FAILED + 1))
fi
test_end pass "keys shows arrows"

# ============================================================================
# JSON OUTPUT TESTS
# ============================================================================

test_start "Keys: JSON output is valid"
output=$($SAVANT --json keys 2>&1)
if echo "$output" | jq -e '.' >/dev/null 2>&1; then
    log_pass "JSON output is valid"
    PASSED=$((PASSED + 1))
else
    log_fail "JSON output invalid: $output"
    FAILED=$((FAILED + 1))
fi
test_end pass "keys --json valid"

test_start "Keys: JSON has modifiers array"
output=$($SAVANT --json keys 2>&1)
if echo "$output" | jq -e '.modifiers | type == "array"' >/dev/null 2>&1; then
    log_pass "JSON has modifiers array"
    PASSED=$((PASSED + 1))
else
    log_fail "JSON missing modifiers"
    FAILED=$((FAILED + 1))
fi
test_end pass "keys --json modifiers"

test_start "Keys: JSON has keys object"
output=$($SAVANT --json keys 2>&1)
if echo "$output" | jq -e '.keys' >/dev/null 2>&1; then
    log_pass "JSON has keys object"
    PASSED=$((PASSED + 1))
else
    log_fail "JSON missing keys"
    FAILED=$((FAILED + 1))
fi
test_end pass "keys --json keys"

test_start "Keys: JSON modifiers have names"
output=$($SAVANT --json keys 2>&1)
if echo "$output" | jq -e '.modifiers | all(.names | length > 0)' >/dev/null 2>&1; then
    log_pass "Modifiers have names"
    PASSED=$((PASSED + 1))
else
    log_fail "Modifiers missing names"
    FAILED=$((FAILED + 1))
fi
test_end pass "keys --json modifier names"

test_start "Keys: JSON has letters category"
output=$($SAVANT --json keys 2>&1)
if echo "$output" | jq -e '.keys.letters | length > 0' >/dev/null 2>&1; then
    log_pass "JSON has letters"
    PASSED=$((PASSED + 1))
else
    log_fail "JSON missing letters"
    FAILED=$((FAILED + 1))
fi
test_end pass "keys --json letters"

test_start "Keys: JSON has function_keys category"
output=$($SAVANT --json keys 2>&1)
if echo "$output" | jq -e '.keys.function_keys | length > 0' >/dev/null 2>&1; then
    log_pass "JSON has function_keys"
    PASSED=$((PASSED + 1))
else
    log_fail "JSON missing function_keys"
    FAILED=$((FAILED + 1))
fi
test_end pass "keys --json function_keys"

test_start "Keys: JSON has special category"
output=$($SAVANT --json keys 2>&1)
if echo "$output" | jq -e '.keys.special | length > 0' >/dev/null 2>&1; then
    log_pass "JSON has special keys"
    PASSED=$((PASSED + 1))
else
    log_fail "JSON missing special keys"
    FAILED=$((FAILED + 1))
fi
test_end pass "keys --json special"

# ============================================================================
# VERBOSE MODE TESTS
# ============================================================================

test_start "Keys: verbose mode runs"
output=$($SAVANT keys --verbose 2>&1)
if echo "$output" | grep -qi "key\|modifier"; then
    log_pass "Verbose mode runs"
    PASSED=$((PASSED + 1))
else
    log_fail "Verbose mode failed: $output"
    FAILED=$((FAILED + 1))
fi
test_end pass "keys --verbose"

# ============================================================================
# CLEANUP
# ============================================================================

suite_end "$PASSED" "$FAILED"
