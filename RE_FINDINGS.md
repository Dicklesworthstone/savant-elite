# Kinesis Savant Elite Reverse Engineering Findings

## Device Overview

**Manufacturer:** Kinesis Corporation (using PI Engineering X-keys hardware)
**Original Programming Software:** XKWdkApp.exe (Delphi application, 32-bit Windows only)
**Device Type:** USB HID foot pedal (3 pedals)

## USB Identifiers

| Mode | VID | PID | Description |
|------|-----|-----|-------------|
| Play | 0x05F3 | 0x030C | Normal operation mode |
| Program | 0x05F3 | 0x0232 | Programming mode (different PID!) |

## HID Interfaces (Play Mode)

The device exposes 3 HID endpoints:

1. **Interface 0** - Keyboard (Usage Page 0x0001, Usage 0x0006)
   - Standard 8-byte boot keyboard reports
   - Report format: [modifiers, reserved, key1, key2, key3, key4, key5, key6]

2. **Interface 1** - Mouse (Usage Page 0x0001, Usage 0x0002)
   - 4-byte mouse reports
   - Used for mouse click functionality

3. **Interface 1** - Generic Desktop (Usage Page 0x0001, Usage 0x0001)
   - Additional HID functionality

## Factory Default Scancodes

From savantconf Linux project analysis:

| Pedal | Modifiers | Key | Combined Scancode |
|-------|-----------|-----|-------------------|
| Left | Ctrl+Alt (0xE0+0xE2) | 4 (0x21) | 0x70021 |
| Middle | Ctrl+Alt (0xE0+0xE2) | 5 (0x22) | 0x70022 |
| Right | Ctrl+Alt (0xE0+0xE2) | 6 (0x23) | 0x70023 |

## Programming Mode Switch

The device has a **physical switch** on the bottom:
- Recessed switch near the Kinesis sticker
- Two positions: "Play" and "Program"
- Use a paperclip to toggle
- **Must unplug and replug USB after switching**

When switched to "Program" mode:
- Device re-enumerates with PID 0x0232 instead of 0x030C
- Accepts X-keys programming commands

## PI Engineering X-keys Protocol

The Savant Elite uses PI Engineering's X-keys protocol. Commands are sent via HID output reports:

### Command Bytes (sent to device)

| Command | Value | Description |
|---------|-------|-------------|
| CMD_GENERATE_DATA | 0xB5 | Request current device state |
| CMD_SET_LED | 0xB6 | Set LED state |
| CMD_SET_FLASH_FREQ | 0xB7 | Set LED flash frequency |
| CMD_SET_TIMESTAMP | 0xB8 | Enable/disable timestamp |
| CMD_GET_DESCRIPTOR | 0xC1 | Request device descriptor |
| CMD_SET_UNIT_ID | 0xC9 | Set unit ID |
| CMD_SET_PID | 0xCA | Change product ID (mode switch) |
| CMD_REBOOT | 0xCB | Reboot device |
| CMD_SET_KEY_MACRO | 0xCC | Program a key macro |
| CMD_GET_KEY_MACRO | 0xCD | Get key macro |
| CMD_SAVE_TO_EEPROM | 0xCE | Save settings to EEPROM |

### Command Format

Commands are sent as 36-byte HID output reports:
```
[0] = Report ID (0x00)
[1] = Command byte
[2...] = Command-specific data
```

### SET_KEY_MACRO (0xCC) Format

```
[0] = Report ID (0x00)
[1] = 0xCC (command)
[2] = Pedal index (0=left, 1=middle, 2=right)
[3] = Modifier byte (HID modifier bitmap)
[4] = Key code (HID usage code)
```

### Modifier Bitmap

| Bit | Modifier |
|-----|----------|
| 0x01 | Left Control |
| 0x02 | Left Shift |
| 0x04 | Left Alt |
| 0x08 | Left GUI (Command on Mac) |
| 0x10 | Right Control |
| 0x20 | Right Shift |
| 0x40 | Right Alt |
| 0x80 | Right GUI |

## Windows Driver Analysis

### Driver Files (from Savant-Elite-Driver.zip)

| File | Description |
|------|-------------|
| XKWdkApp.exe | Main application (Delphi/VCL) |
| XK2kJrnl.dll | Macro recording/journaling DLL |
| XkeysW2k.sys | Windows 2000 kernel driver |
| XkeysW2k.inf | Driver INF file |

### Driver INF Key Information

From XkeysW2k.inf:
- Device class: HIDClass
- Compatible IDs include: `USB\VID_05F3&PID_0232` (programming mode)
- Uses standard WDM HID filter driver architecture

### IOCTL Codes Found

| Code | Method | Description |
|------|--------|-------------|
| 0x220003 | METHOD_NEITHER | Pass-through command |

The driver is a WDM filter that forwards to the underlying USB HID stack.

## macOS Compatibility

macOS has no native support for reprogramming the device. Options:

1. **Karabiner-Elements** - Remap keys at OS level (workaround)
2. **hidapi** - Direct HID access for programming mode
3. **This tool** - Rust CLI for native programming support

## Implementation Status

- [x] Device detection (play mode)
- [x] Device detection (programming mode)
- [x] Monitor pedal input (play mode)
- [x] X-keys protocol constants
- [x] Actual EEPROM programming (programming mode, via libusb SET_REPORT; multiple formats)
- [ ] Read current configuration
- [ ] Protocol verification across firmware revisions

## Testing Notes

To fully test the programming functionality:
1. Switch device to programming mode (flip switch)
2. Replug USB
3. Run `savant status` - should show PID 0x0232
4. Run `savant program --dry-run` to see what would be sent
5. Run `savant program` to actually program
6. Switch back to play mode
7. Test pedal functionality

## References

- PI Engineering X-keys SDK documentation
- savantconf Linux project (scancodes)
- USB HID Usage Tables 1.4 specification
- Ghidra/radare2 analysis of XKWdkApp.exe and XkeysW2k.sys
