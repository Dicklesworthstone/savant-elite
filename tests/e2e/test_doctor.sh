#!/bin/bash
# E2E tests for doctor command diagnostics
# Tests: doctor output, JSON format, check types, summary validation

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

suite_start "Doctor Command Diagnostics"

PASSED=0
FAILED=0

# ============================================================================
# BASIC DOCTOR COMMAND TESTS
# ============================================================================

test_start "Doctor: runs successfully"
if $SAVANT doctor >/dev/null 2>&1; then
    log_pass "Doctor command runs"
    PASSED=$((PASSED + 1))
else
    log_fail "Doctor command failed"
    FAILED=$((FAILED + 1))
fi
test_end pass "doctor runs"

test_start "Doctor: shows version info"
output=$($SAVANT doctor 2>&1)
if echo "$output" | grep -qi "version\|v[0-9]"; then
    log_pass "Version info shown"
    PASSED=$((PASSED + 1))
else
    log_fail "Version info missing: $output"
    FAILED=$((FAILED + 1))
fi
test_end pass "doctor shows version"

test_start "Doctor: shows platform info"
output=$($SAVANT doctor 2>&1)
if echo "$output" | grep -qi "macos\|linux\|windows\|platform"; then
    log_pass "Platform info shown"
    PASSED=$((PASSED + 1))
else
    log_fail "Platform info missing: $output"
    FAILED=$((FAILED + 1))
fi
test_end pass "doctor shows platform"

test_start "Doctor: shows summary"
output=$($SAVANT doctor 2>&1)
if echo "$output" | grep -qi "summary\|passed\|check"; then
    log_pass "Summary shown"
    PASSED=$((PASSED + 1))
else
    log_fail "Summary missing: $output"
    FAILED=$((FAILED + 1))
fi
test_end pass "doctor shows summary"

# ============================================================================
# JSON OUTPUT TESTS
# ============================================================================

test_start "Doctor: JSON output is valid"
output=$($SAVANT --json doctor 2>&1)
if echo "$output" | jq -e '.' >/dev/null 2>&1; then
    log_pass "JSON output is valid"
    PASSED=$((PASSED + 1))
else
    log_fail "JSON output invalid: $output"
    FAILED=$((FAILED + 1))
fi
test_end pass "doctor --json valid"

test_start "Doctor: JSON has version field"
output=$($SAVANT --json doctor 2>&1)
if echo "$output" | jq -e '.version' >/dev/null 2>&1; then
    log_pass "JSON has version"
    PASSED=$((PASSED + 1))
else
    log_fail "JSON missing version: $output"
    FAILED=$((FAILED + 1))
fi
test_end pass "doctor --json version"

test_start "Doctor: JSON has platform field"
output=$($SAVANT --json doctor 2>&1)
if echo "$output" | jq -e '.platform' >/dev/null 2>&1; then
    log_pass "JSON has platform"
    PASSED=$((PASSED + 1))
else
    log_fail "JSON missing platform: $output"
    FAILED=$((FAILED + 1))
fi
test_end pass "doctor --json platform"

test_start "Doctor: JSON has arch field"
output=$($SAVANT --json doctor 2>&1)
if echo "$output" | jq -e '.arch' >/dev/null 2>&1; then
    log_pass "JSON has arch"
    PASSED=$((PASSED + 1))
else
    log_fail "JSON missing arch: $output"
    FAILED=$((FAILED + 1))
fi
test_end pass "doctor --json arch"

test_start "Doctor: JSON has checks array"
output=$($SAVANT --json doctor 2>&1)
if echo "$output" | jq -e '.checks | type == "array"' >/dev/null 2>&1; then
    log_pass "JSON has checks array"
    PASSED=$((PASSED + 1))
else
    log_fail "JSON missing checks array: $output"
    FAILED=$((FAILED + 1))
fi
test_end pass "doctor --json checks"

test_start "Doctor: JSON has summary object"
output=$($SAVANT --json doctor 2>&1)
if echo "$output" | jq -e '.summary | type == "object"' >/dev/null 2>&1; then
    log_pass "JSON has summary object"
    PASSED=$((PASSED + 1))
else
    log_fail "JSON missing summary: $output"
    FAILED=$((FAILED + 1))
fi
test_end pass "doctor --json summary"

# ============================================================================
# CHECK TYPES VALIDATION
# ============================================================================

test_start "Doctor: has binary check"
output=$($SAVANT --json doctor 2>&1)
if echo "$output" | jq -e '.checks[] | select(.name == "binary")' >/dev/null 2>&1; then
    log_pass "Has binary check"
    PASSED=$((PASSED + 1))
else
    log_fail "Missing binary check"
    FAILED=$((FAILED + 1))
fi
test_end pass "doctor check: binary"

test_start "Doctor: has platform check"
output=$($SAVANT --json doctor 2>&1)
if echo "$output" | jq -e '.checks[] | select(.name == "platform")' >/dev/null 2>&1; then
    log_pass "Has platform check"
    PASSED=$((PASSED + 1))
else
    log_fail "Missing platform check"
    FAILED=$((FAILED + 1))
fi
test_end pass "doctor check: platform"

test_start "Doctor: has device check"
output=$($SAVANT --json doctor 2>&1)
if echo "$output" | jq -e '.checks[] | select(.name == "device")' >/dev/null 2>&1; then
    log_pass "Has device check"
    PASSED=$((PASSED + 1))
else
    log_fail "Missing device check"
    FAILED=$((FAILED + 1))
fi
test_end pass "doctor check: device"

test_start "Doctor: has config check"
output=$($SAVANT --json doctor 2>&1)
if echo "$output" | jq -e '.checks[] | select(.name == "config")' >/dev/null 2>&1; then
    log_pass "Has config check"
    PASSED=$((PASSED + 1))
else
    log_fail "Missing config check"
    FAILED=$((FAILED + 1))
fi
test_end pass "doctor check: config"

test_start "Doctor: has profiles check"
output=$($SAVANT --json doctor 2>&1)
if echo "$output" | jq -e '.checks[] | select(.name == "profiles")' >/dev/null 2>&1; then
    log_pass "Has profiles check"
    PASSED=$((PASSED + 1))
else
    log_fail "Missing profiles check"
    FAILED=$((FAILED + 1))
fi
test_end pass "doctor check: profiles"

test_start "Doctor: has permissions check"
output=$($SAVANT --json doctor 2>&1)
if echo "$output" | jq -e '.checks[] | select(.name == "permissions")' >/dev/null 2>&1; then
    log_pass "Has permissions check"
    PASSED=$((PASSED + 1))
else
    log_fail "Missing permissions check"
    FAILED=$((FAILED + 1))
fi
test_end pass "doctor check: permissions"

# ============================================================================
# CHECK STRUCTURE VALIDATION
# ============================================================================

test_start "Doctor: checks have name field"
output=$($SAVANT --json doctor 2>&1)
if echo "$output" | jq -e '.checks | all(.name)' >/dev/null 2>&1; then
    log_pass "All checks have name"
    PASSED=$((PASSED + 1))
else
    log_fail "Some checks missing name"
    FAILED=$((FAILED + 1))
fi
test_end pass "doctor checks have name"

test_start "Doctor: checks have status field"
output=$($SAVANT --json doctor 2>&1)
if echo "$output" | jq -e '.checks | all(.status)' >/dev/null 2>&1; then
    log_pass "All checks have status"
    PASSED=$((PASSED + 1))
else
    log_fail "Some checks missing status"
    FAILED=$((FAILED + 1))
fi
test_end pass "doctor checks have status"

test_start "Doctor: checks have message field"
output=$($SAVANT --json doctor 2>&1)
if echo "$output" | jq -e '.checks | all(.message)' >/dev/null 2>&1; then
    log_pass "All checks have message"
    PASSED=$((PASSED + 1))
else
    log_fail "Some checks missing message"
    FAILED=$((FAILED + 1))
fi
test_end pass "doctor checks have message"

test_start "Doctor: status values are valid"
output=$($SAVANT --json doctor 2>&1)
if echo "$output" | jq -e '.checks | all(.status == "pass" or .status == "warn" or .status == "fail")' >/dev/null 2>&1; then
    log_pass "All status values valid"
    PASSED=$((PASSED + 1))
else
    log_fail "Invalid status values"
    FAILED=$((FAILED + 1))
fi
test_end pass "doctor status values"

# ============================================================================
# SUMMARY VALIDATION
# ============================================================================

test_start "Doctor: summary has total field"
output=$($SAVANT --json doctor 2>&1)
if echo "$output" | jq -e '.summary.total | type == "number"' >/dev/null 2>&1; then
    log_pass "Summary has total"
    PASSED=$((PASSED + 1))
else
    log_fail "Summary missing total"
    FAILED=$((FAILED + 1))
fi
test_end pass "doctor summary total"

test_start "Doctor: summary has passed field"
output=$($SAVANT --json doctor 2>&1)
if echo "$output" | jq -e '.summary.passed | type == "number"' >/dev/null 2>&1; then
    log_pass "Summary has passed"
    PASSED=$((PASSED + 1))
else
    log_fail "Summary missing passed"
    FAILED=$((FAILED + 1))
fi
test_end pass "doctor summary passed"

test_start "Doctor: summary has warnings field"
output=$($SAVANT --json doctor 2>&1)
if echo "$output" | jq -e '.summary.warnings | type == "number"' >/dev/null 2>&1; then
    log_pass "Summary has warnings"
    PASSED=$((PASSED + 1))
else
    log_fail "Summary missing warnings"
    FAILED=$((FAILED + 1))
fi
test_end pass "doctor summary warnings"

test_start "Doctor: summary has failed field"
output=$($SAVANT --json doctor 2>&1)
if echo "$output" | jq -e '.summary.failed | type == "number"' >/dev/null 2>&1; then
    log_pass "Summary has failed"
    PASSED=$((PASSED + 1))
else
    log_fail "Summary missing failed"
    FAILED=$((FAILED + 1))
fi
test_end pass "doctor summary failed"

test_start "Doctor: summary has healthy field"
output=$($SAVANT --json doctor 2>&1)
if echo "$output" | jq -e '.summary.healthy | type == "boolean"' >/dev/null 2>&1; then
    log_pass "Summary has healthy"
    PASSED=$((PASSED + 1))
else
    log_fail "Summary missing healthy"
    FAILED=$((FAILED + 1))
fi
test_end pass "doctor summary healthy"

test_start "Doctor: summary math is correct"
output=$($SAVANT --json doctor 2>&1)
if echo "$output" | jq -e '.summary.total == (.summary.passed + .summary.warnings + .summary.failed)' >/dev/null 2>&1; then
    log_pass "Summary counts add up correctly"
    PASSED=$((PASSED + 1))
else
    log_fail "Summary math incorrect"
    FAILED=$((FAILED + 1))
fi
test_end pass "doctor summary math"

test_start "Doctor: checks count matches total"
output=$($SAVANT --json doctor 2>&1)
if echo "$output" | jq -e '(.checks | length) == .summary.total' >/dev/null 2>&1; then
    log_pass "Checks count matches total"
    PASSED=$((PASSED + 1))
else
    log_fail "Checks count mismatch"
    FAILED=$((FAILED + 1))
fi
test_end pass "doctor checks count"

# ============================================================================
# VERBOSE MODE TESTS
# ============================================================================

test_start "Doctor: verbose mode runs"
output=$($SAVANT doctor --verbose 2>&1)
if [[ $? -eq 0 ]] || echo "$output" | grep -qi "check\|version"; then
    log_pass "Verbose mode runs"
    PASSED=$((PASSED + 1))
else
    log_fail "Verbose mode failed: $output"
    FAILED=$((FAILED + 1))
fi
test_end pass "doctor --verbose"

test_start "Doctor: verbose JSON output"
# Only capture stdout (JSON), not stderr (verbose messages)
output=$($SAVANT --json doctor --verbose 2>/dev/null)
if echo "$output" | jq -e '.' >/dev/null 2>&1; then
    log_pass "Verbose JSON is valid"
    PASSED=$((PASSED + 1))
else
    log_fail "Verbose JSON invalid: $output"
    FAILED=$((FAILED + 1))
fi
test_end pass "doctor --json --verbose"

# ============================================================================
# CLEANUP
# ============================================================================

suite_end "$PASSED" "$FAILED"
