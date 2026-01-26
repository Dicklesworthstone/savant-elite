#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

LOG_DIR="${LOG_DIR:-$ROOT/tests/logs}"
mkdir -p "$LOG_DIR"
LOG_FILE="$LOG_DIR/e2e_config_$(date +%Y%m%d_%H%M%S).log"

BINARY="${BINARY:-$ROOT/target/release/savant}"

if [[ "${OSTYPE:-}" == darwin* ]]; then
  CONFIG_BASE="${HOME}/Library/Application Support"
else
  CONFIG_BASE="${XDG_CONFIG_HOME:-$HOME/.config}"
fi

CONFIG_DIR="${CONFIG_BASE}/savant-elite"
CONFIG_FILE="${CONFIG_DIR}/pedals.conf"
BACKUP_FILE="${LOG_DIR}/pedals.conf.backup.$(date +%Y%m%d_%H%M%S)"

log() {
  printf '[%s] %s\n' "$(date +%H:%M:%S)" "$*" | tee -a "$LOG_FILE"
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
  log "Release binary not found at: $BINARY"
  log "Building with: cargo build --release"
  cargo build --release
}

restore_backup() {
  if [[ -f "$BACKUP_FILE" ]]; then
    mkdir -p "$CONFIG_DIR"
    mv "$BACKUP_FILE" "$CONFIG_FILE"
    log "Restored previous config to: $CONFIG_FILE"
  fi
}

cleanup() {
  restore_backup || true
}
trap cleanup EXIT

section() {
  log ""
  log "=== $* ==="
}

section "Savant Elite config-file E2E (Manual)"
ensure_binary
log "Binary:     $BINARY"
log "Log file:   $LOG_FILE"
log "Config dir: $CONFIG_DIR"
log "Config file:$CONFIG_FILE"

section "Backup"
if [[ -f "$CONFIG_FILE" ]]; then
  cp "$CONFIG_FILE" "$BACKUP_FILE"
  log "Backed up existing config to: $BACKUP_FILE"
else
  log "No existing config file found; no backup needed."
fi

section "Write a known-good config"
mkdir -p "$CONFIG_DIR"
cat >"$CONFIG_FILE" <<'EOF'
left=cmd+c
middle=cmd+a
right=cmd+v
EOF
log "Wrote config:"
cat "$CONFIG_FILE" | tee -a "$LOG_FILE"

section "Format checks"
if grep -q '^left=' "$CONFIG_FILE"; then
  log "PASS: left="
else
  log "FAIL: missing left="
fi
if grep -q '^middle=' "$CONFIG_FILE"; then
  log "PASS: middle="
else
  log "FAIL: missing middle="
fi
if grep -q '^right=' "$CONFIG_FILE"; then
  log "PASS: right="
else
  log "FAIL: missing right="
fi

section "Optional manual verification (requires hardware)"
log "With device connected (any mode), run:"
log "  $BINARY info"
log "Expected: pedal visualization reflects cmd+c / cmd+a / cmd+v when device is detected."

section "Done"
log "Restoring your original config (if any) on exit."
