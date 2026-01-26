# Tests

This repo has two automated test layers:

- **Unit tests** live in `src/main.rs` (run with `cargo test`).
- **CLI integration tests** live in `tests/cli_validation.rs` (also run with `cargo test`).

## Manual / E2E (requires hardware)

These scripts are for **manual** verification with real Savant Elite hardware and are intentionally **not** run in CI:

- `tests/e2e_hardware_sim.sh` – guided flow through `status`, `info`, `program`, and `monitor` with logging
- `tests/e2e_config.sh` – safe config-file checks with backup/restore and optional manual verification via `savant info`

### Notes

- `savant monitor` requires macOS **Input Monitoring** permission for your terminal.
- `savant program` may require `sudo` on macOS for USB access.

