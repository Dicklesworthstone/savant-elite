# Changelog

All notable changes to [savant-elite](https://github.com/Dicklesworthstone/savant-elite) are documented in this file.

Entries are grouped by tagged release (each corresponding to a [GitHub Release](https://github.com/Dicklesworthstone/savant-elite/releases) with pre-built binaries). Within each release, changes are organized by capability area rather than commit order. Every commit hash links to the canonical repository.

---

## [Unreleased] (since v0.1.3)

> Changes on `main` after the [v0.1.3](https://github.com/Dicklesworthstone/savant-elite/releases/tag/v0.1.3) release. Not yet part of any GitHub Release.

### Documentation

- Add AGENTS.md with cass (Cross-Agent Session Search) tool reference ([c06413d](https://github.com/Dicklesworthstone/savant-elite/commit/c06413d5a6879aa9e6f073d0472b4b9e3cde96ad))

### Open Dependabot Branches (unmerged)

The following dependency-update branches exist on the remote but have **not** been merged into `main`. They are listed here for completeness; none affect the released codebase.

- `dependabot/cargo/minor-and-patch-0ea519e74d` -- bump the minor-and-patch group with 9 updates
- `dependabot/cargo/minor-and-patch-5ead927e8c` -- bump the minor-and-patch group with 8 updates
- `dependabot/github_actions/actions/attest-build-provenance-4` -- bump actions/attest-build-provenance from 2 to 4
- `dependabot/github_actions/actions/upload-artifact-7` -- bump actions/upload-artifact from 4 to 7
- `dependabot/github_actions/actions/download-artifact-8` -- bump actions/download-artifact from 4 to 8
- `dependabot/cargo/minor-and-patch-00c6fdc4b0` -- bump the minor-and-patch group with 7 updates
- `dependabot/cargo/minor-and-patch-31af542006` -- bump the minor-and-patch group with 5 updates
- `dependabot/cargo/minor-and-patch-12ee9c3d80` -- bump the minor-and-patch group with 3 updates
- `dependabot/cargo/minor-and-patch-458b43f816` -- bump the minor-and-patch group with 2 updates
- `dependabot/cargo/minor-and-patch-87104bf27d` -- bump the minor-and-patch group with 2 updates
- `dependabot/cargo/dirs-6.0.0` -- bump dirs from 5.0.1 to 6.0.0
- `dependabot/cargo/minor-and-patch-ddce1ac049` -- bump rich_rust in the minor-and-patch group
- `dependabot/github_actions/actions/attest-build-provenance-3` -- bump actions/attest-build-provenance from 2 to 3
- `dependabot/github_actions/actions/download-artifact-7` -- bump actions/download-artifact from 4 to 7
- `dependabot/github_actions/actions/upload-artifact-6` -- bump actions/upload-artifact from 4 to 6
- `dependabot/github_actions/actions/checkout-6` -- bump actions/checkout from 4 to 6

---

## [v0.1.3](https://github.com/Dicklesworthstone/savant-elite/releases/tag/v0.1.3) -- 2026-02-22

Tagged at [`c414b94`](https://github.com/Dicklesworthstone/savant-elite/commit/c414b942fad1ce116aa2500bb3a418b1f9d18e6f). [Full diff from v0.1.2](https://github.com/Dicklesworthstone/savant-elite/compare/v0.1.2...v0.1.3).

The largest release to date. Adds a complete UX command suite (presets, profiles, doctor, keys, config management), shell completions, JSON output mode, verbose diagnostics, a polished curl-pipe installer, and a comprehensive test harness covering 100+ E2E scenarios. Also includes licensing changes and numerous robustness fixes.

### New Commands

- **`savant preset`** -- Apply built-in pedal configurations for common workflows (e.g., copy/paste, media controls) ([7e124d5](https://github.com/Dicklesworthstone/savant-elite/commit/7e124d5cdda941731c0c4356db36f343f7996c5e))
- **`savant doctor`** -- Run system diagnostics: USB connectivity, permissions, driver state, device detection ([302f63f](https://github.com/Dicklesworthstone/savant-elite/commit/302f63f42f9180f7a95a9d392576cecb62d7831f))
- **`savant keys`** -- List every valid key name with its HID usage code ([616f366](https://github.com/Dicklesworthstone/savant-elite/commit/616f366a915e0c8ea006ee55fe2645bcfe682117))
- **`savant config check`** -- Validate a config file's syntax and required fields ([fc71cdf](https://github.com/Dicklesworthstone/savant-elite/commit/fc71cdfb73fdf7fca2d5fc3000d21aa91f01aa64))
- **`savant config history` / `savant config restore`** -- Automatic config backup on every write, with history browsing and point-in-time restore ([4de1dbd](https://github.com/Dicklesworthstone/savant-elite/commit/4de1dbda249a401461cdfbd3b5e3150dc8d6277f))
- **Config profiles** (`savant config save-profile`, `savant config load-profile`) -- Save and load named pedal configurations for quick switching ([9276314](https://github.com/Dicklesworthstone/savant-elite/commit/9276314b734a0955e599b924690bb0e39527fad7))

### New CLI Flags

- `--json` flag for machine-readable JSON output across all commands ([9d1ec6b](https://github.com/Dicklesworthstone/savant-elite/commit/9d1ec6b96c956e73aa6239c6a2f1035380b5f862))
- `-v` / `--verbose` flag for debug-level troubleshooting output ([27dbecd](https://github.com/Dicklesworthstone/savant-elite/commit/27dbecd45f8e18017e23da20a9f4ff90e9fee68b))
- `--timeout` flag for configurable USB operation timeout ([c031daf](https://github.com/Dicklesworthstone/savant-elite/commit/c031daf9a19ff815bbc7c793f1fcf65c11402c4b))

### Shell Completions

- Generate completion scripts for zsh, bash, and fish ([93a0955](https://github.com/Dicklesworthstone/savant-elite/commit/93a095542482a20915ff4d767102ebb119460e86))

### Installer

- Add polished `install.sh` with ASCII art banner, progress output, architecture auto-detection, and SHA256 checksum verification ([69cbc9a](https://github.com/Dicklesworthstone/savant-elite/commit/69cbc9af415c385ea11b16d53fcb5fe095534b45))
- Update README with one-liner `curl | bash` install command ([db6d744](https://github.com/Dicklesworthstone/savant-elite/commit/db6d744240bf8190b9141f8e172db2158202b5d3))
- Fix ASCII art alignment in installer ([e71ce2a](https://github.com/Dicklesworthstone/savant-elite/commit/e71ce2a1cbafe0b8733a31fdd42d399b7cb36954))
- Accept v-prefixed `SAVANT_VERSION` environment variable ([5e8b7c0](https://github.com/Dicklesworthstone/savant-elite/commit/5e8b7c0645463a41ad98648c67a1cded1262bdb6))
- Fix programming mode instructions in install script ([d98e692](https://github.com/Dicklesworthstone/savant-elite/commit/d98e6926e33723e201521b713b2d820fda33dadf))

### UI / Output Improvements

- Show current pedal config in `savant info` command output ([68d0275](https://github.com/Dicklesworthstone/savant-elite/commit/68d0275e657dc35ee77c51f907f342bca8565d64))
- Fix box alignment issues in banner and status messages ([66291e6](https://github.com/Dicklesworthstone/savant-elite/commit/66291e6c2252e08c38cd26cc6e7b22e9a35e37f9))
- Correct box border alignment for emoji/symbol content ([e73b6ce](https://github.com/Dicklesworthstone/savant-elite/commit/e73b6ce76e1efd41ce03ce9cb8d8befc16ca2eb9))
- Improve UI messaging when device is in programming mode ([0cc6839](https://github.com/Dicklesworthstone/savant-elite/commit/0cc683953eaabf944bd2941b15aca8c42c108503))
- Improve EEPROM save/disconnect message clarity ([0629d6e](https://github.com/Dicklesworthstone/savant-elite/commit/0629d6e678431e17b8253bf3e331ca244146c4d7))

### Bug Fixes

- Fix potential out-of-bounds read in `verify_pedal_programming` ([ecc86e1](https://github.com/Dicklesworthstone/savant-elite/commit/ecc86e1f901f99686c8031087724c17b78a12f1d))
- Correct `verify_pedal_programming` format check logic ([d34e4c7](https://github.com/Dicklesworthstone/savant-elite/commit/d34e4c72b6bd48a29713deb24a3cdf6b2533f216))
- Use `saturating_sub` to prevent panic on long preset names ([2e4e47a](https://github.com/Dicklesworthstone/savant-elite/commit/2e4e47a3bd1e6c19bbcd6dd18c586049383d2bae))
- Use configurable timeout for `read_control` in verification ([4de617a](https://github.com/Dicklesworthstone/savant-elite/commit/4de617acb7f3ce2f061d5eba26cdb2f5ce858045))
- Improve config check error display for missing fields ([a0193a6](https://github.com/Dicklesworthstone/savant-elite/commit/a0193a6557aa77ae910f71d57696c8795a6a9634))
- Improve config restore error handling and JSON output ([4ada402](https://github.com/Dicklesworthstone/savant-elite/commit/4ada4025c73cbbc0f756a452288e2ace52ea338a))
- Trim config values, remove dead code ([ba10f60](https://github.com/Dicklesworthstone/savant-elite/commit/ba10f607e2f7e2837c04f52e88c1d2690c39840c))
- Make integration tests platform-independent (accept both macOS and Linux) ([c414b94](https://github.com/Dicklesworthstone/savant-elite/commit/c414b942fad1ce116aa2500bb3a418b1f9d18e6f))

### Testing

- Add comprehensive input validation and E2E tests ([e24ced9](https://github.com/Dicklesworthstone/savant-elite/commit/e24ced9ab961ab6fa345393d661669f37fcdb9fc))
- Add edge case E2E tests for input validation ([474148b](https://github.com/Dicklesworthstone/savant-elite/commit/474148bea305c2fafc05c80d4710ffdd5ffb4d6f))
- Add comprehensive USB HID key code unit tests (BEAD-107) ([6fbe515](https://github.com/Dicklesworthstone/savant-elite/commit/6fbe515ed9a68ba14f4ed0acfa5e75dc138af83a))
- Add modifier parsing coverage unit tests (BEAD-108) ([249b642](https://github.com/Dicklesworthstone/savant-elite/commit/249b64219528d22ace5c54b07290d35a6f090719))
- Extend modifier parsing tests with additional coverage ([643d10a](https://github.com/Dicklesworthstone/savant-elite/commit/643d10a66aa949ae3b1bfd1a55fbfeb48af1831d))
- Add normalization edge tests and manual E2E scripts ([97ae8a2](https://github.com/Dicklesworthstone/savant-elite/commit/97ae8a220fba844da274bfb80842fb1aa90f4b1a))
- Use tempfile for PedalConfig tests ([ad8f5d4](https://github.com/Dicklesworthstone/savant-elite/commit/ad8f5d44845b38c6c4dfb6a7987684b0333c88af))
- Add E2E test script infrastructure with logging framework ([ea2f745](https://github.com/Dicklesworthstone/savant-elite/commit/ea2f745c2b434592401c692f82caee6752762d0c))
- Add test fixture files for config validation ([4afcff5](https://github.com/Dicklesworthstone/savant-elite/commit/4afcff55adf8a22c3436c7fa51f6973bb66b362e))
- Add E2E test scripts: config/profile management (30 tests) ([c71cba3](https://github.com/Dicklesworthstone/savant-elite/commit/c71cba31d1bfc95091032371cd6ad2f903ed65ba)), preset workflows (19 tests) ([695db4b](https://github.com/Dicklesworthstone/savant-elite/commit/695db4b5cf448f14cc21add7312ba16feefcb917)), doctor diagnostics (29 tests) ([891f837](https://github.com/Dicklesworthstone/savant-elite/commit/891f8371fd09a17ae04ee6901e01949e03afbd62)), keys command (23 tests) ([5c221b6](https://github.com/Dicklesworthstone/savant-elite/commit/5c221b6e096612e2f7417ae67868f87558cba558))
- Split CI test job into separate unit and E2E test steps ([3c37162](https://github.com/Dicklesworthstone/savant-elite/commit/3c371627d9d0b8be902d53d5106d3177abe5873c))
- Fix deprecated `cargo_bin` usage in E2E tests ([ded0095](https://github.com/Dicklesworthstone/savant-elite/commit/ded0095343ce629e8459c47e6ee88bc040aa61ec))
- Fix clippy `len_zero` warning in tests ([0d749b1](https://github.com/Dicklesworthstone/savant-elite/commit/0d749b111530e00f1aca6a9f12e2759b4d48687e))

### CI / Build

- Use macos-13 for Intel CI builds ([4dca304](https://github.com/Dicklesworthstone/savant-elite/commit/4dca304fa91de6de6988b3b909982d701bd0b7c0))
- Harden program path and clean up E2E script ([63b7a4c](https://github.com/Dicklesworthstone/savant-elite/commit/63b7a4c3d52dd2a10a60348bfdbc4aae79a13823))
- Bump Cargo.toml version to 0.1.3 ([5283977](https://github.com/Dicklesworthstone/savant-elite/commit/528397742ad634c98ddd93a55f14088c7f25703f))

### Dependencies

- Update `rich_rust` from pre-release git ref to crates.io v0.2.0 ([0223aed](https://github.com/Dicklesworthstone/savant-elite/commit/0223aedc539566fc464378cc6ad34f27289c73a9))

### Licensing / Metadata

- Update license to MIT with OpenAI/Anthropic Rider ([634bbb6](https://github.com/Dicklesworthstone/savant-elite/commit/634bbb6bc8326bd8a76e613aeddb7d466a70c935))
- Update README license badge and references ([63c0ff1](https://github.com/Dicklesworthstone/savant-elite/commit/63c0ff1fceb19415888030ce5cb8faf0d5d3e606))
- Add GitHub social preview image (1280x640) ([45e1a9f](https://github.com/Dicklesworthstone/savant-elite/commit/45e1a9f09506ca2c59e33debfcaf8aa8994d038b))

### Project Tracking

- Add issue tracking beads for follow-up work ([54d2be0](https://github.com/Dicklesworthstone/savant-elite/commit/54d2be020b46e57fd7537fd1257982fe10e8deb2))
- Update bead descriptions with comprehensive specs and test plans ([b855d11](https://github.com/Dicklesworthstone/savant-elite/commit/b855d116f241468c86a72e7ce1bae894585d172e))

---

## [v0.1.2](https://github.com/Dicklesworthstone/savant-elite/releases/tag/v0.1.2) -- 2026-01-25

Tagged at [`30b97f1`](https://github.com/Dicklesworthstone/savant-elite/commit/30b97f12a04ae3d02dcfdd9cacec848a6afc7c88). [Full diff from v0.1.1](https://github.com/Dicklesworthstone/savant-elite/compare/v0.1.1...v0.1.2).

Single-commit release focused on binary portability. Released the same day as v0.1.1 to fix the external libusb dependency problem.

### Build / Portability

- Vendor `libusb` via the `libusb1-sys` `vendored` Cargo feature, statically compiling libusb into the binary so release artifacts have **zero external runtime dependencies** -- no Homebrew libusb install required ([30b97f1](https://github.com/Dicklesworthstone/savant-elite/commit/30b97f12a04ae3d02dcfdd9cacec848a6afc7c88))

---

## [v0.1.1](https://github.com/Dicklesworthstone/savant-elite/releases/tag/v0.1.1) -- 2026-01-25

Tagged at [`55bbbda`](https://github.com/Dicklesworthstone/savant-elite/commit/55bbbda346380d1e772bdbcef7632050f3832f47). [Full diff from initial commit](https://github.com/Dicklesworthstone/savant-elite/compare/d9c8ade...v0.1.1).

First public release with CI-built binaries for both Apple Silicon (`aarch64-apple-darwin`) and Intel (`x86_64-apple-darwin`) Macs. Established the project's core functionality: direct USB EEPROM programming of the Kinesis Savant Elite foot pedal on modern macOS, with no VM and no background daemon required.

### Core USB Programming

- **Initial implementation** of the Kinesis Savant Elite foot pedal programmer via reverse-engineered X-keys protocol ([d9c8ade](https://github.com/Dicklesworthstone/savant-elite/commit/d9c8ade9d81fb0e08d60b51a60e65f5d170450b6)):
  - `savant status` -- detect device and report Play vs Programming mode (by USB PID)
  - `savant program` -- write key+modifier mappings to device EEPROM via USB control transfers
  - `savant info` -- display detailed USB device information (VID, PID, interface, usage page)
  - `savant monitor` -- real-time pedal input monitoring in Play mode
  - `savant probe` -- deep protocol probing for reverse engineering
  - Full modifier support: Cmd, Ctrl, Shift, Alt/Opt with multiple aliases (`cmd`/`command`/`gui`/`meta`, etc.)
  - Robust multi-format command fallthrough: fmt1-feat, fmt2-feat, fmt1-out, fmt2-out, 36-byte, vendor -- automatically tries alternative command layouts when the first fails
  - `--dry-run` flag for previewing what would be sent without writing to the device
- Remove Karabiner/hidutil workaround code -- the tool does direct USB programming only, no OS-level key remapping ([9897dfc](https://github.com/Dicklesworthstone/savant-elite/commit/9897dfc8ffd21e81bb494706595c13b9e835be77))
- Remove redundant cmd3 format that was identical to cmd1 ([7085a07](https://github.com/Dicklesworthstone/savant-elite/commit/7085a076a8a28c1ae25183c1734dad9513d41516))
- Fix `RawCmd` to work in both Play and Programming modes ([5199efc](https://github.com/Dicklesworthstone/savant-elite/commit/5199efc84236b28693686257cf527644389f2f3b))

### User Interface

- Add visual pedal display and auto-monitor feature -- shows a graphical representation of the three pedals and their current assignments ([8a56be8](https://github.com/Dicklesworthstone/savant-elite/commit/8a56be8eb4b4f8e01ee9f1de0205a08f242e040c))

### CI / Build

- Add GitHub Actions CI/CD workflows with cross-compilation build matrix for `aarch64-apple-darwin` and `x86_64-apple-darwin`, SHA256 checksums, and SLSA build provenance attestations ([bbf6257](https://github.com/Dicklesworthstone/savant-elite/commit/bbf6257bd44af461c7f90658bbba7c4835d09fd2))
- Update macOS CI runners from `macos-13` (retired) to `macos-15-intel` ([55bbbda](https://github.com/Dicklesworthstone/savant-elite/commit/55bbbda346380d1e772bdbcef7632050f3832f47))
- Fix formatting and remove doc tests from CI (binary-only crate, no library doc tests) ([31a95a8](https://github.com/Dicklesworthstone/savant-elite/commit/31a95a8a207d66b9dd5b22110d5a647f77849be5))

### Documentation

- Add project illustration (webp) and update README with `curl` install instructions ([0a818d3](https://github.com/Dicklesworthstone/savant-elite/commit/0a818d3f3854310844ed4168e1de74fda171740d))
- Expand README with detailed USB protocol deep dive, architecture diagram, key mapping reference, X-keys command byte tables, and FAQ ([fd8c62e](https://github.com/Dicklesworthstone/savant-elite/commit/fd8c62ed4b066ea99b5599f5da421792fc82f6f5))

---

## Links

- Repository: <https://github.com/Dicklesworthstone/savant-elite>
- Releases: <https://github.com/Dicklesworthstone/savant-elite/releases>
- Issue tracker: <https://github.com/Dicklesworthstone/savant-elite/issues>

[Unreleased]: https://github.com/Dicklesworthstone/savant-elite/compare/v0.1.3...main
[v0.1.3]: https://github.com/Dicklesworthstone/savant-elite/compare/v0.1.2...v0.1.3
[v0.1.2]: https://github.com/Dicklesworthstone/savant-elite/compare/v0.1.1...v0.1.2
[v0.1.1]: https://github.com/Dicklesworthstone/savant-elite/compare/d9c8ade...v0.1.1
