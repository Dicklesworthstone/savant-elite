#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

LOG_DIR="${LOG_DIR:-$ROOT/tests/logs}"
mkdir -p "$LOG_DIR"
LOG_FILE="$LOG_DIR/e2e_hardware_$(date +%Y%m%d_%H%M%S).log"

BINARY="${BINARY:-$ROOT/target/release/savant}"

log() {
  printf '[%s] %s\n' "$(date +%H:%M:%S)" "$*" | tee -a "$LOG_FILE"
}

section() {
  log ""
  log "=== $* ==="
}

prompt() {
  read -r -p "$*"
}

run_cmd() {
  local rc=0
  log "\n$ $*"
  set +e
  "$@" 2>&1 | tee -a "$LOG_FILE"
  rc=${PIPESTATUS[0]}
  set -e
  if [[ $rc -ne 0 ]]; then
    log "(exit $rc)"
  fi
  return 0
}

ensure_binary() {
  if [[ -x "$BINARY" ]]; then
    return 0
  fi
  section "Build"
  log "Release binary not found at: $BINARY"
  log "Building with: cargo build --release"
  cargo build --release
  if [[ ! -x "$BINARY" ]]; then
    log "ERROR: expected binary at $BINARY after build"
    exit 1
  fi
}

section "Savant Elite E2E (Manual) — Setup"
ensure_binary
log "Binary:   $BINARY"
log "Log file: $LOG_FILE"

section "Baseline (no hardware required)"
run_cmd "$BINARY" status
run_cmd "$BINARY" program --dry-run

section "Hardware-connected tests (requires Savant Elite)"
prompt "Connect device in PLAY mode, then press Enter to continue..."
run_cmd "$BINARY" status
run_cmd "$BINARY" info

prompt "Flip device switch to PROGRAM mode, replug USB, then press Enter..."
run_cmd "$BINARY" status

log ""
log "NOTE: 'savant program' may require sudo on macOS."
prompt "Ready to run a REAL programming test? (Press Enter to continue, Ctrl+C to skip)..."
run_cmd "$BINARY" program --left cmd+c --middle cmd+a --right cmd+v

prompt "Flip device back to PLAY mode, replug USB, then press Enter..."
run_cmd "$BINARY" status

log ""
log "NOTE: 'savant monitor' requires Input Monitoring permission for your terminal."
run_cmd "$BINARY" monitor --duration 10

section "Done"
log "Review the log at: $LOG_FILE"

