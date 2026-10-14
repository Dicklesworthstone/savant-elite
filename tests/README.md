# Tests

None of the automated tests needs a pedal. Each test that writes configuration runs with its own
temporary `HOME` (and `XDG_CONFIG_HOME`), so the real `~/Library/Application Support/savant-elite`
is never touched.

## Quick start

```bash
cargo test                                   # unit + CLI integration tests
./tests/e2e/run_all.sh --skip-hardware       # E2E shell suites against target/{release,debug}/savant
```

## Layers

| Layer | Where | What it covers |
|-------|-------|----------------|
| Unit | `src/main.rs` (`#[cfg(test)]`) | key/modifier parsing, `usb_hid` helpers, `PedalConfig` parse/serialize, profile names and paths, presets |
| CLI integration | `tests/cli_validation.rs` | argument validation, error messages, `--json` output, help/version |
| Config workflow | `tests/config_workflow.rs` | profile save/list/show/delete, automatic backups, `config history`/`restore`, `SAVANT_HISTORY_SIZE` pruning |
| E2E suites | `tests/e2e/test_*.sh` | help/version, keys, presets, config and profiles, doctor, verbose mode, shell completions |
| Manual hardware | `tests/e2e_hardware_sim.sh`, `tests/e2e_config.sh` | guided `status`/`info`/`program`/`monitor` on a real pedal |

Config fixtures for `savant config check` live in `tests/fixtures/` (see its README).

## Running specific tests

```bash
cargo test validate_profile_name             # tests whose name matches
cargo test --test cli_validation             # one integration file
cargo test --test config_workflow
bash tests/e2e/test_completions.sh           # one E2E suite
```

## E2E suites

`run_all.sh` runs every `tests/e2e/test_*.sh` suite and exits non-zero if any fails. Each suite
also runs on its own. Options and environment:

| Variable / flag | Meaning |
|-----------------|---------|
| `SAVANT` | Binary under test. Default: `target/release/savant`, then `target/debug/savant`, then `savant` on `PATH` |
| `--skip-hardware` | Skip hardware suites even when a pedal is connected |
| `--verbose` / `LOG_LEVEL=DEBUG` | Debug logging |
| `LOG_FILE` | Log path (default `/tmp/savant-e2e-<timestamp>.log`) |
| `NO_COLOR` | Plain output |

The suites need `jq`. The completion suite syntax-checks zsh and fish output with `zsh -n` and
`fish --no-execute` when those shells are installed, and checks only the script header otherwise.

## Manual hardware checks

These need a Savant Elite and are never run automatically:

- `tests/e2e_hardware_sim.sh`: guided `status`, `info`, `program` and `monitor`, logged to
  `tests/logs/` (`BINARY=` picks the binary, default `target/release/savant`).
- `tests/e2e_config.sh`: config-file checks; it backs up and restores your `pedals.conf`.

`savant monitor` needs macOS **Input Monitoring** permission for your terminal, and `savant program`
may need `sudo` for USB access.

## Running in CI

Any macOS runner (or a local pre-push hook) can run the same commands. The full gate:

```bash
cargo fmt --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked --no-fail-fast
cargo build --locked --release
SAVANT=target/release/savant NO_COLOR=1 ./tests/e2e/run_all.sh --skip-hardware
```
