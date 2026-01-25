# savant-elite

<div align="center">

<img src="https://raw.githubusercontent.com/Dicklesworthstone/savant-elite/main/savant_elite_illustration.webp" alt="Savant Elite Foot Pedal Illustration" width="600">

[![CI](https://github.com/Dicklesworthstone/savant-elite/actions/workflows/ci.yml/badge.svg)](https://github.com/Dicklesworthstone/savant-elite/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![macOS](https://img.shields.io/badge/macOS-12+-blue.svg)](https://www.apple.com/macos/)

**Native macOS programmer for the discontinued Kinesis Savant Elite USB foot pedal.**

Program your foot pedals directly via USB—no Windows VM, no 32-bit compatibility hacks, no Karabiner workarounds. Just `savant program` and you're done.

### Quick Install

**macOS (Apple Silicon)**
```bash
curl -fsSL https://github.com/Dicklesworthstone/savant-elite/releases/latest/download/savant-darwin-arm64.tar.xz | tar -xJ && sudo mv savant /usr/local/bin/
```

**macOS (Intel)**
```bash
curl -fsSL https://github.com/Dicklesworthstone/savant-elite/releases/latest/download/savant-darwin-amd64.tar.xz | tar -xJ && sudo mv savant /usr/local/bin/
```

**Verify Checksum (optional)**
```bash
curl -fsSL https://github.com/Dicklesworthstone/savant-elite/releases/latest/download/SHA256SUMS
shasum -a 256 -c SHA256SUMS
```

</div>

---

## TL;DR

**The Problem**: The Kinesis Savant Elite foot pedal (discontinued 2015) can only be programmed with 32-bit Windows software. macOS users are stuck with whatever keys were programmed at the factory, or hacky remapping solutions.

**The Solution**: `savant-elite` reverse-engineers the USB protocol and programs the pedals directly from macOS. Changes are written to the device's EEPROM and persist forever—no background software needed.

### Why Use savant-elite?

| Feature | What It Does |
|---------|--------------|
| **Native Programming** | Write key mappings directly to device EEPROM |
| **Zero Runtime Overhead** | No daemon, no remapper—pedals send programmed keys natively |
| **Full Modifier Support** | Program Cmd+C, Ctrl+Shift+Alt+F12, or any combination |
| **Reverse-Engineered Protocol** | Based on analysis of original Windows driver and PI Engineering X-keys SDK |

---

## Quick Example

```bash
# Check if device is connected and what mode it's in
$ savant status
Found device in PROGRAMMING mode (PID 0x0232)

# Program your pedals (Copy / Select All / Paste)
$ savant program --left "cmd+c" --middle "cmd+a" --right "cmd+v"
Programming Left pedal... OK
Programming Middle pedal... OK
Programming Right pedal... OK
Saving to EEPROM... OK

# Flip switch back to Play mode, replug USB, done!
```

---

## How It Works

The Savant Elite has a physical switch on the bottom that toggles between **Play** (normal operation) and **Program** (accepts configuration commands) modes. Each mode uses a different USB Product ID:

| Mode | PID | Purpose |
|------|-----|---------|
| Play | `0x030C` | Normal operation—sends programmed keys |
| Program | `0x0232` | Accepts programming commands via USB |

This tool sends X-keys protocol commands to program the EEPROM when in Program mode.

---

## How savant-elite Compares

| Feature | savant-elite | Windows Software |
|---------|--------------|------------------|
| Native EEPROM programming | ✅ Direct | ✅ Direct |
| Works on modern macOS | ✅ Native | ❌ Requires VM |
| Persists after unplug | ✅ Yes | ✅ Yes |
| Runtime overhead | ✅ None | ✅ None |
| Any key combination | ✅ Full HID | ✅ Full |

**When to use savant-elite:**
- You have a Kinesis Savant Elite foot pedal
- You want to program it on macOS without a Windows VM
- You want permanent programming with zero runtime overhead

**When savant-elite might not be ideal:**
- You want to change mappings frequently without flipping the physical switch

---

## Installation

### Pre-built Binary (Recommended)

**macOS (Apple Silicon)**
```bash
curl -fsSL https://github.com/Dicklesworthstone/savant-elite/releases/latest/download/savant-darwin-arm64.tar.xz | tar -xJ
sudo mv savant /usr/local/bin/
```

**macOS (Intel)**
```bash
curl -fsSL https://github.com/Dicklesworthstone/savant-elite/releases/latest/download/savant-darwin-amd64.tar.xz | tar -xJ
sudo mv savant /usr/local/bin/
```

**Verify Checksum**
```bash
# Download checksum file
curl -fsSL https://github.com/Dicklesworthstone/savant-elite/releases/latest/download/SHA256SUMS -o SHA256SUMS

# Download binary
curl -fsSL https://github.com/Dicklesworthstone/savant-elite/releases/latest/download/savant-darwin-arm64.tar.xz -o savant-darwin-arm64.tar.xz

# Verify
shasum -a 256 -c SHA256SUMS --ignore-missing
```

### From Source

```bash
# Clone and build
git clone https://github.com/Dicklesworthstone/savant-elite.git
cd savant-elite
cargo build --release

# Install to PATH
sudo cp target/release/savant /usr/local/bin/
```

### Via Cargo

```bash
cargo install --git https://github.com/Dicklesworthstone/savant-elite.git
```

### Requirements

- **macOS 12+** (tested on macOS 15 Sequoia)
- **Rust 1.70+** (for building from source)
- **Kinesis Savant Elite** foot pedal (VID `0x05F3`)

---

## Quick Start

### 1. Check Device Status

```bash
savant status
```

If in **Play mode**, you'll see instructions to switch to Programming mode.

### 2. Switch to Programming Mode

1. Flip the pedal over
2. Find the recessed switch near the "Kinesis" sticker
3. Use a paperclip to flip it from **Play** to **Program**
4. Unplug and replug the USB cable

### 3. Program Your Pedals

```bash
# Default: Copy / Select All / Paste
savant program

# Custom configuration
savant program --left "cmd+c" --middle "cmd+a" --right "cmd+v"

# Dry run (see what would be sent)
savant program --dry-run
```

### 4. Return to Play Mode

1. Flip the switch back to **Play**
2. Unplug and replug the USB cable
3. Test your pedals!

---

## Commands

### `savant status`

Check device connection and mode.

```bash
$ savant status
Found device in PROGRAMMING mode (PID 0x0232) [via libusb]
  Bus 001 Device 016
  Product: Footpedal
  Manufacturer: Kinesis
```

### `savant program`

Program the pedals (requires Programming mode).

```bash
# Default configuration
savant program

# Custom keys
savant program --left "ctrl+z" --middle "ctrl+shift+z" --right "cmd+s"

# Preview without writing
savant program --dry-run
```

**Supported modifiers:** `cmd`, `ctrl`, `shift`, `alt`, `opt`
**Supported keys:** `a-z`, `0-9`, `f1-f12`, `enter`, `space`, `tab`, `escape`, etc.

### `savant info`

Show detailed device information.

```bash
$ savant info
Found Kinesis Savant Elite foot pedal:
  Vendor ID:  0x05F3
  Product ID: 0x030C
  Interface:  0
  Usage Page: 0x0001
  Usage:      0x0006
```

### `savant monitor`

Monitor pedal input in real-time (Play mode only).

```bash
$ savant monitor --duration 30
Monitoring Savant Elite foot pedal...
Press pedals to see what keys they send.

Pressed: LCtrl+LAlt+4  [raw: 050000210000000]
Released [raw: 0000000000000000]
```

### `savant probe`

Deep protocol probing for reverse engineering.

```bash
savant probe
```

---

## Key Mapping Reference

### Modifiers

| Modifier | Aliases | HID Code |
|----------|---------|----------|
| Command | `cmd`, `command`, `gui`, `meta` | `0x08` |
| Control | `ctrl`, `control` | `0x01` |
| Shift | `shift` | `0x02` |
| Option | `alt`, `opt`, `option` | `0x04` |

### Common Keys

| Key | HID Code | Key | HID Code |
|-----|----------|-----|----------|
| A-Z | `0x04-0x1D` | 0-9 | `0x27, 0x1E-0x26` |
| F1-F12 | `0x3A-0x45` | Enter | `0x28` |
| Space | `0x2C` | Tab | `0x2B` |
| Escape | `0x29` | Backspace | `0x2A` |
| Arrows | `0x4F-0x52` | | |

---

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                      User Command                               │
│   savant program --left "cmd+c" --middle "cmd+a" --right "cmd+v"│
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                    savant-elite CLI                             │
│   • Parse key combinations                                      │
│   • Convert to HID modifier + keycode format                    │
│   • Detect device mode (Play vs Program)                        │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                     rusb / libusb                               │
│   • USB control transfers                                       │
│   • HID SET_REPORT requests                                     │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│               Kinesis Savant Elite (Program Mode)               │
│   • Receives X-keys protocol commands                           │
│   • CMD_SET_KEY_MACRO (0xCC) per pedal                         │
│   • CMD_SAVE_TO_EEPROM (0xCE)                                  │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                        EEPROM                                   │
│   Permanent storage - survives power cycles                     │
└─────────────────────────────────────────────────────────────────┘
```

---

## Design Philosophy

### Direct Hardware Access Over Workarounds

The Savant Elite stores key mappings in onboard EEPROM. Rather than intercepting keystrokes at the OS level (which requires background processes and introduces latency), `savant-elite` writes directly to the device's permanent storage. Once programmed, the pedal sends the correct keys natively—no software running, no daemon needed, no CPU cycles spent remapping.

### Exhaustive Protocol Discovery

USB HID devices speak a specific protocol, but manufacturers rarely document it. `savant-elite` was built through systematic reverse engineering:

1. **Driver Analysis**: Extracting vendor/product IDs and protocol hints from Windows INF files
2. **USB Capture**: Using Wireshark to capture programming sessions from the original Windows software
3. **Protocol Fuzzing**: Systematically trying different command formats to find what works
4. **Iterative Refinement**: Multiple transfer methods (feature reports, output reports, vendor requests) until the right one succeeds

### Robustness Through Redundancy

The programming logic tries multiple command formats automatically:

```
fmt1-feat  →  fmt2-feat  →  fmt1-out  →  fmt2-out  →  36-byte  →  vendor
```

If the first format fails (PIPE error), it falls through to the next. This handles firmware variations and ensures programming succeeds across different device batches.

### Minimal Dependencies

The tool uses only what's necessary:

| Crate | Purpose |
|-------|---------|
| `clap` | CLI argument parsing |
| `hidapi` | HID device enumeration (Play mode) |
| `rusb` | Raw USB access (Programming mode) |
| `anyhow` | Error handling |
| `hex` | Debug output formatting |

No async runtime, no network access, no configuration files.

---

## USB Protocol Deep Dive

### Device Identification

The Savant Elite identifies itself differently based on mode:

| Mode | Vendor ID | Product ID | USB Class |
|------|-----------|------------|-----------|
| Play | `0x05F3` | `0x030C` | HID (keyboard + mouse composite) |
| Program | `0x05F3` | `0x0232` | HID (generic) |

In Play mode, macOS sees a standard HID keyboard and can read input via `hidapi`. In Program mode, the device doesn't expose standard HID interfaces—we use `rusb` (libusb) for raw USB control transfers.

### The X-keys Protocol

The Savant Elite uses PI Engineering's X-keys protocol (the same family as X-keys keypads). Key commands:

| Command | Byte | Purpose |
|---------|------|---------|
| `CMD_SET_KEY_MACRO` | `0xCC` | Set a pedal's key mapping |
| `CMD_SAVE_TO_EEPROM` | `0xCE` | Persist changes to flash |
| `CMD_GET_KEY_MACRO` | `0xCD` | Read current mapping (partially implemented) |
| `CMD_GENERATE_DATA` | `0xB5` | Request device state |
| `CMD_GET_DESCRIPTOR` | `0xC1` | Get device descriptor |

### SET_KEY_MACRO Command Format

```
Byte 0: Command (0xCC)
Byte 1: Pedal index (0=left, 1=middle, 2=right)
Byte 2: HID modifier byte
Byte 3: HID keycode
Bytes 4-7: Reserved (zeros)
```

The modifier byte follows USB HID convention:

```
Bit 0: Left Control   (0x01)
Bit 1: Left Shift     (0x02)
Bit 2: Left Alt       (0x04)
Bit 3: Left GUI/Cmd   (0x08)
Bit 4: Right Control  (0x10)
Bit 5: Right Shift    (0x20)
Bit 6: Right Alt      (0x40)
Bit 7: Right GUI/Cmd  (0x80)
```

### USB Control Transfer Details

Programming uses HID SET_REPORT via USB control transfers:

```
bmRequestType: 0x21 (Host-to-device, Class, Interface)
bRequest:      0x09 (SET_REPORT)
wValue:        0x0200 | report_id (Output report type)
wIndex:        Interface number (0)
Data:          8-byte command buffer
```

The tool detaches the kernel driver if necessary (`handle.detach_kernel_driver()`) and claims the interface for exclusive access.

---

## How EEPROM Programming Works

### The Programming Sequence

1. **Mode Check**: Enumerate USB devices looking for VID `0x05F3`, PID `0x0232`
2. **Device Open**: Get libusb handle, detach kernel driver if attached
3. **Interface Claim**: Exclusively claim interface 0
4. **Send Commands**: For each pedal:
   - Build SET_KEY_MACRO command with modifier + keycode
   - Send via USB control transfer
   - Try multiple formats until one succeeds
   - Brief delay between pedals (50ms)
5. **Save to EEPROM**: Send CMD_SAVE_TO_EEPROM (0xCE)
6. **Cleanup**: Release interface, device auto-closes

### Why Multiple Command Formats?

Different firmware versions may expect slightly different data layouts:

| Format | Report ID Position | Data Layout |
|--------|-------------------|-------------|
| fmt1 | Byte 0 = Command | `[CMD, pedal, mod, key, 0, 0, 0, 0]` |
| fmt2 | Byte 0 = 0 | `[0, CMD, pedal, mod, key, 0, 0, 0]` |
| 36-byte | Extended buffer | PI Engineering SDK default size |
| vendor | Vendor request | Alternative transfer method |

The tool tries Feature reports (`wValue = 0x0300`) first, then Output reports (`wValue = 0x0200`). Most devices respond to fmt1-out (Output report with command as first byte).

### EEPROM Write Verification

After programming, the SAVE_TO_EEPROM command triggers a flash write cycle. The device doesn't provide explicit acknowledgment, but a successful control transfer indicates the command was received. The 200ms delay after saving ensures the write completes before releasing the interface.

---

## Key Parsing Algorithm

### Input Processing

Key combinations are parsed from human-readable strings:

```
"cmd+shift+c" → { modifiers: 0x0A, key: 0x06 }
```

The parser:
1. Splits on `+` delimiter
2. Processes all but the last token as modifiers
3. Last token is the base key
4. Accumulates modifier bits with OR operations

### Modifier Resolution

Multiple names map to the same modifier bit:

```rust
"cmd" | "command" | "gui" | "meta" | "super" → MOD_LEFT_GUI (0x08)
"ctrl" | "control"                           → MOD_LEFT_CTRL (0x01)
"shift"                                      → MOD_LEFT_SHIFT (0x02)
"alt" | "option" | "opt"                     → MOD_LEFT_ALT (0x04)
```

### Key Code Lookup

Keys are resolved to HID usage codes via a lookup table. Examples:

```
"a" → 0x04
"c" → 0x06
"v" → 0x19
"f12" → 0x45
"space" → 0x2C
```

The parser is case-insensitive and validates that each token resolves to a known modifier or key.

---

## Security Considerations

### USB Device Access

The tool requires sufficient permissions to:
- Enumerate USB devices
- Detach kernel drivers (may require root)
- Send control transfers to raw USB endpoints

On macOS, this typically requires running with `sudo` for the `program` command.

### No Network Access

`savant-elite` never accesses the network. All operations are local USB communication. The tool has no telemetry, no update checks, no external dependencies at runtime.

### Device Safety

The Savant Elite's firmware is read-only—the tool cannot modify it. Only the user-programmable EEPROM area is written to. The worst case scenario is programming unintended keys, which is easily corrected by reprogramming.

### Checksum Verification

Release binaries include SHA256 checksums and SLSA build provenance attestations, allowing verification that binaries match the source code.

---

## Why Rust?

### Memory Safety

USB protocol parsing involves raw byte manipulation. Rust's ownership system prevents buffer overflows and use-after-free bugs without runtime overhead.

### Cross-Compilation

Rust compiles to native binaries for both Apple Silicon (aarch64) and Intel (x86_64) Macs from the same codebase, with GitHub Actions handling the build matrix.

### Excellent USB Libraries

The `rusb` and `hidapi` crates provide mature, well-tested bindings to libusb and hidapi, abstracting platform-specific details while preserving low-level control when needed.

### Error Handling

Rust's `Result` type and the `anyhow` crate enable clear error propagation with context. Every fallible operation produces actionable error messages rather than silent failures.

---

## Troubleshooting

### "No Savant Elite device found"

```bash
# Check USB connection
system_profiler SPUSBDataType | grep -A5 Kinesis

# Verify with ioreg
ioreg -p IOUSB | grep -i foot
```

**Fix:** Unplug and replug the USB cable. Ensure the switch is in the correct position.

### "Device is in PLAY mode, not PROGRAMMING mode"

The device needs to be in Programming mode to accept configuration.

**Fix:**
1. Flip pedal over
2. Use paperclip to flip switch to "Program"
3. Unplug and replug USB
4. Run `savant status` to verify

### "Failed to claim interface"

macOS may have claimed the device.

**Fix:**
```bash
# Check if anything is using the device
sudo lsof | grep -i hid
```

### Programming worked but keys don't match

The EEPROM was programmed but you may still be in Program mode.

**Fix:**
1. Flip switch back to "Play"
2. Unplug and replug USB
3. Test in a text editor

---

## Limitations

### What savant-elite Doesn't Do

- **No macro recording**: Only single key + modifiers (not sequences)
- **No per-application mappings**: Device programming is global
- **No LED control**: The Savant Elite doesn't have programmable LEDs
- **No Windows/Linux support**: macOS only (PRs welcome for other platforms)

### Known Limitations

| Capability | Status | Notes |
|------------|--------|-------|
| Read current config | ⚠️ Partial | Detection works, readback WIP |
| Multi-key macros | ❌ Not supported | Hardware limitation |
| Mouse button output | ❌ Not implemented | Possible but not done |

---

## History & Background

### The Kinesis Savant Elite

The Savant Elite is a three-pedal USB foot controller manufactured by Kinesis (known for their ergonomic keyboards). It was designed for users who wanted hands-free keyboard shortcuts—ideal for transcriptionists, video editors, and programmers with RSI.

The hardware was actually manufactured by PI Engineering (makers of X-keys products) and rebranded by Kinesis. This explains why it uses the PI Engineering USB vendor ID (`0x05F3`) and speaks the X-keys protocol.

### The Programming Problem

The Savant Elite was designed to be user-programmable via Windows software called "SmartSet." Unfortunately:

- SmartSet was 32-bit only
- Microsoft removed 32-bit app support in Windows on ARM
- Apple removed 32-bit app support in macOS Catalina (2019)
- Kinesis discontinued the product around 2015

This left users with pedals that could only send whatever keys were programmed at the factory, unless they maintained a legacy Windows system.

### Common Workarounds (Before This Tool)

| Approach | Downsides |
|----------|-----------|
| Windows VM | Requires Windows license, VM software, significant overhead |
| Old Mac/PC | Maintaining legacy hardware just for one tool |
| Karabiner-Elements | Requires background process, added latency, complex configuration |
| hidutil remapping | Limited to key-to-key (no modifier combinations) |

### The Solution: Reverse Engineering

By analyzing the Windows driver files, capturing USB traffic, and testing X-keys protocol commands, it's possible to program the device directly from modern macOS. The pedal stores its configuration in EEPROM, so once programmed, it works natively with zero software overhead.

---

## FAQ

### Why "savant-elite"?

Named after the Kinesis Savant Elite foot pedal—the device this tool programs.

### Is this safe? Can I brick my pedal?

The tool only writes to user-programmable EEPROM. The device firmware is read-only. Worst case, you program keys you didn't intend and need to reprogram.

### What's the factory default?

Most Savant Elite pedals ship programmed with:
- Left: Ctrl+Alt+4
- Middle: Ctrl+Alt+5
- Right: Ctrl+Alt+6

### Does it work with other Kinesis/PI Engineering devices?

Possibly! The X-keys protocol is shared across PI Engineering products. Try `savant probe` to investigate. Other devices may have different PIDs.

### Can I use this to read what's currently programmed?

Not yet. The read functionality is partially implemented. Use `savant monitor` in Play mode to see what keys your pedals currently send.

### Why does programming require a physical switch?

This is a hardware design choice by PI Engineering. The separate PIDs for Play/Program modes ensure you can't accidentally reprogram the device during normal use.

---

## Reverse Engineering Notes

See [RE_FINDINGS.md](RE_FINDINGS.md) for detailed protocol documentation including:
- USB descriptors and PIDs
- X-keys command bytes
- HID report formats
- Windows driver analysis

---

## About Contributions

Please don't take this the wrong way, but I do not accept outside contributions for any of my projects. I simply don't have the mental bandwidth to review anything, and it's my name on the thing, so I'm responsible for any problems it causes; thus, the risk-reward is highly asymmetric from my perspective. I'd also have to worry about other "stakeholders," which seems unwise for tools I mostly make for myself for free. Feel free to submit issues, and even PRs if you want to illustrate a proposed fix, but know I won't merge them directly. Instead, I'll have Claude or Codex review submissions via `gh` and independently decide whether and how to address them. Bug reports in particular are welcome. Sorry if this offends, but I want to avoid wasted time and hurt feelings. I understand this isn't in sync with the prevailing open-source ethos that seeks community contributions, but it's the only way I can move at this velocity and keep my sanity.

---

## License

MIT License. See [LICENSE](LICENSE) for details.

---

<div align="center">

*Made with ☕ and a foot pedal*

</div>
