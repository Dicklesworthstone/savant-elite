# Savant Elite - Bug Fixes, Tests, and Hardening

## Overview

This document tracks all identified bugs, missing tests, and hardening work needed for the savant-elite USB foot pedal programmer. Tasks are organized as "beads" with dependencies.

**Legend:**
- `[BUG]` - Code defect that needs fixing
- `[TEST]` - Missing test coverage
- `[HARDEN]` - Defensive programming improvement
- `[E2E]` - End-to-end integration test
- `[BLOCKED BY: X]` - This task depends on task X completing first

---

## Phase 1: Critical Bug Fixes (No Dependencies)

### BEAD-001: [BUG] Empty/Malformed Key Action Input Validation
**Priority:** CRITICAL
**File:** `src/main.rs:451-483` (`KeyAction::from_string`)
**Dependencies:** None

**Problem:**
- Empty string `""` produces cryptic "Unknown key: " error
- Just `"+"` produces `["", ""]` split with confusing "Unknown modifier: " error
- Trailing plus like `"shift+"` produces "Unknown key: " error
- No validation for whitespace-only input

**Fix Required:**
```rust
fn from_string(s: &str) -> Result<Self> {
    let s = s.trim();
    if s.is_empty() {
        return Err(anyhow!("Key action cannot be empty"));
    }
    if s.starts_with('+') || s.ends_with('+') {
        return Err(anyhow!("Key action cannot start or end with '+': {}", s));
    }
    // ... rest of function
}
```

**Tests Needed:** BEAD-101, BEAD-102

---

### BEAD-002: [BUG] Config File Newline Injection
**Priority:** HIGH
**File:** `src/main.rs:55-66` (`PedalConfig::save`)
**Dependencies:** None

**Problem:**
If a key action string contains newlines (shell injection like `$'cmd+c\nright=evil'`), the config file format is corrupted. The `load()` function would misparse subsequent lines.

**Fix Required:**
```rust
fn save(&self) -> Result<()> {
    // Validate no newlines in values
    for (name, val) in [("left", &self.left), ("middle", &self.middle), ("right", &self.right)] {
        if val.contains('\n') || val.contains('\r') {
            return Err(anyhow!("Key action for {} contains invalid newline character", name));
        }
    }
    // ... rest of function
}
```

**Tests Needed:** BEAD-103

---

### BEAD-003: [BUG] Auto-Monitor Infinite Wait Loop
**Priority:** HIGH
**File:** `src/main.rs:1987-1993`
**Dependencies:** None

**Problem:**
The loop waiting for device in play mode has no timeout. If user forgets to switch modes, program hangs forever. Current message says "press Ctrl+C to cancel" but no periodic reminder or timeout.

**Fix Required:**
```rust
// Add timeout (e.g., 60 seconds) with periodic reminders
let wait_start = std::time::Instant::now();
let timeout = Duration::from_secs(60);
let mut last_reminder = wait_start;

loop {
    if self.open_keyboard_interface().is_ok() {
        break;
    }

    if wait_start.elapsed() > timeout {
        self.console.print("  [bold #e74c3c]Timeout![/] Device not detected in play mode after 60s.");
        self.console.print("  Run [bold]savant monitor[/] manually after switching modes.");
        return Ok(());
    }

    // Reminder every 10 seconds
    if last_reminder.elapsed() > Duration::from_secs(10) {
        self.console.print("  [dim]Still waiting... (switch to Play mode and replug USB)[/]");
        last_reminder = std::time::Instant::now();
    }

    std::thread::sleep(Duration::from_millis(500));
}
```

**Tests Needed:** N/A (requires hardware)

---

### BEAD-004: [BUG] Raw Command Data Truncation Silent
**Priority:** MEDIUM
**File:** `src/main.rs:1339-1343`
**Dependencies:** None

**Problem:**
If `data_bytes` exceeds 34 bytes (36-2), extra bytes are silently discarded without warning.

**Fix Required:**
```rust
const MAX_RAW_DATA_LEN: usize = 34;
if data_bytes.len() > MAX_RAW_DATA_LEN {
    return Err(anyhow!(
        "Data too long: {} bytes exceeds maximum {} bytes (would be truncated)",
        data_bytes.len(),
        MAX_RAW_DATA_LEN
    ));
}
```

**Tests Needed:** BEAD-104

---

### BEAD-005: [BUG] Negative Interface Number Validation
**Priority:** LOW
**File:** `src/main.rs:439-440`
**Dependencies:** None

**Problem:**
`--interface -1` is accepted by clap (it's `i32`) but makes no sense for USB interfaces.

**Fix Required:**
Add clap validation:
```rust
#[arg(long, default_value = "0", value_parser = clap::value_parser!(i32).range(0..=255))]
interface: i32,
```

**Tests Needed:** BEAD-105

---

### BEAD-006: [BUG] Config Not Saved on Partial Success
**Priority:** MEDIUM
**File:** `src/main.rs:1909-1931`
**Dependencies:** None

**Problem:**
If EEPROM save fails but pedal programming succeeded, the config file isn't updated. User's intent is lost.

**Fix Required:**
Save config even on partial success, but include a warning in the saved file or a separate status:
```rust
// Always save what was attempted, regardless of success
let config = PedalConfig {
    left: left.to_string(),
    middle: middle.to_string(),
    right: right.to_string(),
};
if let Err(e) = config.save() {
    self.console.print(&format!(
        "  [dim]Warning: Could not save config to disk: {}[/]",
        e
    ));
}

// Then show success/warning message based on pedal_failures and save_success
```

**Tests Needed:** BEAD-106

---

### BEAD-007: [BUG] Missing Error Context for Kernel Driver Detach
**Priority:** LOW
**File:** `src/main.rs:1578-1579`
**Dependencies:** None

**Problem:**
```rust
handle.detach_kernel_driver(interface_num)?;  // No context!
```

**Fix Required:**
```rust
handle.detach_kernel_driver(interface_num)
    .context("Failed to detach kernel driver - try running with sudo")?;
```

**Tests Needed:** N/A (error path)

---

## Phase 2: Hardening (Depends on Phase 1)

### BEAD-020: [HARDEN] Add USB Disconnect Detection During Programming
**Priority:** HIGH
**File:** `src/main.rs:1636-1808`
**Dependencies:** BEAD-001, BEAD-002

**Problem:**
If device is unplugged during programming sequence, operations fail but device could be left in partially programmed state (in RAM, not EEPROM).

**Fix Required:**
1. Add pre-programming warning about keeping device connected
2. After each pedal programming, verify device still present
3. If any step fails, print clear warning about potential partial state
4. Consider adding a "verify after programming" step using GET_KEY_MACRO if supported

---

### BEAD-021: [HARDEN] Add Read-Back Verification After Programming
**Priority:** MEDIUM
**File:** `src/main.rs` (new function)
**Dependencies:** BEAD-020

**Problem:**
No verification that programming actually succeeded beyond USB ACK.

**Fix Required:**
1. After SET_KEY_MACRO, attempt GET_KEY_MACRO (0xCD) to read back value
2. Compare read-back to intended value
3. Report discrepancy if found
4. Note: May not be supported by all firmware versions - handle gracefully

---

### BEAD-022: [HARDEN] Improve HidApi Lifecycle Management
**Priority:** LOW
**File:** `src/main.rs:511-517`
**Dependencies:** None

**Problem:**
HidApi created and dropped in `new()`, then recreated in every method.

**Fix Required:**
Either:
- Store HidApi in struct: `struct SavantElite { api: HidApi, console: Console }`
- Or remove the check in `new()` and let methods handle their own errors

---

### BEAD-023: [HARDEN] Add USB Constants Module
**Priority:** LOW
**File:** `src/main.rs` (refactor)
**Dependencies:** None

**Problem:**
Magic numbers like `0x21`, `0x09`, `0x0300`, `0x0200` scattered throughout code.

**Fix Required:**
```rust
mod usb_constants {
    pub const HID_REQUEST_TYPE_CLASS_INTERFACE: u8 = 0x21;
    pub const HID_SET_REPORT: u8 = 0x09;
    pub const HID_REPORT_TYPE_OUTPUT: u16 = 0x0200;
    pub const HID_REPORT_TYPE_FEATURE: u16 = 0x0300;
}
```

---

## Phase 3: Unit Tests (Can Start in Parallel with Phase 1-2)

### BEAD-101: [TEST] KeyAction Empty String Handling
**Priority:** HIGH
**Dependencies:** BEAD-001

```rust
#[test]
fn parse_key_action_rejects_empty() {
    let err = KeyAction::from_string("").unwrap_err();
    assert!(err.to_string().contains("cannot be empty"));
}

#[test]
fn parse_key_action_rejects_whitespace_only() {
    let err = KeyAction::from_string("   ").unwrap_err();
    assert!(err.to_string().contains("cannot be empty"));
}
```

---

### BEAD-102: [TEST] KeyAction Malformed Plus Handling
**Priority:** HIGH
**Dependencies:** BEAD-001

```rust
#[test]
fn parse_key_action_rejects_leading_plus() {
    let err = KeyAction::from_string("+c").unwrap_err();
    assert!(err.to_string().contains("cannot start or end with"));
}

#[test]
fn parse_key_action_rejects_trailing_plus() {
    let err = KeyAction::from_string("cmd+").unwrap_err();
    assert!(err.to_string().contains("cannot start or end with"));
}

#[test]
fn parse_key_action_rejects_just_plus() {
    let err = KeyAction::from_string("+").unwrap_err();
    assert!(err.to_string().contains("cannot start or end with"));
}

#[test]
fn parse_key_action_handles_double_plus() {
    // "cmd++c" should fail with clear error
    let err = KeyAction::from_string("cmd++c").unwrap_err();
    assert!(err.to_string().to_lowercase().contains("unknown") ||
            err.to_string().contains("empty"));
}
```

---

### BEAD-103: [TEST] PedalConfig Newline Handling
**Priority:** HIGH
**Dependencies:** BEAD-002

```rust
#[test]
fn pedal_config_rejects_newline_in_value() {
    let config = PedalConfig {
        left: "cmd+c\nright=evil".to_string(),
        middle: "cmd+a".to_string(),
        right: "cmd+v".to_string(),
    };
    let err = config.save().unwrap_err();
    assert!(err.to_string().contains("newline"));
}

#[test]
fn pedal_config_rejects_carriage_return_in_value() {
    let config = PedalConfig {
        left: "cmd+c\rright=evil".to_string(),
        middle: "cmd+a".to_string(),
        right: "cmd+v".to_string(),
    };
    let err = config.save().unwrap_err();
    assert!(err.to_string().contains("newline"));
}
```

---

### BEAD-104: [TEST] Raw Command Data Length Validation
**Priority:** MEDIUM
**Dependencies:** BEAD-004

```rust
// Note: This test would require refactoring raw_cmd to have a testable validation function
#[test]
fn raw_cmd_validates_data_length() {
    // Extract validation logic into separate function for testing
    fn validate_raw_data(data: &[u8]) -> Result<()> {
        const MAX_RAW_DATA_LEN: usize = 34;
        if data.len() > MAX_RAW_DATA_LEN {
            return Err(anyhow!("Data too long"));
        }
        Ok(())
    }

    assert!(validate_raw_data(&[0u8; 34]).is_ok());
    assert!(validate_raw_data(&[0u8; 35]).is_err());
}
```

---

### BEAD-105: [TEST] Interface Number Validation
**Priority:** LOW
**Dependencies:** BEAD-005

```rust
// This is a clap validation, tested via CLI integration test
// See BEAD-201
```

---

### BEAD-106: [TEST] PedalConfig Save/Load Roundtrip
**Priority:** HIGH
**Dependencies:** None

```rust
#[test]
fn pedal_config_roundtrip() {
    // Use tempdir for isolation
    let temp_dir = tempfile::tempdir().unwrap();
    let config_path = temp_dir.path().join("savant-elite").join("pedals.conf");

    // Would need to refactor PedalConfig to accept custom path for testing
    // Or use test-specific environment variable for config dir
}

#[test]
fn pedal_config_load_returns_none_for_missing_file() {
    // Ensure clean state, verify load() returns None
}

#[test]
fn pedal_config_load_returns_none_for_partial_file() {
    // Create file with only "left=cmd+c\nmiddle=cmd+a\n" (missing right)
    // Verify load() returns None
}

#[test]
fn pedal_config_load_handles_extra_whitespace() {
    // "left = cmd+c \n middle=cmd+a\nright= cmd+v"
    // Verify values are trimmed correctly
}
```

---

### BEAD-107: [TEST] USB HID Key Code Coverage
**Priority:** MEDIUM
**Dependencies:** None

```rust
#[test]
fn parse_key_name_all_letters() {
    for (letter, expected) in ('a'..='z').zip(0x04u8..=0x1D) {
        let result = usb_hid::parse_key_name(&letter.to_string());
        assert_eq!(result, Some(expected), "Failed for letter '{}'", letter);
    }
}

#[test]
fn parse_key_name_all_numbers() {
    for (num, expected) in ('1'..='9').zip(0x1Eu8..=0x26) {
        let result = usb_hid::parse_key_name(&num.to_string());
        assert_eq!(result, Some(expected), "Failed for number '{}'", num);
    }
    assert_eq!(usb_hid::parse_key_name("0"), Some(0x27));
}

#[test]
fn parse_key_name_all_function_keys() {
    for (i, expected) in (1..=12).zip(0x3Au8..=0x45) {
        let result = usb_hid::parse_key_name(&format!("f{}", i));
        assert_eq!(result, Some(expected), "Failed for F{}", i);
    }
}

#[test]
fn parse_key_name_case_insensitive() {
    assert_eq!(usb_hid::parse_key_name("A"), usb_hid::parse_key_name("a"));
    assert_eq!(usb_hid::parse_key_name("ENTER"), usb_hid::parse_key_name("enter"));
    assert_eq!(usb_hid::parse_key_name("F12"), usb_hid::parse_key_name("f12"));
}
```

---

### BEAD-108: [TEST] Modifier Parsing Coverage
**Priority:** MEDIUM
**Dependencies:** None

```rust
#[test]
fn key_action_all_modifier_aliases() {
    // cmd/command/gui/meta/super all map to MOD_LEFT_GUI
    for alias in ["cmd", "command", "gui", "meta", "super"] {
        let action = KeyAction::from_string(&format!("{}+a", alias)).unwrap();
        assert_eq!(action.modifiers, usb_hid::MOD_LEFT_GUI, "Failed for '{}'", alias);
    }

    // ctrl/control
    for alias in ["ctrl", "control"] {
        let action = KeyAction::from_string(&format!("{}+a", alias)).unwrap();
        assert_eq!(action.modifiers, usb_hid::MOD_LEFT_CTRL, "Failed for '{}'", alias);
    }

    // alt/option/opt
    for alias in ["alt", "option", "opt"] {
        let action = KeyAction::from_string(&format!("{}+a", alias)).unwrap();
        assert_eq!(action.modifiers, usb_hid::MOD_LEFT_ALT, "Failed for '{}'", alias);
    }
}

#[test]
fn key_action_modifier_combinations() {
    // All four modifiers together
    let action = KeyAction::from_string("cmd+ctrl+shift+alt+a").unwrap();
    assert_eq!(action.modifiers,
        usb_hid::MOD_LEFT_GUI | usb_hid::MOD_LEFT_CTRL |
        usb_hid::MOD_LEFT_SHIFT | usb_hid::MOD_LEFT_ALT);
}
```

---

### BEAD-109: [TEST] Keyboard Report Normalization Edge Cases
**Priority:** MEDIUM
**Dependencies:** None

```rust
#[test]
fn normalize_report_too_short() {
    let data = [0u8; 7];  // Less than 8 bytes
    assert!(usb_hid::normalize_boot_keyboard_report(&data).is_none());
}

#[test]
fn normalize_report_exact_8_bytes() {
    let data = [usb_hid::MOD_LEFT_GUI, 0, usb_hid::KEY_C, 0, 0, 0, 0, 0];
    let report = usb_hid::normalize_boot_keyboard_report(&data).unwrap();
    assert_eq!(report, data);
}

#[test]
fn normalize_report_all_zeros() {
    let data = [0u8; 8];
    let report = usb_hid::normalize_boot_keyboard_report(&data).unwrap();
    assert_eq!(report, data);
}

#[test]
fn normalize_report_all_keys_pressed() {
    // Modifier + 6 simultaneous keys (max for boot protocol)
    let data = [0xFF, 0, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09];
    let report = usb_hid::normalize_boot_keyboard_report(&data).unwrap();
    assert_eq!(report, data);
}
```

---

## Phase 4: Integration/E2E Tests

### BEAD-201: [E2E] CLI Argument Validation Tests
**Priority:** HIGH
**File:** `tests/cli_tests.rs` (new file)
**Dependencies:** Phase 1 bugs fixed

Create integration tests using `assert_cmd` crate:

```rust
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn cli_rejects_empty_left_pedal() {
    Command::cargo_bin("savant")
        .unwrap()
        .args(["program", "--left", "", "--dry-run"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("cannot be empty"));
}

#[test]
fn cli_rejects_invalid_key() {
    Command::cargo_bin("savant")
        .unwrap()
        .args(["program", "--left", "cmd+notakey", "--dry-run"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Unknown key"));
}

#[test]
fn cli_accepts_valid_key_actions() {
    // Dry run should succeed without hardware
    Command::cargo_bin("savant")
        .unwrap()
        .args(["program", "--left", "cmd+c", "--middle", "cmd+a", "--right", "cmd+v", "--dry-run"])
        .assert()
        .success();
}

#[test]
fn cli_shows_help() {
    Command::cargo_bin("savant")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Kinesis Savant Elite"));
}

#[test]
fn cli_shows_version() {
    Command::cargo_bin("savant")
        .unwrap()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn cli_raw_cmd_rejects_invalid_hex() {
    Command::cargo_bin("savant")
        .unwrap()
        .args(["raw-cmd", "--cmd", "zz"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid command byte"));
}

#[test]
fn cli_raw_cmd_rejects_negative_interface() {
    Command::cargo_bin("savant")
        .unwrap()
        .args(["raw-cmd", "--cmd", "b5", "--interface", "-1"])
        .assert()
        .failure();
}
```

---

### BEAD-202: [E2E] Hardware Simulation Test Script
**Priority:** MEDIUM
**File:** `tests/e2e_hardware_sim.sh` (new file)
**Dependencies:** BEAD-201

A bash script that simulates various hardware scenarios using USB gadget mode (if available) or documents manual test procedures:

```bash
#!/usr/bin/env bash
# E2E Hardware Test Script for Savant Elite
#
# Requirements:
# - macOS with actual Kinesis Savant Elite hardware
# - Or: Linux with USB gadget mode for simulation
#
# This script documents and (where possible) automates hardware testing.

set -euo pipefail

LOG_FILE="e2e_test_$(date +%Y%m%d_%H%M%S).log"
BINARY="./target/release/savant"

log() {
    echo "[$(date +%H:%M:%S)] $*" | tee -a "$LOG_FILE"
}

test_info_no_device() {
    log "TEST: info command with no device connected"
    log "  Expected: Shows 'NO DEVICE FOUND' message"
    $BINARY info 2>&1 | tee -a "$LOG_FILE"
    log "  MANUAL CHECK: Verify output shows no device error gracefully"
}

test_info_play_mode() {
    log "TEST: info command with device in PLAY mode"
    log "  Precondition: Connect device in Play mode"
    read -p "Press Enter when device is connected in Play mode..."
    $BINARY info 2>&1 | tee -a "$LOG_FILE"
    log "  MANUAL CHECK: Verify device detected, shows PLAY mode"
}

test_info_program_mode() {
    log "TEST: info command with device in PROGRAMMING mode"
    log "  Precondition: Switch device to Program mode, replug"
    read -p "Press Enter when device is in Programming mode..."
    $BINARY info 2>&1 | tee -a "$LOG_FILE"
    log "  MANUAL CHECK: Verify device detected, shows PROGRAMMING mode"
}

test_program_dry_run() {
    log "TEST: program --dry-run"
    $BINARY program --left cmd+c --middle cmd+a --right cmd+v --dry-run 2>&1 | tee -a "$LOG_FILE"
    log "  Expected: Shows what would be programmed without hardware access"
}

test_program_actual() {
    log "TEST: Actual programming"
    log "  Precondition: Device in Programming mode"
    read -p "Press Enter when device is in Programming mode..."
    $BINARY program --left cmd+c --middle cmd+a --right cmd+v 2>&1 | tee -a "$LOG_FILE"
    log "  MANUAL CHECK: Verify success message"

    log "  Verification: Switch to Play mode, test pedals"
    read -p "Switch to Play mode, replug, then press Enter..."

    log "  Starting monitor for verification (10 seconds)..."
    timeout 10 $BINARY monitor --duration 10 2>&1 | tee -a "$LOG_FILE" || true
    log "  MANUAL CHECK: Did pedals send Cmd+C, Cmd+A, Cmd+V?"
}

test_monitor_permissions() {
    log "TEST: monitor without Input Monitoring permission"
    log "  Precondition: Revoke Input Monitoring permission for Terminal"
    read -p "Revoke permission in System Settings, then press Enter..."
    $BINARY monitor --duration 5 2>&1 | tee -a "$LOG_FILE" || true
    log "  Expected: Clear error about Input Monitoring permission"
}

test_disconnect_during_program() {
    log "TEST: Unplug during programming"
    log "  WARNING: This tests partial failure handling"
    log "  Precondition: Device in Programming mode"
    read -p "Press Enter, then QUICKLY unplug the device..."
    $BINARY program --left cmd+x --middle cmd+y --right cmd+z 2>&1 | tee -a "$LOG_FILE" || true
    log "  Expected: Error message, config should NOT be saved"
    log "  Verify: After replug, old configuration should remain"
}

# Main
log "=== Savant Elite E2E Hardware Tests ==="
log "Log file: $LOG_FILE"
log ""

test_info_no_device
test_program_dry_run

read -p "Continue with hardware-connected tests? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    test_info_play_mode
    test_info_program_mode
    test_program_actual
    # test_monitor_permissions  # Requires manual permission changes
    # test_disconnect_during_program  # Potentially dangerous
fi

log ""
log "=== Tests Complete ==="
log "Review $LOG_FILE for results"
```

---

### BEAD-203: [E2E] Config File Persistence Tests
**Priority:** HIGH
**File:** `tests/e2e_config.sh` (new file)
**Dependencies:** BEAD-106

```bash
#!/usr/bin/env bash
# Test config file handling

set -euo pipefail

CONFIG_DIR="${HOME}/.config/savant-elite"
CONFIG_FILE="${CONFIG_DIR}/pedals.conf"
BACKUP_FILE="${CONFIG_FILE}.backup"
BINARY="./target/release/savant"

log() { echo "[TEST] $*"; }

setup() {
    # Backup existing config
    if [[ -f "$CONFIG_FILE" ]]; then
        cp "$CONFIG_FILE" "$BACKUP_FILE"
        log "Backed up existing config to $BACKUP_FILE"
    fi
}

teardown() {
    # Restore backup
    if [[ -f "$BACKUP_FILE" ]]; then
        mv "$BACKUP_FILE" "$CONFIG_FILE"
        log "Restored config from backup"
    fi
}

trap teardown EXIT

test_no_config_shows_unknown() {
    log "Test: No config file shows 'unknown' configuration"
    rm -f "$CONFIG_FILE"
    $BINARY info 2>&1 | grep -q "UNKNOWN\|unknown\|Unknown" && log "PASS" || log "FAIL"
}

test_config_created_after_program() {
    log "Test: Config file created after successful program --dry-run"
    rm -f "$CONFIG_FILE"
    # Note: dry-run doesn't save config in current implementation
    # This test documents expected behavior after BEAD-006 fix
}

test_config_survives_reboot() {
    log "Test: Config persists across runs"
    # Would need actual programming or manual config creation
}

test_config_format_valid() {
    log "Test: Config file format is valid"
    if [[ -f "$CONFIG_FILE" ]]; then
        # Check for expected lines
        grep -q "^left=" "$CONFIG_FILE" && log "  left= found" || log "  FAIL: missing left="
        grep -q "^middle=" "$CONFIG_FILE" && log "  middle= found" || log "  FAIL: missing middle="
        grep -q "^right=" "$CONFIG_FILE" && log "  right= found" || log "  FAIL: missing right="
    else
        log "  SKIP: No config file exists"
    fi
}

setup
test_no_config_shows_unknown
test_config_format_valid
```

---

## Phase 5: Documentation and Test Infrastructure

### BEAD-301: [INFRA] Add Test Dependencies to Cargo.toml
**Priority:** HIGH
**Dependencies:** None

```toml
[dev-dependencies]
assert_cmd = "2"
predicates = "3"
tempfile = "3"
```

---

### BEAD-302: [INFRA] Create tests/ Directory Structure
**Priority:** HIGH
**Dependencies:** BEAD-301

```
tests/
├── cli_tests.rs          # BEAD-201
├── e2e_hardware_sim.sh   # BEAD-202
├── e2e_config.sh         # BEAD-203
└── README.md             # Documents test procedures
```

---

### BEAD-303: [INFRA] Update CI to Run Integration Tests
**Priority:** MEDIUM
**File:** `.github/workflows/ci.yml`
**Dependencies:** BEAD-301, BEAD-302

Add integration test job that runs on macOS (can't test hardware but can test CLI args).

---

### BEAD-304: [DOCS] Create TESTING.md
**Priority:** LOW
**Dependencies:** All above

Document:
- How to run unit tests
- How to run integration tests
- Hardware test procedures
- What each test covers
- Known limitations

---

## Dependency Graph

```
                    ┌─────────────────────────────────────────────────────────────┐
                    │                      PHASE 1: BUG FIXES                      │
                    │  (No dependencies - can all start immediately)               │
                    └─────────────────────────────────────────────────────────────┘
                                                 │
        ┌────────────────────────────────────────┼────────────────────────────────┐
        │                                        │                                │
        ▼                                        ▼                                ▼
   ┌─────────┐                            ┌─────────┐                      ┌─────────┐
   │ BEAD-001│ Empty input validation     │ BEAD-002│ Newline injection    │ BEAD-003│ Infinite loop
   └────┬────┘                            └────┬────┘                      └─────────┘
        │                                      │
        │                                      │
        ▼                                      ▼
   ┌─────────┐                            ┌─────────┐
   │ BEAD-101│ Test: empty input          │ BEAD-103│ Test: newlines
   │ BEAD-102│ Test: malformed plus       └─────────┘
   └─────────┘

   ┌─────────┐     ┌─────────┐     ┌─────────┐     ┌─────────┐
   │ BEAD-004│     │ BEAD-005│     │ BEAD-006│     │ BEAD-007│
   │ Truncate│     │ Negative│     │ Partial │     │ Context │
   └────┬────┘     └────┬────┘     └────┬────┘     └─────────┘
        │               │               │
        ▼               ▼               ▼
   ┌─────────┐     ┌─────────┐     ┌─────────┐
   │ BEAD-104│     │ BEAD-105│     │ BEAD-106│
   │ Test    │     │ Test    │     │ Test    │
   └─────────┘     └─────────┘     └─────────┘

                    ┌─────────────────────────────────────────────────────────────┐
                    │                      PHASE 2: HARDENING                      │
                    │  (Depends on Phase 1 core fixes)                             │
                    └─────────────────────────────────────────────────────────────┘

   BEAD-001 + BEAD-002 ──────┬──────────────────────────────────────────┐
                             │                                          │
                             ▼                                          │
                        ┌─────────┐                                     │
                        │ BEAD-020│ USB disconnect detection            │
                        └────┬────┘                                     │
                             │                                          │
                             ▼                                          │
                        ┌─────────┐                                     │
                        │ BEAD-021│ Read-back verification              │
                        └─────────┘                                     │
                                                                        │
   Independent: ┌─────────┐  ┌─────────┐                               │
                │ BEAD-022│  │ BEAD-023│                               │
                │ HidApi  │  │Constants│                               │
                └─────────┘  └─────────┘                               │
                                                                        │
                    ┌───────────────────────────────────────────────────┘
                    │          PHASE 3: UNIT TESTS
                    │  (Can run parallel with Phase 1-2, tests need fixes first)
                    └─────────────────────────────────────────────────────────────┐

   BEAD-107, BEAD-108, BEAD-109 ─── No dependencies, can start immediately

                    ┌─────────────────────────────────────────────────────────────┐
                    │                     PHASE 4: E2E TESTS                       │
                    │  (Depends on Phase 1 fixes for meaningful tests)             │
                    └─────────────────────────────────────────────────────────────┘

   Phase 1 complete ─────────┬──────────────────────────────────────────┐
                             │                                          │
                             ▼                                          ▼
                        ┌─────────┐                                ┌─────────┐
                        │ BEAD-201│ CLI integration tests          │ BEAD-203│ Config E2E
                        └────┬────┘                                └─────────┘
                             │
                             ▼
                        ┌─────────┐
                        │ BEAD-202│ Hardware simulation script
                        └─────────┘

                    ┌─────────────────────────────────────────────────────────────┐
                    │                    PHASE 5: INFRASTRUCTURE                   │
                    └─────────────────────────────────────────────────────────────┘

                        ┌─────────┐
                        │ BEAD-301│ Add test deps to Cargo.toml
                        └────┬────┘
                             │
                             ▼
                        ┌─────────┐
                        │ BEAD-302│ Create tests/ directory
                        └────┬────┘
                             │
                             ▼
                        ┌─────────┐
                        │ BEAD-303│ Update CI for integration tests
                        └────┬────┘
                             │
                             ▼
                        ┌─────────┐
                        │ BEAD-304│ TESTING.md documentation
                        └─────────┘
```

---

## Execution Order Recommendation

**Immediate (can start in parallel):**
1. BEAD-001 (empty input) - ~30 min
2. BEAD-002 (newline injection) - ~15 min
3. BEAD-003 (infinite loop) - ~30 min
4. BEAD-004 (truncation) - ~15 min
5. BEAD-005 (negative interface) - ~10 min
6. BEAD-007 (error context) - ~5 min
7. BEAD-301 (Cargo.toml deps) - ~5 min

**After BEAD-001, BEAD-002:**
8. BEAD-006 (partial success config) - ~30 min

**After bugs fixed:**
9. BEAD-101, BEAD-102, BEAD-103, BEAD-104, BEAD-105, BEAD-106 (unit tests) - ~2 hours
10. BEAD-107, BEAD-108, BEAD-109 (coverage tests) - ~1 hour

**After unit tests:**
11. BEAD-020 (disconnect detection) - ~1 hour
12. BEAD-021 (read-back verification) - ~2 hours
13. BEAD-022, BEAD-023 (cleanup) - ~1 hour

**Final:**
14. BEAD-201, BEAD-202, BEAD-203 (E2E tests) - ~3 hours
15. BEAD-302, BEAD-303, BEAD-304 (infra/docs) - ~2 hours

**Total estimated work:** ~15 hours

---

## Test Coverage Goals

| Component | Current | Target | Tests Needed |
|-----------|---------|--------|--------------|
| KeyAction::from_string | 55% | 95% | BEAD-101, BEAD-102, BEAD-108 |
| usb_hid::parse_key_name | 60% | 95% | BEAD-107 |
| usb_hid::normalize_boot_keyboard_report | 40% | 90% | BEAD-109 |
| PedalConfig | 0% | 90% | BEAD-103, BEAD-106 |
| CLI argument validation | 0% | 80% | BEAD-201 |
| Error paths | 10% | 70% | Various |
| Hardware operations | 0% | N/A* | BEAD-202 (manual) |

*Hardware operations require actual device; documented manual test procedures instead.
