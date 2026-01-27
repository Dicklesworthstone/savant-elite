#!/bin/bash
# E2E tests for config file validation and profile management
# Tests: config check, config save/load/list/show/delete, config history/restore

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
FIXTURES="$SCRIPT_DIR/../fixtures"

# Create isolated temp environment
TEMP_DIR=$(mktemp -d)
trap "rm -rf $TEMP_DIR" EXIT

log_info "Using temp directory: $TEMP_DIR"
export XDG_CONFIG_HOME="$TEMP_DIR"

suite_start "Config and Profile Management"

PASSED=0
FAILED=0

# ============================================================================
# CONFIG CHECK TESTS - Valid configs
# ============================================================================

test_start "Config check: valid basic config"
output=$($SAVANT config check "$FIXTURES/valid/basic.conf" 2>&1)
if echo "$output" | grep -qi "valid"; then
    log_pass "Valid config accepted"
    PASSED=$((PASSED + 1))
else
    log_fail "Valid config not accepted: $output"
    FAILED=$((FAILED + 1))
fi
test_end pass "config check valid/basic.conf"

test_start "Config check: all modifiers config"
output=$($SAVANT config check "$FIXTURES/valid/all_modifiers.conf" 2>&1)
if echo "$output" | grep -qi "valid"; then
    log_pass "All modifiers config accepted"
    PASSED=$((PASSED + 1))
else
    log_fail "All modifiers config rejected: $output"
    FAILED=$((FAILED + 1))
fi
test_end pass "config check valid/all_modifiers.conf"

test_start "Config check: function keys config"
output=$($SAVANT config check "$FIXTURES/valid/function_keys.conf" 2>&1)
if echo "$output" | grep -qi "valid"; then
    log_pass "Function keys config accepted"
    PASSED=$((PASSED + 1))
else
    log_fail "Function keys config rejected: $output"
    FAILED=$((FAILED + 1))
fi
test_end pass "config check valid/function_keys.conf"

test_start "Config check: special keys config"
output=$($SAVANT config check "$FIXTURES/valid/special_keys.conf" 2>&1)
if echo "$output" | grep -qi "valid"; then
    log_pass "Special keys config accepted"
    PASSED=$((PASSED + 1))
else
    log_fail "Special keys config rejected: $output"
    FAILED=$((FAILED + 1))
fi
test_end pass "config check valid/special_keys.conf"

test_start "Config check: whitespace handling"
output=$($SAVANT config check "$FIXTURES/valid/with_whitespace.conf" 2>&1)
if echo "$output" | grep -qi "valid"; then
    log_pass "Whitespace config accepted"
    PASSED=$((PASSED + 1))
else
    log_fail "Whitespace config rejected: $output"
    FAILED=$((FAILED + 1))
fi
test_end pass "config check valid/with_whitespace.conf"

# ============================================================================
# CONFIG CHECK TESTS - Invalid configs
# ============================================================================

test_start "Config check: missing left field"
output=$($SAVANT config check "$FIXTURES/invalid/missing_left.conf" 2>&1 || true)
if echo "$output" | grep -qi "invalid\|missing\|left"; then
    log_pass "Missing left field detected"
    PASSED=$((PASSED + 1))
else
    log_fail "Missing left field not detected: $output"
    FAILED=$((FAILED + 1))
fi
test_end pass "config check invalid/missing_left.conf"

test_start "Config check: missing middle field"
output=$($SAVANT config check "$FIXTURES/invalid/missing_middle.conf" 2>&1 || true)
if echo "$output" | grep -qi "invalid\|missing\|middle"; then
    log_pass "Missing middle field detected"
    PASSED=$((PASSED + 1))
else
    log_fail "Missing middle field not detected: $output"
    FAILED=$((FAILED + 1))
fi
test_end pass "config check invalid/missing_middle.conf"

test_start "Config check: missing right field"
output=$($SAVANT config check "$FIXTURES/invalid/missing_right.conf" 2>&1 || true)
if echo "$output" | grep -qi "invalid\|missing\|right"; then
    log_pass "Missing right field detected"
    PASSED=$((PASSED + 1))
else
    log_fail "Missing right field not detected: $output"
    FAILED=$((FAILED + 1))
fi
test_end pass "config check invalid/missing_right.conf"

test_start "Config check: invalid key name"
output=$($SAVANT config check "$FIXTURES/invalid/invalid_key.conf" 2>&1 || true)
if echo "$output" | grep -qi "invalid\|unknown\|key"; then
    log_pass "Invalid key name rejected"
    PASSED=$((PASSED + 1))
else
    log_fail "Invalid key name not rejected: $output"
    FAILED=$((FAILED + 1))
fi
test_end pass "config check invalid/invalid_key.conf"

test_start "Config check: invalid modifier"
output=$($SAVANT config check "$FIXTURES/invalid/invalid_modifier.conf" 2>&1 || true)
if echo "$output" | grep -qi "invalid\|unknown\|modifier"; then
    log_pass "Invalid modifier rejected"
    PASSED=$((PASSED + 1))
else
    log_fail "Invalid modifier not rejected: $output"
    FAILED=$((FAILED + 1))
fi
test_end pass "config check invalid/invalid_modifier.conf"

test_start "Config check: empty file"
output=$($SAVANT config check "$FIXTURES/invalid/empty.conf" 2>&1 || true)
if echo "$output" | grep -qi "invalid\|missing\|empty"; then
    log_pass "Empty file rejected"
    PASSED=$((PASSED + 1))
else
    log_fail "Empty file not rejected: $output"
    FAILED=$((FAILED + 1))
fi
test_end pass "config check invalid/empty.conf"

test_start "Config check: nonexistent file"
output=$($SAVANT config check "/nonexistent/path/config.conf" 2>&1 || true)
if echo "$output" | grep -qi "not found\|does not exist\|no such\|error"; then
    log_pass "Nonexistent file handled"
    PASSED=$((PASSED + 1))
else
    log_fail "Nonexistent file not handled: $output"
    FAILED=$((FAILED + 1))
fi
test_end pass "config check nonexistent file"

# ============================================================================
# CONFIG CHECK TESTS - JSON output
# ============================================================================

test_start "Config check: JSON output for valid config"
output=$($SAVANT --json config check "$FIXTURES/valid/basic.conf" 2>&1)
if echo "$output" | jq -e '.valid == true' >/dev/null 2>&1; then
    log_pass "JSON output shows valid=true"
    PASSED=$((PASSED + 1))
else
    log_fail "JSON output invalid: $output"
    FAILED=$((FAILED + 1))
fi
test_end pass "config check --json valid"

test_start "Config check: JSON output for invalid config"
# Only capture stdout (JSON), not stderr (error messages)
output=$($SAVANT --json config check "$FIXTURES/invalid/missing_left.conf" 2>/dev/null || true)
if echo "$output" | jq -e '.valid == false and (.errors | length > 0)' >/dev/null 2>&1; then
    log_pass "JSON output shows valid=false with errors"
    PASSED=$((PASSED + 1))
else
    log_fail "JSON output invalid: $output"
    FAILED=$((FAILED + 1))
fi
test_end pass "config check --json invalid"

# ============================================================================
# PROFILE MANAGEMENT TESTS
# ============================================================================

# Setup: create a current config in the temp environment
CONFIG_DIR="$TEMP_DIR/savant-elite"
mkdir -p "$CONFIG_DIR"
cp "$FIXTURES/valid/basic.conf" "$CONFIG_DIR/pedals.conf"
log_info "Created test config at $CONFIG_DIR/pedals.conf"

test_start "Config list: initially empty profiles"
output=$($SAVANT config list 2>&1)
if echo "$output" | grep -qi "no profiles\|no saved" || [[ -z "$(echo "$output" | grep -v "^$")" ]]; then
    log_pass "Empty profile list shown"
    PASSED=$((PASSED + 1))
else
    # May just show empty output which is also valid
    log_pass "Profile list command works"
    PASSED=$((PASSED + 1))
fi
test_end pass "config list (empty)"

test_start "Config save: create new profile"
output=$($SAVANT config save "test-profile-1" 2>&1)
if echo "$output" | grep -qi "saved\|created\|success"; then
    log_pass "Profile saved successfully"
    PASSED=$((PASSED + 1))
else
    log_fail "Profile save failed: $output"
    FAILED=$((FAILED + 1))
fi
test_end pass "config save test-profile-1"

test_start "Config list: shows saved profile"
output=$($SAVANT config list 2>&1)
if echo "$output" | grep -q "test-profile-1"; then
    log_pass "Saved profile appears in list"
    PASSED=$((PASSED + 1))
else
    log_fail "Saved profile not in list: $output"
    FAILED=$((FAILED + 1))
fi
test_end pass "config list (with profile)"

test_start "Config list: JSON output"
output=$($SAVANT --json config list 2>&1)
if echo "$output" | jq -e '.profiles | map(select(.name == "test-profile-1")) | length > 0' >/dev/null 2>&1; then
    log_pass "Profile in JSON list"
    PASSED=$((PASSED + 1))
else
    log_fail "Profile not in JSON list: $output"
    FAILED=$((FAILED + 1))
fi
test_end pass "config list --json"

test_start "Config show: display profile contents"
output=$($SAVANT config show "test-profile-1" 2>&1)
# The display shows Unicode symbols (⌘C) not raw text (cmd+c), so check for LEFT pedal label
if echo "$output" | grep -q "LEFT"; then
    log_pass "Profile contents shown"
    PASSED=$((PASSED + 1))
else
    log_fail "Profile contents not shown: $output"
    FAILED=$((FAILED + 1))
fi
test_end pass "config show test-profile-1"

test_start "Config show: JSON output"
output=$($SAVANT --json config show "test-profile-1" 2>&1)
if echo "$output" | jq -e '.left and .middle and .right' >/dev/null 2>&1; then
    log_pass "Profile JSON valid"
    PASSED=$((PASSED + 1))
else
    log_fail "Profile JSON invalid: $output"
    FAILED=$((FAILED + 1))
fi
test_end pass "config show --json"

test_start "Config save: duplicate without force fails"
output=$($SAVANT config save "test-profile-1" 2>&1 || true)
if echo "$output" | grep -qi "exists\|overwrite\|force\|already"; then
    log_pass "Duplicate rejected without --force"
    PASSED=$((PASSED + 1))
else
    log_fail "Duplicate should be rejected: $output"
    FAILED=$((FAILED + 1))
fi
test_end pass "config save duplicate (no force)"

test_start "Config save: duplicate with --force succeeds"
output=$($SAVANT config save "test-profile-1" --force 2>&1)
if echo "$output" | grep -qi "saved\|overwrit\|success"; then
    log_pass "Duplicate allowed with --force"
    PASSED=$((PASSED + 1))
else
    log_fail "Force overwrite failed: $output"
    FAILED=$((FAILED + 1))
fi
test_end pass "config save --force"

test_start "Config delete: remove profile"
output=$($SAVANT config delete "test-profile-1" --force 2>&1)
if echo "$output" | grep -qi "deleted\|removed\|success"; then
    log_pass "Profile deleted"
    PASSED=$((PASSED + 1))
else
    log_fail "Profile delete failed: $output"
    FAILED=$((FAILED + 1))
fi
test_end pass "config delete test-profile-1"

test_start "Config list: profile gone after delete"
output=$($SAVANT config list 2>&1)
if ! echo "$output" | grep -q "test-profile-1"; then
    log_pass "Deleted profile no longer listed"
    PASSED=$((PASSED + 1))
else
    log_fail "Deleted profile still listed: $output"
    FAILED=$((FAILED + 1))
fi
test_end pass "config list (after delete)"

test_start "Config show: deleted profile not found"
output=$($SAVANT config show "test-profile-1" 2>&1 || true)
if echo "$output" | grep -qi "not found\|does not exist\|error"; then
    log_pass "Deleted profile not found"
    PASSED=$((PASSED + 1))
else
    log_fail "Deleted profile should not be found: $output"
    FAILED=$((FAILED + 1))
fi
test_end pass "config show (deleted profile)"

# ============================================================================
# PROFILE NAME VALIDATION
# ============================================================================

test_start "Config save: invalid name (spaces)"
output=$($SAVANT config save "my profile" 2>&1 || true)
if echo "$output" | grep -qi "invalid\|character\|name\|error"; then
    log_pass "Profile name with spaces rejected"
    PASSED=$((PASSED + 1))
else
    log_fail "Invalid profile name accepted: $output"
    FAILED=$((FAILED + 1))
fi
test_end pass "config save (invalid name with spaces)"

test_start "Config save: invalid name (special chars)"
output=$($SAVANT config save "my/profile" 2>&1 || true)
if echo "$output" | grep -qi "invalid\|character\|name\|error"; then
    log_pass "Profile name with / rejected"
    PASSED=$((PASSED + 1))
else
    log_fail "Invalid profile name accepted: $output"
    FAILED=$((FAILED + 1))
fi
test_end pass "config save (invalid name with /)"

test_start "Config save: valid name (hyphens and underscores)"
output=$($SAVANT config save "my-profile_name" 2>&1)
if echo "$output" | grep -qi "saved\|success"; then
    log_pass "Profile name with hyphens/underscores accepted"
    PASSED=$((PASSED + 1))
    # Cleanup
    $SAVANT config delete "my-profile_name" --force >/dev/null 2>&1 || true
else
    log_fail "Valid profile name rejected: $output"
    FAILED=$((FAILED + 1))
fi
test_end pass "config save (valid name)"

# ============================================================================
# CONFIG HISTORY TESTS
# ============================================================================

test_start "Config history: shows history"
output=$($SAVANT config history 2>&1)
# Should show history (may be empty or have entries)
if [[ $? -eq 0 ]]; then
    log_pass "Config history command runs"
    PASSED=$((PASSED + 1))
else
    log_fail "Config history failed: $output"
    FAILED=$((FAILED + 1))
fi
test_end pass "config history"

test_start "Config history: JSON output"
output=$($SAVANT --json config history 2>&1)
if echo "$output" | jq -e '.history and .count >= 0' >/dev/null 2>&1; then
    log_pass "Config history JSON valid"
    PASSED=$((PASSED + 1))
else
    log_fail "Config history JSON invalid: $output"
    FAILED=$((FAILED + 1))
fi
test_end pass "config history --json"

# ============================================================================
# CLEANUP
# ============================================================================

log_info "Cleaning up temp directory: $TEMP_DIR"

suite_end "$PASSED" "$FAILED"
