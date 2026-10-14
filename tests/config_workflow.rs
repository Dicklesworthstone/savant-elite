//! Config profile workflow and automatic-backup integration tests.
//!
//! Every command runs with its own HOME (and XDG_CONFIG_HOME), so the tests never touch the
//! real configuration and can run in parallel. None of them needs a pedal: `config restore`
//! without `--apply` only rewrites pedals.conf, which is what takes the automatic backup.

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

const CURRENT: &str = "left=cmd+c\nmiddle=cmd+a\nright=cmd+v\n";

fn savant(home: &Path) -> Command {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_savant"));
    cmd.env("HOME", home)
        .env("XDG_CONFIG_HOME", home.join(".config"))
        .env("NO_COLOR", "1")
        .env_remove("SAVANT_HISTORY_SIZE");
    cmd
}

/// Where `dirs::config_dir()` resolves under the isolated HOME.
fn config_dir(home: &Path) -> PathBuf {
    if cfg!(target_os = "macos") {
        home.join("Library/Application Support/savant-elite")
    } else {
        home.join(".config/savant-elite")
    }
}

fn home_with_current_config() -> TempDir {
    let home = TempDir::new().expect("temp HOME");
    let dir = config_dir(home.path());
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("pedals.conf"), CURRENT).unwrap();
    home
}

fn history_entries(home: &Path) -> Vec<String> {
    let mut names: Vec<String> = fs::read_dir(config_dir(home).join("history"))
        .map(|entries| {
            entries
                .filter_map(|e| e.ok())
                .map(|e| e.file_name().to_string_lossy().into_owned())
                .collect()
        })
        .unwrap_or_default();
    names.sort();
    names
}

/// `YYYY-MM-DD_HHMMSS.conf`, the name `config history` and `restore` parse back.
fn is_backup_name(name: &str) -> bool {
    name.strip_suffix(".conf")
        .is_some_and(|stem| chrono::NaiveDateTime::parse_from_str(stem, "%Y-%m-%d_%H%M%S").is_ok())
}

// ============================================================================
// Profile workflow (savant_elite_rust-899)
// ============================================================================

#[test]
fn config_save_list_show_delete_workflow() {
    let home = home_with_current_config();
    let profile = config_dir(home.path()).join("profiles/testprofile.conf");

    savant(home.path())
        .args(["config", "save", "testprofile"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Saved profile 'testprofile'"));
    assert_eq!(fs::read_to_string(&profile).unwrap(), CURRENT);

    savant(home.path())
        .args(["config", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("testprofile"));

    savant(home.path())
        .args(["config", "show", "testprofile"])
        .assert()
        .success()
        .stdout(predicate::str::contains("PROFILE: TESTPROFILE"))
        .stdout(predicate::str::contains("⌘C"))
        .stdout(predicate::str::contains("savant config load testprofile"));

    savant(home.path())
        .args(["config", "delete", "testprofile", "--force"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Deleted profile 'testprofile'"));
    assert!(!profile.exists());

    savant(home.path())
        .args(["config", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("testprofile").not());
}

#[test]
fn config_save_fails_without_a_current_config() {
    let home = TempDir::new().unwrap();
    savant(home.path())
        .args(["config", "save", "testprofile"])
        .assert()
        .failure();
    assert!(!config_dir(home.path())
        .join("profiles/testprofile.conf")
        .exists());
}

#[test]
fn config_save_refuses_to_overwrite_without_force() {
    let home = home_with_current_config();
    let profile = config_dir(home.path()).join("profiles/testprofile.conf");
    savant(home.path())
        .args(["config", "save", "testprofile"])
        .assert()
        .success();
    fs::write(
        config_dir(home.path()).join("pedals.conf"),
        "left=f1\nmiddle=f2\nright=f3\n",
    )
    .unwrap();

    savant(home.path())
        .args(["config", "save", "testprofile"])
        .assert()
        .failure();
    assert_eq!(fs::read_to_string(&profile).unwrap(), CURRENT);

    savant(home.path())
        .args(["config", "save", "testprofile", "--force"])
        .assert()
        .success();
    assert!(fs::read_to_string(&profile).unwrap().contains("left=f1"));
}

// ============================================================================
// Automatic backups (savant_elite_rust-r2n)
// ============================================================================

#[test]
fn restore_backs_up_the_current_config_before_changing_it() {
    let home = home_with_current_config();
    let history = config_dir(home.path()).join("history");
    fs::create_dir_all(&history).unwrap();
    fs::write(
        history.join("2020-01-01_000000.conf"),
        "left=cmd+z\nmiddle=cmd+a\nright=cmd+y\n",
    )
    .unwrap();

    savant(home.path())
        .args(["config", "restore", "1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Config restored"));

    // The restored backup is now the current config ...
    let current = fs::read_to_string(config_dir(home.path()).join("pedals.conf")).unwrap();
    assert_eq!(current, "left=cmd+z\nmiddle=cmd+a\nright=cmd+y\n");
    // ... and the config it replaced was backed up first, under a timestamped name.
    let entries = history_entries(home.path());
    assert_eq!(entries.len(), 2, "{entries:?}");
    let new_backup = entries
        .iter()
        .find(|name| name.as_str() != "2020-01-01_000000.conf")
        .unwrap();
    assert!(is_backup_name(new_backup), "{new_backup}");
    assert_eq!(
        fs::read_to_string(history.join(new_backup)).unwrap(),
        CURRENT
    );
}

#[test]
fn history_lists_backups_newest_first_and_restore_counts_from_newest() {
    let home = home_with_current_config();
    let history = config_dir(home.path()).join("history");
    fs::create_dir_all(&history).unwrap();
    fs::write(
        history.join("2020-01-01_000000.conf"),
        "left=f1\nmiddle=f1\nright=f1\n",
    )
    .unwrap();
    fs::write(
        history.join("2021-01-01_000000.conf"),
        "left=f2\nmiddle=f2\nright=f2\n",
    )
    .unwrap();

    let output = savant(home.path())
        .args(["--json", "config", "history"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let json: serde_json::Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(json["count"], 2);
    assert_eq!(json["history"][0]["timestamp"], "2021-01-01 00:00:00");
    assert_eq!(json["history"][0]["number"], 1);
    assert_eq!(json["history"][1]["left"], "f1");

    savant(home.path())
        .args(["config", "restore", "2"])
        .assert()
        .success();
    let current = fs::read_to_string(config_dir(home.path()).join("pedals.conf")).unwrap();
    assert_eq!(current, "left=f1\nmiddle=f1\nright=f1\n");
}

#[test]
fn backups_are_pruned_to_savant_history_size() {
    let home = home_with_current_config();
    let history = config_dir(home.path()).join("history");
    fs::create_dir_all(&history).unwrap();
    for year in 2019..=2021 {
        fs::write(
            history.join(format!("{year}-01-01_000000.conf")),
            "left=f1\nmiddle=f2\nright=f3\n",
        )
        .unwrap();
    }

    savant(home.path())
        .env("SAVANT_HISTORY_SIZE", "2")
        .args(["config", "restore", "1"])
        .assert()
        .success();

    // Four backups existed after the restore's own backup; the two oldest are gone.
    let entries = history_entries(home.path());
    assert_eq!(entries.len(), 2, "{entries:?}");
    assert!(!entries.contains(&"2019-01-01_000000.conf".to_string()));
    assert!(!entries.contains(&"2020-01-01_000000.conf".to_string()));
    assert!(entries.contains(&"2021-01-01_000000.conf".to_string()));
}

#[test]
fn restore_with_no_history_fails_cleanly() {
    let home = home_with_current_config();
    savant(home.path())
        .args(["config", "restore", "1"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("No configuration history"));
    assert_eq!(
        fs::read_to_string(config_dir(home.path()).join("pedals.conf")).unwrap(),
        CURRENT
    );
}
