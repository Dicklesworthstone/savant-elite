//! E2E integration tests for CLI argument validation
//!
//! These tests verify that the CLI correctly rejects invalid inputs
//! before attempting any device operations.

use assert_cmd::Command;
use predicates::prelude::*;

/// Helper to get the savant command
fn savant() -> Command {
    Command::cargo_bin("savant").unwrap()
}

// ============================================================================
// Help and Version Tests
// ============================================================================

#[test]
fn cli_shows_help() {
    savant()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Kinesis Savant Elite"))
        .stdout(predicate::str::contains("program"))
        .stdout(predicate::str::contains("monitor"))
        .stdout(predicate::str::contains("info"));
}

#[test]
fn cli_shows_version() {
    savant()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("savant"));
}

#[test]
fn cli_shows_subcommand_help() {
    savant()
        .args(["program", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--left"))
        .stdout(predicate::str::contains("--middle"))
        .stdout(predicate::str::contains("--right"))
        .stdout(predicate::str::contains("--dry-run"));
}

// ============================================================================
// Valid Key Action Tests (dry-run mode, no device required)
// ============================================================================

#[test]
fn cli_accepts_valid_single_key() {
    // Test single key without modifiers
    savant()
        .args(["program", "--left", "a", "--dry-run"])
        .assert()
        .success();
}

#[test]
fn cli_accepts_valid_modifier_key() {
    // Test modifier+key combo
    savant()
        .args(["program", "--left", "cmd+c", "--dry-run"])
        .assert()
        .success();
}

#[test]
fn cli_accepts_valid_multiple_modifiers() {
    // Test multiple modifiers
    savant()
        .args(["program", "--left", "ctrl+shift+a", "--dry-run"])
        .assert()
        .success();
}

#[test]
fn cli_accepts_all_pedals_custom() {
    // Test all pedals with custom values
    savant()
        .args([
            "program",
            "--left",
            "cmd+z",
            "--middle",
            "cmd+shift+z",
            "--right",
            "cmd+s",
            "--dry-run",
        ])
        .assert()
        .success();
}

#[test]
fn cli_accepts_function_keys() {
    savant()
        .args(["program", "--left", "f1", "--dry-run"])
        .assert()
        .success();

    savant()
        .args(["program", "--left", "cmd+f12", "--dry-run"])
        .assert()
        .success();
}

#[test]
fn cli_rejects_modifier_only() {
    // Single modifier names like "ctrl" are not valid key actions
    // They are interpreted as keys, and there is no key called "ctrl"
    // To send a modifier, you need to combine it with a key (e.g., "ctrl+a")
    savant()
        .args(["program", "--left", "ctrl", "--dry-run"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Unknown key"));

    savant()
        .args(["program", "--left", "shift", "--dry-run"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Unknown key"));
}

// ============================================================================
// Invalid Key Action Tests
// ============================================================================

#[test]
fn cli_rejects_empty_left_pedal() {
    savant()
        .args(["program", "--left", "", "--dry-run"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Key action cannot be empty"));
}

#[test]
fn cli_rejects_empty_middle_pedal() {
    savant()
        .args(["program", "--middle", "", "--dry-run"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Key action cannot be empty"));
}

#[test]
fn cli_rejects_empty_right_pedal() {
    savant()
        .args(["program", "--right", "", "--dry-run"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Key action cannot be empty"));
}

#[test]
fn cli_rejects_whitespace_only_pedal() {
    savant()
        .args(["program", "--left", "   ", "--dry-run"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Key action cannot be empty"));
}

#[test]
fn cli_rejects_invalid_key() {
    savant()
        .args(["program", "--left", "notakey", "--dry-run"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Unknown key"));
}

#[test]
fn cli_rejects_invalid_modifier() {
    savant()
        .args(["program", "--left", "notamod+a", "--dry-run"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Unknown modifier"));
}

#[test]
fn cli_rejects_leading_plus() {
    savant()
        .args(["program", "--left", "+cmd+c", "--dry-run"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("cannot start or end with '+'"));
}

#[test]
fn cli_rejects_trailing_plus() {
    savant()
        .args(["program", "--left", "cmd+c+", "--dry-run"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("cannot start or end with '+'"));
}

#[test]
fn cli_rejects_consecutive_plus() {
    savant()
        .args(["program", "--left", "cmd++c", "--dry-run"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("consecutive '+'"));
}

#[test]
fn cli_rejects_only_plus() {
    savant()
        .args(["program", "--left", "+", "--dry-run"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("cannot start or end with '+'"));
}

#[test]
fn cli_rejects_multiple_plus_only() {
    savant()
        .args(["program", "--left", "+++", "--dry-run"])
        .assert()
        .failure();
}

// ============================================================================
// Raw Command Tests
// ============================================================================

#[test]
fn cli_raw_cmd_rejects_invalid_hex_cmd() {
    // Invalid hex characters
    savant()
        .args(["raw-cmd", "--cmd", "zz"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid command byte"));
}

#[test]
fn cli_raw_cmd_rejects_invalid_hex_data() {
    // Valid command but invalid data hex
    savant()
        .args(["raw-cmd", "--cmd", "b5", "--data", "gg"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid data bytes"));
}

#[test]
fn cli_raw_cmd_rejects_odd_length_hex() {
    // Odd number of hex characters
    savant()
        .args(["raw-cmd", "--cmd", "b5", "--data", "001"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid hex").or(predicate::str::contains("Odd")));
}

#[test]
fn cli_raw_cmd_rejects_negative_interface() {
    // Negative interface number should be rejected by clap's value_parser
    // Clap treats "-1" as an unknown flag, so it says "unexpected argument"
    savant()
        .args(["raw-cmd", "--cmd", "b5", "--interface", "-1"])
        .assert()
        .failure()
        .stderr(
            predicate::str::contains("unexpected argument")
                .or(predicate::str::contains("invalid value")),
        );
}

#[test]
fn cli_raw_cmd_rejects_interface_too_large() {
    // Interface number > 255 should be rejected
    savant()
        .args(["raw-cmd", "--cmd", "b5", "--interface", "256"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid value"));
}

#[test]
fn cli_raw_cmd_rejects_data_too_long() {
    // Data exceeding 34 bytes should be rejected
    // 35 bytes = 70 hex chars
    let long_data = "00".repeat(35);
    savant()
        .args(["raw-cmd", "--cmd", "b5", "--data", &long_data])
        .assert()
        .failure()
        .stderr(predicate::str::contains("exceeds maximum"));
}

// ============================================================================
// Monitor Command Tests
// ============================================================================

#[test]
fn cli_monitor_accepts_valid_duration() {
    // This will fail because no device, but the argument parsing should succeed
    // We check that it doesn't fail with a parsing error
    let result = savant().args(["monitor", "--duration", "10"]).assert();

    // Should fail due to device issues, not argument parsing
    let output = result.get_output();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stderr.contains("invalid value"),
        "Should accept valid duration"
    );
}

#[test]
fn cli_monitor_rejects_negative_duration() {
    // Clap treats "-1" as an unknown flag, so it says "unexpected argument"
    savant()
        .args(["monitor", "--duration", "-1"])
        .assert()
        .failure()
        .stderr(
            predicate::str::contains("unexpected argument")
                .or(predicate::str::contains("invalid value")),
        );
}

// ============================================================================
// Unknown Subcommand Tests
// ============================================================================

#[test]
fn cli_rejects_unknown_subcommand() {
    savant().arg("unknown").assert().failure().stderr(
        predicate::str::contains("unrecognized subcommand")
            .or(predicate::str::contains("invalid subcommand")),
    );
}

#[test]
fn cli_requires_subcommand() {
    savant()
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage").or(predicate::str::contains("subcommand")));
}
