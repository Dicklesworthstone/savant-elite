# Changelog

All notable changes to [savant-elite](https://github.com/Dicklesworthstone/savant-elite) are documented in this file.

Format: entries are grouped by release (tagged versions with GitHub Releases) and by capability area. Commit links point to the canonical repository. Unreleased work tracks changes on `main` since the latest tag.

---

## [Unreleased] (since v0.1.3)

> Commits on `main` after [v0.1.3](https://github.com/Dicklesworthstone/savant-elite/releases/tag/v0.1.3). Not yet part of a GitHub Release.

### Documentation

- Add AGENTS.md with cass (Cross-Agent Session Search) tool reference ([c06413d](https://github.com/Dicklesworthstone/savant-elite/commit/c06413d5a6879aa9e6f073d0472b4b9e3cde96ad))

---

## [v0.1.3](https://github.com/Dicklesworthstone/savant-elite/releases/tag/v0.1.3) -- 2026-02-22

Tagged at [`c414b94`](https://github.com/Dicklesworthstone/savant-elite/commit/c414b942fad1ce116aa2500bb3a418b1f9d18e6f). [Full diff from v0.1.2](https://github.com/Dicklesworthstone/savant-elite/compare/v0.1.2...v0.1.3).

This is the largest release so far, adding a full UX command suite (presets, profiles, doctor, keys, config management), shell completions, JSON output, verbose mode, a comprehensive test harness, and many robustness fixes.

### New Commands

- **`savant preset`** -- Built-in pedal configuration presets for common workflows ([7e124d5](https://github.com/Dicklesworthstone/savant-elite/commit/7e124d5cdda941731c0c4356db36f343f7996c5e))
- **`savant doctor`** -- System diagnostics command: checks USB connectivity, permissions, device state ([302f63f](https://github.com/Dicklesworthstone/savant-elite/commit/302f63f42f9180f7a95a9d392576cecb62d7831f))
- **`savant keys`** -- List all valid key names and their HID codes ([616f366](https://github.com/Dicklesworthstone/savant-elite/commit/616f366a915e0c8ea006ee55fe2645bcfe682117))
- **`savant config check`** -- Validate config file syntax and required fields ([fc71cdf](https://github.com/Dicklesworthstone/savant-elite/commit/fc71cdfb73fdf7fca2d5fc3000d21aa91f01aa64))
- **`savant config history` / `savant config restore`** -- Automatic config backup with history browsing and point-in-time restore ([4de1dbd](https://github.com/Dicklesworthstone/savant-elite/commit/4de1dbda249a401461cdfbd3b5e3150dc8d6277f))
- **Config profiles** -- Save and load named pedal configurations (`savant config save-profile`, `savant config load-profile`) ([9276314](https://github.com/Dicklesworthstone/savant-elite/commit/9276314b734a0955e599b924690bb0e39527fad7))

### New CLI Flags

- `--json` flag for machine-readable output across all commands ([9d1ec6b](https://github.com/Dicklesworthstone/savant-elite/commit/9d1ec6b96c956e73aa6239c6a2f1035380b5f862))
- `-v` / `--verbose` flag for debug-level troubleshooting output ([27dbecd](https://github.com/Dicklesworthstone/savant-elite/commit/27dbecd45f8e18017e23da20a9f4ff90e9fee68b))
- `--timeout` flag for configurable USB operation timeout ([c031daf](https://github.com/Dicklesworthstone/savant-elite/commit/c031daf9a19ff815bbc7c793f1fcf65c11402c4b))

### Shell Completions

- Add completion scripts for zsh, bash, and fish ([93a0955](https://github.com/Dicklesworthstone/savant-elite/commit/93a095542482a20915ff4d767102ebb119460e86))

### Installer

- Add polished `install.sh` with ASCII art, progress output, and checksum verification ([69cbc9a](https://github.com/Dicklesworthstone/savant-elite/commit/69cbc9af415c385ea11b16d53fcb5fe095534b45))
- Update README with one-liner `curl | bash` installer ([db6d744](https://github.com/Dicklesworthstone/savant-elite/commit/db6d744240bf8190b9141f8e172db2158202b5d3))
- Fix ASCII art alignment in installer ([e71ce2a](https://github.com/Dicklesworthstone/savant-elite/commit/e71ce2a1cbafe0b8733a31fdd42d399b7cb36954))
- Accept v-prefixed `SAVANT_VERSION` environment variable ([5e8b7c0](https://github.com/Dicklesworthstone/savant-elite/commit/5e8b7c0645463a41ad98648c67a1cded1262bdb6))
- Fix programming mode instructions in install script ([d98e692](https://github.com/Dicklesworthstone/savant-elite/commit/d98e6926e33723e201521b713b2d820fda33dadf))

### UI Improvements

- Show pedal config in `savant info` command, fix box alignments ([68d0275](https://github.com/Dicklesworthstone/savant-elite/commit/68d0275e657dc35ee77c51f907f342bca8565d64))
- Fix box alignment issues in banner and status messages ([66291e6](https://github.com/Dicklesworthstone/savant-elite/commit/66291e6c2252e08c38cd26cc6e7b22e9a35e37f9))
- Correct box border alignment for emoji/symbol content ([e73b6ce](https://github.com/Dicklesworthstone/savant-elite/commit/e73b6ce76e1efd41ce03ce9cb8d8befc16ca2eb9))
- Improve UI messaging for programming mode ([0cc6839](https://github.com/Dicklesworthstone/savant-elite/commit/0cc683953eaabf944bd2941b15aca8c42c108503))
- Improve EEPROM save disconnect message clarity ([0629d6e](https://github.com/Dicklesworthstone/savant-elite/commit/0629d6e678431e17b8253bf3e331ca244146c4d7))

### Bug Fixes

- Fix potential out-of-bounds read in `verify_pedal_programming` ([ecc86e1](https://github.com/Dicklesworthstone/savant-elite/commit/ecc86e1f901f99686c8031087724c17b78a12f1d))
- Correct `verify_pedal_programming` format check ([d34e4c7](https://github.com/Dicklesworthstone/savant-elite/commit/d34e4c72b6bd48a29713deb24a3cdf6b2533f216))
- Use `saturating_sub` to prevent panic on long preset names ([2e4e47a](https://github.com/Dicklesworthstone/savant-elite/commit/2e4e47a3bd1e6c19bbcd6dd18c586049383d2bae))
- Use configurable timeout for `read_control` in verification ([4de617a](https://github.com/Dicklesworthstone/savant-elite/commit/4de617acb7f3ce2f061d5eba26cdb2f5ce858045))
- Improve config check error display for missing fields ([a0193a6](https://github.com/Dicklesworthstone/savant-elite/commit/a0193a6557aa77ae910f71d57696c8795a6a9634))
- Improve config restore error handling and JSON output ([4ada402](https://github.com/Dicklesworthstone/savant-elite/commit/4ada4025c73cbbc0f756a452288e2ace52ea338a))
- Trim config values, remove dead code ([ba10f60](https://github.com/Dicklesworthstone/savant-elite/commit/ba10f607e2f7e2837c04f52e88c1d2690c39840c))
- Make integration tests platform-independent ([c414b94](https://github.com/Dicklesworthstone/savant-elite/commit/c414b942fad1ce116aa2500bb3a418b1f9d18e6f))

### Testing

- Add comprehensive input validation and E2E tests ([e24ced9](https://github.com/Dicklesworthstone/savant-elite/commit/e24ced9ab961ab6fa345393d661669f37fcdb9fc))
- Add edge case E2E tests for input validation ([474148b](https://github.com/Dicklesworthstone/savant-elite/commit/474148bea305c2fafc05c80d4710ffdd5ffb4d6f))
- Add comprehensive USB HID key code unit tests ([6fbe515](https://github.com/Dicklesworthstone/savant-elite/commit/6fbe515ed9a68ba14f4ed0acfa5e75dc138af83a))
- Add modifier parsing coverage unit tests ([249b642](https://github.com/Dicklesworthstone/savant-elite/commit/249b64219528d22ace5c54b07290d35a6f090719))
- Extend modifier parsing tests with additional coverage ([643d10a](https://github.com/Dicklesworthstone/savant-elite/commit/643d10a66aa949ae3b1bfd1a55fbfeb48af1831d))
- Add normalization edge tests and manual E2E scripts ([97ae8a2](https://github.com/Dicklesworthstone/savant-elite/commit/97ae8a220fba844da274bfb80842fb1aa90f4b1a))
- Use tempfile for PedalConfig tests ([ad8f5d4](https://github.com/Dicklesworthstone/savant-elite/commit/ad8f5d44845b38c6c4dfb6a7987684b0333c88af))
- Split CI test job into unit and E2E test steps ([3c37162](https://github.com/Dicklesworthstone/savant-elite/commit/3c371627d9d0b8be902d53d5106d3177abe5873c))
- Fix deprecated `cargo_bin` usage in E2E tests ([ded0095](https://github.com/Dicklesworthstone/savant-elite/commit/ded0095343ce629e8459c47e6ee88bc040aa61ec))
- Add E2E test script infrastructure with logging ([ea2f745](https://github.com/Dicklesworthstone/savant-elite/commit/ea2f745c2b434592401c692f82caee6752762d0c))
- Add test fixture files for config validation ([4afcff5](https://github.com/Dicklesworthstone/savant-elite/commit/4afcff55adf8a22c3436c7fa51f6973bb66b362e))
- Add E2E test scripts: config/profile management (30 tests), preset workflows (19 tests), doctor diagnostics (29 tests), keys command (23 tests) ([c71cba3](https://github.com/Dicklesworthstone/savant-elite/commit/c71cba31d1bfc95091032371cd6ad2f903ed65ba), [695db4b](https://github.com/Dicklesworthstone/savant-elite/commit/695db4b5cf448f14cc21add7312ba16feefcb917), [891f837](https://github.com/Dicklesworthstone/savant-elite/commit/891f8371fd09a17ae04ee6901e01949e03afbd62), [5c221b6](https://github.com/Dicklesworthstone/savant-elite/commit/5c221b6e096612e2f7417ae67868f87558cba558))
- Fix clippy `len_zero` warning in tests ([0d749b1](https://github.com/Dicklesworthstone/savant-elite/commit/0d749b111530e00f1aca6a9f12e2759b4d48687e))

### CI / Build

- Use macos-13 for Intel CI builds ([4dca304](https://github.com/Dicklesworthstone/savant-elite/commit/4dca304fa91de6de6988b3b909982d701bd0b7c0))
- Harden program path and clean up E2E script ([63b7a4c](https://github.com/Dicklesworthstone/savant-elite/commit/63b7a4c3d52dd2a10a60348bfdbc4aae79a13823))

### Dependencies

- Update `rich_rust` dependency from pre-release/git ref to crates.io v0.2.0 ([0223aed](https://github.com/Dicklesworthstone/savant-elite/commit/0223aedc539566fc464378cc6ad34f27289c73a9))

### Chores

- Add GitHub social preview image ([45e1a9f](https://github.com/Dicklesworthstone/savant-elite/commit/45e1a9f09506ca2c59e33debfcaf8aa8994d038b))
- Update license to MIT with OpenAI/Anthropic Rider ([634bbb6](https://github.com/Dicklesworthstone/savant-elite/commit/634bbb6bc8326bd8a76e613aeddb7d466a70c935))
- Update README license references ([63c0ff1](https://github.com/Dicklesworthstone/savant-elite/commit/63c0ff1fceb19415888030ce5cb8faf0d5d3e606))
- Add issue tracking beads for follow-up work ([54d2be0](https://github.com/Dicklesworthstone/savant-elite/commit/54d2be020b46e57fd7537fd1257982fe10e8deb2))
- Update bead descriptions with comprehensive specs and test plans ([b855d11](https://github.com/Dicklesworthstone/savant-elite/commit/b855d116f241468c86a72e7ce1bae894585d172e))

---

## [v0.1.2](https://github.com/Dicklesworthstone/savant-elite/releases/tag/v0.1.2) -- 2026-01-25

Tagged at [`30b97f1`](https://github.com/Dicklesworthstone/savant-elite/commit/30b97f12a04ae3d02dcfdd9cacec848a6afc7c88). [Full diff from v0.1.1](https://github.com/Dicklesworthstone/savant-elite/compare/v0.1.1...v0.1.2).

Single-commit release focused on portability.

### Build

- Vendor `libusb` via `libusb1-sys` `vendored` feature so release binaries have zero external dependencies -- no Homebrew libusb required at runtime ([30b97f1](https://github.com/Dicklesworthstone/savant-elite/commit/30b97f12a04ae3d02dcfdd9cacec848a6afc7c88))

---

## [v0.1.1](https://github.com/Dicklesworthstone/savant-elite/releases/tag/v0.1.1) -- 2026-01-25

Tagged at [`55bbbda`](https://github.com/Dicklesworthstone/savant-elite/commit/55bbbda346380d1e772bdbcef7632050f3832f47). [Full diff from initial commit](https://github.com/Dicklesworthstone/savant-elite/compare/d9c8ade...v0.1.1).

First public release with CI-built binaries for Apple Silicon and Intel.

### Core Functionality

- **Initial implementation** of the Kinesis Savant Elite USB foot pedal programmer ([d9c8ade](https://github.com/Dicklesworthstone/savant-elite/commit/d9c8ade9d81fb0e08d60b51a60e65f5d170450b6))
  - `savant status` -- detect device and report Play vs Programming mode
  - `savant program` -- write key mappings to device EEPROM via X-keys protocol
  - `savant info` -- display detailed USB device information
  - `savant monitor` -- real-time pedal input monitoring (Play mode)
  - `savant probe` -- deep protocol probing for reverse engineering
  - Full modifier support (Cmd, Ctrl, Shift, Alt/Opt) with multiple aliases
  - Robust multi-format command fallthrough: fmt1-feat, fmt2-feat, fmt1-out, fmt2-out, 36-byte, vendor
  - `--dry-run` flag for previewing programming operations
- Remove Karabiner/hidutil workaround code -- direct USB programming only ([9897dfc](https://github.com/Dicklesworthstone/savant-elite/commit/9897dfc8ffd21e81bb494706595c13b9e835be77))
- Remove redundant cmd3 format (was identical to cmd1) ([7085a07](https://github.com/Dicklesworthstone/savant-elite/commit/7085a076a8a28c1ae25183c1734dad9513d41516))
- Fix `RawCmd` to work in both Play and Programming modes ([5199efc](https://github.com/Dicklesworthstone/savant-elite/commit/5199efc84236b28693686257cf527644389f2f3b))

### UI

- Add visual pedal display and auto-monitor feature ([8a56be8](https://github.com/Dicklesworthstone/savant-elite/commit/8a56be8eb4b4f8e01ee9f1de0205a08f242e040c))

### CI / Build

- Add GitHub Actions CI/CD workflows with cross-compilation for `aarch64-apple-darwin` and `x86_64-apple-darwin`, SHA256 checksums, and SLSA build provenance attestations ([bbf6257](https://github.com/Dicklesworthstone/savant-elite/commit/bbf6257bd44af461c7f90658bbba7c4835d09fd2))
- Update macOS runners: `macos-13` to `macos-15-intel` (13 retired) ([55bbbda](https://github.com/Dicklesworthstone/savant-elite/commit/55bbbda346380d1e772bdbcef7632050f3832f47))
- Fix formatting and remove doc tests from CI (binary-only crate) ([31a95a8](https://github.com/Dicklesworthstone/savant-elite/commit/31a95a8a207d66b9dd5b22110d5a647f77849be5))

### Documentation

- Add webp illustration and update README with curl install instructions ([0a818d3](https://github.com/Dicklesworthstone/savant-elite/commit/0a818d3f3854310844ed4168e1de74fda171740d))
- Expand README with detailed USB protocol deep dive, architecture diagram, key mapping reference, and FAQ ([fd8c62e](https://github.com/Dicklesworthstone/savant-elite/commit/fd8c62ed4b066ea99b5599f5da421792fc82f6f5))

---

## Links

- Repository: <https://github.com/Dicklesworthstone/savant-elite>
- Release page: <https://github.com/Dicklesworthstone/savant-elite/releases>

[Unreleased]: https://github.com/Dicklesworthstone/savant-elite/compare/v0.1.3...main
