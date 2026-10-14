#!/bin/bash
# E2E tests for shell completions
# Tests: completions for bash/zsh/fish are generated, parse, and complete real subcommands

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

suite_start "Shell Completions"

PASSED=0
FAILED=0

# Run the generated bash completion function for a command line and print COMPREPLY.
# Usage: bash_complete <cword> word0 word1 ...
# The completion script and the probe are piped into a child bash together: no eval, no temp
# file, and it works on macOS's bash 3.2 (where `source <(...)` silently reads nothing).
bash_complete() {
    local cword="$1"
    shift
    {
        $SAVANT completions bash
        printf 'COMP_WORDS=('
        printf '%q ' "$@"
        printf ')\n'
        printf 'COMP_CWORD=%d\n' "$cword"
        printf '_savant savant "${COMP_WORDS[COMP_CWORD]}" "${COMP_WORDS[COMP_CWORD-1]}"\n'
        printf 'printf "%%s\\n" "${COMPREPLY[@]}"\n'
    } | bash
}

# ============================================================================
# GENERATION
# ============================================================================

test_start "Completions command has help"
output=$($SAVANT completions --help 2>&1)
if assert_contains "$output" "bash, elvish, fish, powershell, zsh" "Help lists the supported shells"; then
    PASSED=$((PASSED + 1))
else
    FAILED=$((FAILED + 1))
fi
test_end pass "completions --help"

for shell in bash zsh fish elvish powershell; do
    test_start "Completions generated: $shell"
    if output=$($SAVANT completions "$shell" 2>/dev/null); then
        line_count=$(printf '%s\n' "$output" | wc -l | tr -d ' ')
        if [[ "$line_count" -ge 10 ]]; then
            log_pass "$shell completions: $line_count lines"
            PASSED=$((PASSED + 1))
        else
            log_fail "$shell completions too short ($line_count lines)"
            FAILED=$((FAILED + 1))
        fi
    else
        log_fail "completions $shell exited non-zero"
        FAILED=$((FAILED + 1))
    fi
    test_end pass "completions $shell"
done

test_start "Invalid shell rejected"
set +e
output=$($SAVANT completions invalidshell 2>&1)
rc=$?
set -e
if [[ $rc -ne 0 ]] && echo "$output" | grep -qi "invalid value"; then
    log_pass "Invalid shell exits $rc with an error"
    PASSED=$((PASSED + 1))
else
    log_fail "Invalid shell not rejected (exit $rc): $output"
    FAILED=$((FAILED + 1))
fi
test_end pass "completions invalidshell"

# ============================================================================
# BASH: parses and completes
# ============================================================================

test_start "Bash completions parse"
if $SAVANT completions bash | bash -n; then
    log_pass "bash -n accepts the script"
    PASSED=$((PASSED + 1))
else
    log_fail "bash -n rejected the script"
    FAILED=$((FAILED + 1))
fi
test_end pass "bash syntax"

test_start "Bash completes a subcommand"
output=$(bash_complete 1 savant con 2>&1)
if [[ "$output" == "config" ]]; then
    log_pass "'savant con<TAB>' -> config"
    PASSED=$((PASSED + 1))
else
    log_fail "'savant con<TAB>' gave: $output"
    FAILED=$((FAILED + 1))
fi
test_end pass "bash subcommand completion"

test_start "Bash completes a nested subcommand"
output=$(bash_complete 2 savant config re 2>&1)
if [[ "$output" == "restore" ]]; then
    log_pass "'savant config re<TAB>' -> restore"
    PASSED=$((PASSED + 1))
else
    log_fail "'savant config re<TAB>' gave: $output"
    FAILED=$((FAILED + 1))
fi
test_end pass "bash nested completion"

test_start "Bash completes shell names for completions"
output=$(bash_complete 2 savant completions "" 2>&1)
missing=""
for shell in bash zsh fish; do
    echo "$output" | grep -qx "$shell" || missing="$missing $shell"
done
if [[ -z "$missing" ]]; then
    log_pass "'savant completions <TAB>' offers bash, zsh, fish"
    PASSED=$((PASSED + 1))
else
    log_fail "'savant completions <TAB>' is missing:$missing"
    FAILED=$((FAILED + 1))
fi
test_end pass "bash value completion"

# ============================================================================
# ZSH and FISH: parse (when the shell is installed)
# ============================================================================

test_start "Zsh completions parse"
output=$($SAVANT completions zsh)
if ! assert_contains "$(echo "$output" | head -1)" "#compdef savant" "Starts with #compdef savant"; then
    FAILED=$((FAILED + 1))
elif ! command -v zsh >/dev/null 2>&1; then
    log_warn "zsh not installed; header checked only"
    PASSED=$((PASSED + 1))
elif echo "$output" | zsh -n; then
    log_pass "zsh -n accepts the script"
    PASSED=$((PASSED + 1))
else
    log_fail "zsh -n rejected the script"
    FAILED=$((FAILED + 1))
fi
test_end pass "zsh syntax"

test_start "Fish completions parse"
output=$($SAVANT completions fish)
if ! echo "$output" | grep -q "^complete -c savant"; then
    log_fail "No 'complete -c savant' lines"
    FAILED=$((FAILED + 1))
elif ! command -v fish >/dev/null 2>&1; then
    log_warn "fish not installed; 'complete -c savant' lines checked only"
    PASSED=$((PASSED + 1))
elif echo "$output" | fish --no-execute; then
    log_pass "fish --no-execute accepts the script"
    PASSED=$((PASSED + 1))
else
    log_fail "fish --no-execute rejected the script"
    FAILED=$((FAILED + 1))
fi
test_end pass "fish syntax"

suite_end "$PASSED" "$FAILED"
