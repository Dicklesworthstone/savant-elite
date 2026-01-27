//! E2E integration tests for CLI argument validation
//!
//! These tests verify that the CLI correctly rejects invalid inputs
//! before attempting any device operations.

use assert_cmd::Command;
use predicates::prelude::*;

/// Helper to get the savant command
fn savant() -> Command {
    Command::new(env!("CARGO_BIN_EXE_savant"))
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

#[test]
fn cli_rejects_whitespace_between_plus() {
    // "cmd + + c" has a whitespace-only component between the plus signs
    savant()
        .args(["program", "--left", "cmd + + c", "--dry-run"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("empty component"));
}

#[test]
fn cli_accepts_multiple_modifiers() {
    // Multiple modifiers should work
    savant()
        .args(["program", "--left", "ctrl+shift+alt+cmd+a", "--dry-run"])
        .assert()
        .success();
}

#[test]
fn cli_accepts_modifier_aliases() {
    // Various modifier aliases should all work
    savant()
        .args(["program", "--left", "command+a", "--dry-run"])
        .assert()
        .success();

    savant()
        .args(["program", "--left", "control+a", "--dry-run"])
        .assert()
        .success();

    savant()
        .args(["program", "--left", "option+a", "--dry-run"])
        .assert()
        .success();

    savant()
        .args(["program", "--left", "gui+a", "--dry-run"])
        .assert()
        .success();
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

// ============================================================================
// Keys Command Tests
// ============================================================================

#[test]
fn cli_keys_shows_modifiers() {
    savant()
        .arg("keys")
        .assert()
        .success()
        .stdout(predicate::str::contains("MODIFIERS"))
        .stdout(predicate::str::contains("cmd"))
        .stdout(predicate::str::contains("ctrl"))
        .stdout(predicate::str::contains("shift"))
        .stdout(predicate::str::contains("alt"));
}

#[test]
fn cli_keys_shows_all_categories() {
    savant()
        .arg("keys")
        .assert()
        .success()
        .stdout(predicate::str::contains("LETTERS"))
        .stdout(predicate::str::contains("NUMBERS"))
        .stdout(predicate::str::contains("FUNCTION KEYS"))
        .stdout(predicate::str::contains("SPECIAL KEYS"))
        .stdout(predicate::str::contains("ARROW KEYS"))
        .stdout(predicate::str::contains("PUNCTUATION"))
        .stdout(predicate::str::contains("EXAMPLES"));
}

#[test]
fn cli_keys_json_is_valid() {
    let output = savant()
        .args(["keys", "--json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    // Verify it's valid JSON by parsing it
    let json: serde_json::Value =
        serde_json::from_slice(&output).expect("keys --json should produce valid JSON");

    // Verify structure
    assert!(
        json.get("modifiers").is_some(),
        "JSON should have modifiers"
    );
    assert!(json.get("keys").is_some(), "JSON should have keys");
}

#[test]
fn cli_keys_help() {
    savant()
        .args(["keys", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--json"))
        .stdout(predicate::str::contains("List all valid key names"));
}

// ============================================================================
// Completions Command Tests
// ============================================================================

#[test]
fn cli_completions_zsh() {
    savant()
        .args(["completions", "zsh"])
        .assert()
        .success()
        .stdout(predicate::str::contains("#compdef savant"))
        .stdout(predicate::str::contains("_savant"));
}

#[test]
fn cli_completions_bash() {
    savant()
        .args(["completions", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains("_savant()"))
        .stdout(predicate::str::contains("COMPREPLY"));
}

#[test]
fn cli_completions_fish() {
    savant()
        .args(["completions", "fish"])
        .assert()
        .success()
        .stdout(predicate::str::contains("__fish_savant"))
        .stdout(predicate::str::contains("complete -c savant"));
}

#[test]
fn cli_completions_help() {
    savant()
        .args(["completions", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Generate shell completion scripts",
        ))
        .stdout(predicate::str::contains("bash"))
        .stdout(predicate::str::contains("zsh"))
        .stdout(predicate::str::contains("fish"));
}

#[test]
fn cli_completions_rejects_invalid_shell() {
    savant()
        .args(["completions", "invalid"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid value"));
}

// ============================================================================
// Verbose Mode Tests
// ============================================================================

#[test]
fn cli_verbose_flag_accepted() {
    // -v flag should be accepted on any command
    savant().args(["-v", "keys"]).assert().success();
}

#[test]
fn cli_verbose_long_flag_accepted() {
    // --verbose flag should be accepted on any command
    savant().args(["--verbose", "keys"]).assert().success();
}

#[test]
fn cli_verbose_output_goes_to_stderr() {
    // Verbose output should go to stderr, not stdout
    savant()
        .args(["-v", "keys", "--json"])
        .assert()
        .success()
        .stderr(predicate::str::contains("[verbose]"))
        .stdout(predicate::str::contains("[verbose]").not());
}

#[test]
fn cli_verbose_shows_mode_enabled() {
    savant()
        .args(["-v", "keys"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Verbose mode enabled"));
}

#[test]
fn cli_verbose_with_dry_run() {
    // Verbose mode should work with dry-run
    savant()
        .args(["-v", "program", "--left", "a", "--dry-run"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Parsing left pedal action"));
}

// ============================================================================
// JSON Output Tests
// ============================================================================

#[test]
fn cli_json_flag_accepted() {
    // --json flag should be accepted on info command
    // This will fail because no device, but we check JSON output structure
    let result = savant().args(["--json", "info"]).assert();
    let output = result.get_output();
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should output valid JSON (even if device not found)
    let json: serde_json::Value =
        serde_json::from_str(&stdout).expect("info --json should produce valid JSON");
    assert!(json.get("device").is_some(), "JSON should have device field");
}

#[test]
fn cli_json_info_has_correct_structure() {
    let output = savant()
        .args(["--json", "info"])
        .assert()
        .get_output()
        .stdout
        .clone();

    let json: serde_json::Value =
        serde_json::from_slice(&output).expect("info --json should produce valid JSON");

    // Check device structure
    let device = json.get("device").expect("should have device field");
    assert!(
        device.get("detected").is_some(),
        "device should have detected field"
    );
    assert!(device.get("vid").is_some(), "device should have vid field");
    assert!(
        device.get("interfaces").is_some(),
        "device should have interfaces field"
    );
}

#[test]
fn cli_json_status_produces_valid_json() {
    let result = savant().args(["--json", "status"]).assert();
    let output = result.get_output();
    let stdout = String::from_utf8_lossy(&output.stdout);

    let json: serde_json::Value =
        serde_json::from_str(&stdout).expect("status --json should produce valid JSON");
    assert!(
        json.get("detected").is_some(),
        "JSON should have detected field"
    );
    assert!(
        json.get("ready_to_program").is_some(),
        "JSON should have ready_to_program field"
    );
    assert!(
        json.get("devices").is_some(),
        "JSON should have devices field"
    );
}

#[test]
fn cli_json_output_goes_to_stdout() {
    // JSON output should go to stdout, not stderr
    savant()
        .args(["--json", "info"])
        .assert()
        .stdout(predicate::str::contains("\"device\""))
        .stderr(predicate::str::contains("\"device\"").not());
}

#[test]
fn cli_json_with_verbose() {
    // JSON and verbose should work together
    savant()
        .args(["--json", "-v", "info"])
        .assert()
        .stdout(predicate::str::contains("\"device\""))
        .stderr(predicate::str::contains("[verbose]"));
}

// ============================================================================
// Preset Command Tests
// ============================================================================

#[test]
fn cli_preset_list_shows_all_presets() {
    savant()
        .args(["preset", "--list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("AVAILABLE PRESETS"))
        .stdout(predicate::str::contains("copy-paste"))
        .stdout(predicate::str::contains("undo-redo"))
        .stdout(predicate::str::contains("browser"))
        .stdout(predicate::str::contains("zoom"));
}

#[test]
fn cli_preset_list_json_is_valid() {
    let output = savant()
        .args(["--json", "preset", "--list"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let json: serde_json::Value =
        serde_json::from_slice(&output).expect("preset --list --json should produce valid JSON");

    // Verify structure
    let presets = json.get("presets").expect("JSON should have presets field");
    assert!(presets.is_array(), "presets should be an array");

    let presets_arr = presets.as_array().unwrap();
    assert!(presets_arr.len() >= 4, "should have at least 4 presets");

    // Verify first preset has required fields
    let first = &presets_arr[0];
    assert!(first.get("name").is_some(), "preset should have name");
    assert!(
        first.get("description").is_some(),
        "preset should have description"
    );
    assert!(first.get("left").is_some(), "preset should have left");
    assert!(first.get("middle").is_some(), "preset should have middle");
    assert!(first.get("right").is_some(), "preset should have right");
}

#[test]
fn cli_preset_show_displays_details() {
    savant()
        .args(["preset", "copy-paste", "--show"])
        .assert()
        .success()
        .stdout(predicate::str::contains("PRESET: COPY-PASTE"))
        // Visualization shows formatted keys (⌘C instead of cmd+c)
        .stdout(predicate::str::contains("To apply: savant preset copy-paste"));
}

#[test]
fn cli_preset_show_json_is_valid() {
    let output = savant()
        .args(["--json", "preset", "copy-paste", "--show"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let json: serde_json::Value =
        serde_json::from_slice(&output).expect("preset --show --json should produce valid JSON");

    assert_eq!(json.get("name").unwrap(), "copy-paste");
    assert_eq!(json.get("left").unwrap(), "cmd+c");
    assert_eq!(json.get("middle").unwrap(), "cmd+a");
    assert_eq!(json.get("right").unwrap(), "cmd+v");
}

#[test]
fn cli_preset_rejects_unknown_name() {
    savant()
        .args(["preset", "invalid-preset-name"])
        .assert()
        .failure()
        .stdout(predicate::str::contains("Unknown preset"))
        .stdout(predicate::str::contains("Available presets"));
}

#[test]
fn cli_preset_missing_name_shows_usage() {
    savant()
        .args(["preset"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Missing preset name"))
        .stdout(predicate::str::contains("savant preset --list"));
}

#[test]
fn cli_preset_help() {
    savant()
        .args(["preset", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--list"))
        .stdout(predicate::str::contains("--show"))
        .stdout(predicate::str::contains("--dry-run"))
        .stdout(predicate::str::contains("preset configuration"));
}

#[test]
fn cli_preset_dry_run_works() {
    // --dry-run should show configuration without error (device mode check happens later)
    savant()
        .args(["preset", "browser", "--dry-run"])
        .assert()
        .success()
        // Visualization shows formatted keys - just verify it shows the pedal visualization
        .stdout(predicate::str::contains("YOUR PEDAL CONFIGURATION"));
}
