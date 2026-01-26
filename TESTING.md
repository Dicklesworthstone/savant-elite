# Testing

## Automated tests

Run the full suite:

```bash
cargo test
```

This includes:

- **Unit tests** in `src/main.rs` (key parsing, config parsing, report normalization, etc.)
- **Integration tests** in `tests/cli_validation.rs` (CLI argument validation)

## Manual / E2E scripts (hardware required)

These are guided scripts intended for local, manual verification with real Savant Elite hardware:

- `tests/e2e_hardware_sim.sh`
- `tests/e2e_config.sh`

They write logs under `tests/logs/`.

See `tests/README.md` for details and caveats (Input Monitoring permissions, sudo requirements, etc.).

