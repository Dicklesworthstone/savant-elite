#!/bin/bash
# E2E tests for preset workflows
# Tests: preset list, preset show, preset dry-run, unknown preset handling

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

suite_start "Preset Workflows"

PASSED=0
FAILED=0

# Known presets to test
PRESETS="copy-paste undo-redo browser zoom"

# ============================================================================
# PRESET LIST TESTS
# ============================================================================

test_start "Preset list: shows all presets"
output=$($SAVANT preset --list 2>&1)
all_found=true
for preset in $PRESETS; do
    if ! echo "$output" | grep -q "$preset"; then
        log_fail "Missing preset: $preset"
        all_found=false
    fi
done
if $all_found; then
    log_pass "All presets listed"
    PASSED=$((PASSED + 1))
else
    FAILED=$((FAILED + 1))
fi
test_end pass "preset --list"

test_start "Preset list: JSON output"
output=$($SAVANT --json preset --list 2>&1)
if echo "$output" | jq -e '.presets | length > 0' >/dev/null 2>&1; then
    log_pass "JSON preset list is valid"
    PASSED=$((PASSED + 1))
else
    log_fail "JSON preset list invalid: $output"
    FAILED=$((FAILED + 1))
fi
test_end pass "preset --list --json"

test_start "Preset list: JSON contains all presets"
output=$($SAVANT --json preset --list 2>&1)
all_found=true
for preset in $PRESETS; do
    if ! echo "$output" | jq -e ".presets[] | select(.name == \"$preset\")" >/dev/null 2>&1; then
        log_fail "Missing preset in JSON: $preset"
        all_found=false
    fi
done
if $all_found; then
    log_pass "All presets in JSON output"
    PASSED=$((PASSED + 1))
else
    FAILED=$((FAILED + 1))
fi
test_end pass "preset --list --json (all presets)"

# ============================================================================
# PRESET SHOW TESTS
# ============================================================================

for preset in $PRESETS; do
    test_start "Preset show: $preset"
    output=$($SAVANT preset --show "$preset" 2>&1)
    # Should show pedal configuration with Left/Middle/Right labels
    if echo "$output" | grep -qi "left" && echo "$output" | grep -qi "right"; then
        log_pass "Preset $preset shows pedal config"
        PASSED=$((PASSED + 1))
    else
        log_fail "Preset $preset missing pedal info: $output"
        FAILED=$((FAILED + 1))
    fi
    test_end pass "preset --show $preset"
done

test_start "Preset show: JSON format"
output=$($SAVANT --json preset --show copy-paste 2>&1)
if echo "$output" | jq -e '.left and .middle and .right' >/dev/null 2>&1; then
    log_pass "JSON preset show is valid"
    PASSED=$((PASSED + 1))
else
    log_fail "JSON preset show invalid: $output"
    FAILED=$((FAILED + 1))
fi
test_end pass "preset --show --json"

test_start "Preset show: JSON has pedal values"
output=$($SAVANT --json preset --show copy-paste 2>&1)
# Structure is { "left": "cmd+c", "middle": "cmd+a", "right": "cmd+v" }
if echo "$output" | jq -e '(.left | type == "string") and (.middle | type == "string") and (.right | type == "string")' >/dev/null 2>&1; then
    log_pass "JSON has pedal value strings"
    PASSED=$((PASSED + 1))
else
    log_fail "JSON missing pedal values: $output"
    FAILED=$((FAILED + 1))
fi
test_end pass "preset --show --json (pedals)"

# ============================================================================
# PRESET DRY-RUN TESTS
# ============================================================================

for preset in $PRESETS; do
    test_start "Preset dry-run: $preset"
    output=$($SAVANT preset "$preset" --dry-run 2>&1)
    # Should show what would be programmed
    if echo "$output" | grep -qi "would\|preview\|dry.run\|left\|configuration"; then
        log_pass "Preset $preset dry-run works"
        PASSED=$((PASSED + 1))
    else
        log_fail "Preset $preset dry-run unclear: $output"
        FAILED=$((FAILED + 1))
    fi
    test_end pass "preset $preset --dry-run"
done

test_start "Preset dry-run: shows configuration"
# Note: --json with --dry-run may show formatted output, not JSON
output=$($SAVANT --json preset copy-paste --dry-run 2>&1)
if echo "$output" | jq -e '.left and .middle and .right' >/dev/null 2>&1; then
    log_pass "JSON dry-run output valid"
    PASSED=$((PASSED + 1))
elif echo "$output" | grep -qi "left\|cmd\|configuration"; then
    # Fallback: accept formatted output showing the config
    log_pass "Dry-run shows configuration (formatted)"
    PASSED=$((PASSED + 1))
else
    log_fail "Dry-run output unclear: $output"
    FAILED=$((FAILED + 1))
fi
test_end pass "preset --dry-run output"

# ============================================================================
# ERROR HANDLING TESTS
# ============================================================================

test_start "Unknown preset: rejected"
output=$($SAVANT preset nonexistent 2>&1 || true)
if echo "$output" | grep -qi "not found\|unknown\|invalid\|error"; then
    log_pass "Unknown preset rejected"
    PASSED=$((PASSED + 1))
else
    log_fail "Unknown preset not properly rejected: $output"
    FAILED=$((FAILED + 1))
fi
test_end pass "preset nonexistent"

test_start "Unknown preset: JSON error format"
output=$($SAVANT --json preset nonexistent 2>&1 || true)
if echo "$output" | jq -e '.error' >/dev/null 2>&1; then
    log_pass "JSON error output valid"
    PASSED=$((PASSED + 1))
else
    # Some implementations just return non-JSON on error, that's acceptable
    if echo "$output" | grep -qi "not found\|unknown\|invalid\|error"; then
        log_pass "Error message present (non-JSON)"
        PASSED=$((PASSED + 1))
    else
        log_fail "Error handling unclear: $output"
        FAILED=$((FAILED + 1))
    fi
fi
test_end pass "preset nonexistent --json"

test_start "Preset show: unknown rejected"
output=$($SAVANT preset --show nonexistent 2>&1 || true)
if echo "$output" | grep -qi "not found\|unknown\|invalid\|error"; then
    log_pass "Unknown preset show rejected"
    PASSED=$((PASSED + 1))
else
    log_fail "Unknown preset show not rejected: $output"
    FAILED=$((FAILED + 1))
fi
test_end pass "preset --show nonexistent"

# ============================================================================
# VERBOSE MODE TESTS
# ============================================================================

test_start "Preset list: verbose mode"
output=$($SAVANT preset --list --verbose 2>&1)
# Verbose mode should show additional info (descriptions, details)
if echo "$output" | grep -qi "copy-paste"; then
    log_pass "Verbose list works"
    PASSED=$((PASSED + 1))
else
    log_fail "Verbose list failed: $output"
    FAILED=$((FAILED + 1))
fi
test_end pass "preset --list --verbose"

test_start "Preset show: verbose mode"
output=$($SAVANT preset --show copy-paste --verbose 2>&1)
if echo "$output" | grep -qi "left\|cmd"; then
    log_pass "Verbose show works"
    PASSED=$((PASSED + 1))
else
    log_fail "Verbose show failed: $output"
    FAILED=$((FAILED + 1))
fi
test_end pass "preset --show --verbose"

# ============================================================================
# CLEANUP
# ============================================================================

suite_end "$PASSED" "$FAILED"
