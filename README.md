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
