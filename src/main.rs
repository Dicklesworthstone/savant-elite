use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand};
use hidapi::{HidApi, HidDevice};
use rich_rust::markup;
use rich_rust::prelude::*;
use rich_rust::r#box::ROUNDED;
use rusb::{Device, GlobalContext};
use std::time::Duration;

const KINESIS_VID: u16 = 0x05F3;
const SAVANT_ELITE_PID: u16 = 0x030C; // Normal "play" mode PID
const PROGRAMMING_PID: u16 = 0x0232; // Programming mode PID (from driver INF)

// PI Engineering X-keys protocol commands (used by Kinesis Savant Elite)
// These constants document the full protocol even if not all are currently used.
#[allow(dead_code)]
mod xkeys_protocol {
    // Output report commands (sent to device)
    pub const CMD_GENERATE_DATA: u8 = 0xB5; // Request device state
    pub const CMD_SET_LED: u8 = 0xB6; // Set LED state
    pub const CMD_SET_FLASH_FREQ: u8 = 0xB7; // Set flash frequency
    pub const CMD_SET_TIMESTAMP: u8 = 0xB8; // Enable/disable timestamp
    pub const CMD_GET_DESCRIPTOR: u8 = 0xC1; // Request device descriptor
    pub const CMD_SET_UNIT_ID: u8 = 0xC9; // Set unit ID
    pub const CMD_SET_PID: u8 = 0xCA; // Change product ID (mode switch)
    pub const CMD_REBOOT: u8 = 0xCB; // Reboot device
    pub const CMD_SET_KEY_MACRO: u8 = 0xCC; // Program a key macro
    pub const CMD_GET_KEY_MACRO: u8 = 0xCD; // Get key macro
    pub const CMD_SAVE_TO_EEPROM: u8 = 0xCE; // Save to EEPROM

    // Pedal indices
    pub const PEDAL_LEFT: u8 = 0;
    pub const PEDAL_MIDDLE: u8 = 1;
    pub const PEDAL_RIGHT: u8 = 2;
}

// USB HID keyboard usage codes
// See: https://usb.org/sites/default/files/hut1_4.pdf (Section 10)
// These constants document the full HID spec even if not all are currently used.
#[allow(dead_code)]
mod usb_hid {
    // Modifier keys (byte 0 of keyboard report)
    pub const MOD_LEFT_CTRL: u8 = 0x01;
    pub const MOD_LEFT_SHIFT: u8 = 0x02;
    pub const MOD_LEFT_ALT: u8 = 0x04;
    pub const MOD_LEFT_GUI: u8 = 0x08; // Command on Mac
    pub const MOD_RIGHT_CTRL: u8 = 0x10;
    pub const MOD_RIGHT_SHIFT: u8 = 0x20;
    pub const MOD_RIGHT_ALT: u8 = 0x40;
    pub const MOD_RIGHT_GUI: u8 = 0x80;

    // Common key codes (bytes 2-7 of keyboard report)
    pub const KEY_A: u8 = 0x04;
    pub const KEY_B: u8 = 0x05;
    pub const KEY_C: u8 = 0x06;
    pub const KEY_D: u8 = 0x07;
    pub const KEY_E: u8 = 0x08;
    pub const KEY_F: u8 = 0x09;
    pub const KEY_G: u8 = 0x0A;
    pub const KEY_H: u8 = 0x0B;
    pub const KEY_I: u8 = 0x0C;
    pub const KEY_J: u8 = 0x0D;
    pub const KEY_K: u8 = 0x0E;
    pub const KEY_L: u8 = 0x0F;
    pub const KEY_M: u8 = 0x10;
    pub const KEY_N: u8 = 0x11;
    pub const KEY_O: u8 = 0x12;
    pub const KEY_P: u8 = 0x13;
    pub const KEY_Q: u8 = 0x14;
    pub const KEY_R: u8 = 0x15;
    pub const KEY_S: u8 = 0x16;
    pub const KEY_T: u8 = 0x17;
    pub const KEY_U: u8 = 0x18;
    pub const KEY_V: u8 = 0x19;
    pub const KEY_W: u8 = 0x1A;
    pub const KEY_X: u8 = 0x1B;
    pub const KEY_Y: u8 = 0x1C;
    pub const KEY_Z: u8 = 0x1D;
    pub const KEY_1: u8 = 0x1E;
    pub const KEY_2: u8 = 0x1F;
    pub const KEY_3: u8 = 0x20;
    pub const KEY_4: u8 = 0x21;
    pub const KEY_5: u8 = 0x22;
    pub const KEY_6: u8 = 0x23;
    pub const KEY_7: u8 = 0x24;
    pub const KEY_8: u8 = 0x25;
    pub const KEY_9: u8 = 0x26;
    pub const KEY_0: u8 = 0x27;
    pub const KEY_ENTER: u8 = 0x28;
    pub const KEY_ESC: u8 = 0x29;
    pub const KEY_BACKSPACE: u8 = 0x2A;
    pub const KEY_TAB: u8 = 0x2B;
    pub const KEY_SPACE: u8 = 0x2C;
    pub const KEY_F1: u8 = 0x3A;
    pub const KEY_F2: u8 = 0x3B;
    pub const KEY_F3: u8 = 0x3C;
    pub const KEY_F4: u8 = 0x3D;
    pub const KEY_F5: u8 = 0x3E;
    pub const KEY_F6: u8 = 0x3F;
    pub const KEY_F7: u8 = 0x40;
    pub const KEY_F8: u8 = 0x41;
    pub const KEY_F9: u8 = 0x42;
    pub const KEY_F10: u8 = 0x43;
    pub const KEY_F11: u8 = 0x44;
    pub const KEY_F12: u8 = 0x45;
    pub const KEY_LEFT: u8 = 0x50;
    pub const KEY_RIGHT: u8 = 0x4F;
    pub const KEY_UP: u8 = 0x52;
    pub const KEY_DOWN: u8 = 0x51;

    pub fn key_name(code: u8) -> &'static str {
        match code {
            0x00 => "None",
            0x04 => "A",
            0x05 => "B",
            0x06 => "C",
            0x07 => "D",
            0x08 => "E",
            0x09 => "F",
            0x0A => "G",
            0x0B => "H",
            0x0C => "I",
            0x0D => "J",
            0x0E => "K",
            0x0F => "L",
            0x10 => "M",
            0x11 => "N",
            0x12 => "O",
            0x13 => "P",
            0x14 => "Q",
            0x15 => "R",
            0x16 => "S",
            0x17 => "T",
            0x18 => "U",
            0x19 => "V",
            0x1A => "W",
            0x1B => "X",
            0x1C => "Y",
            0x1D => "Z",
            0x1E => "1",
            0x1F => "2",
            0x20 => "3",
            0x21 => "4",
            0x22 => "5",
            0x23 => "6",
            0x24 => "7",
            0x25 => "8",
            0x26 => "9",
            0x27 => "0",
            0x28 => "Enter",
            0x29 => "Escape",
            0x2A => "Backspace",
            0x2B => "Tab",
            0x2C => "Space",
            0x2D => "Minus",
            0x2E => "Equal",
            0x2F => "LeftBracket",
            0x30 => "RightBracket",
            0x31 => "Backslash",
            0x33 => "Semicolon",
            0x34 => "Quote",
            0x35 => "Grave",
            0x36 => "Comma",
            0x37 => "Period",
            0x38 => "Slash",
            0x39 => "CapsLock",
            0x3A => "F1",
            0x3B => "F2",
            0x3C => "F3",
            0x3D => "F4",
            0x3E => "F5",
            0x3F => "F6",
            0x40 => "F7",
            0x41 => "F8",
            0x42 => "F9",
            0x43 => "F10",
            0x44 => "F11",
            0x45 => "F12",
            0x4F => "Right",
            0x50 => "Left",
            0x51 => "Down",
            0x52 => "Up",
            _ => "Unknown",
        }
    }

    pub fn modifier_names(mods: u8) -> Vec<&'static str> {
        let mut names = Vec::new();
        if mods & MOD_LEFT_CTRL != 0 {
            names.push("LCtrl");
        }
        if mods & MOD_LEFT_SHIFT != 0 {
            names.push("LShift");
        }
        if mods & MOD_LEFT_ALT != 0 {
            names.push("LAlt");
        }
        if mods & MOD_LEFT_GUI != 0 {
            names.push("LCmd");
        }
        if mods & MOD_RIGHT_CTRL != 0 {
            names.push("RCtrl");
        }
        if mods & MOD_RIGHT_SHIFT != 0 {
            names.push("RShift");
        }
        if mods & MOD_RIGHT_ALT != 0 {
            names.push("RAlt");
        }
        if mods & MOD_RIGHT_GUI != 0 {
            names.push("RCmd");
        }
        names
    }

    pub fn normalize_boot_keyboard_report(data: &[u8]) -> Option<[u8; 8]> {
        if data.len() < 8 {
            return None;
        }

        // Heuristic: Some HID backends include a leading report-id byte (often 0) and/or pad
        // input reports out to a larger endpoint size. The Savant Elite keyboard interface is a
        // standard 8-byte boot keyboard report, so we normalize to the first 8 bytes of the
        // actual report.
        let looks_prefixed = data.len() >= 9
            && data[0] == 0
            && data[2] == 0
            && (data[1] != 0 || data[3..9].iter().any(|&b| b != 0));
        let offset = if looks_prefixed { 1 } else { 0 };
        if data.len() < offset + 8 {
            return None;
        }

        let mut report = [0u8; 8];
        report.copy_from_slice(&data[offset..offset + 8]);
        Some(report)
    }

    pub fn parse_key_name(name: &str) -> Option<u8> {
        match name.to_lowercase().as_str() {
            "a" => Some(KEY_A),
            "b" => Some(KEY_B),
            "c" => Some(KEY_C),
            "d" => Some(KEY_D),
            "e" => Some(KEY_E),
            "f" => Some(KEY_F),
            "g" => Some(KEY_G),
            "h" => Some(KEY_H),
            "i" => Some(KEY_I),
            "j" => Some(KEY_J),
            "k" => Some(KEY_K),
            "l" => Some(KEY_L),
            "m" => Some(KEY_M),
            "n" => Some(KEY_N),
            "o" => Some(KEY_O),
            "p" => Some(KEY_P),
            "q" => Some(KEY_Q),
            "r" => Some(KEY_R),
            "s" => Some(KEY_S),
            "t" => Some(KEY_T),
            "u" => Some(KEY_U),
            "v" => Some(KEY_V),
            "w" => Some(KEY_W),
            "x" => Some(KEY_X),
            "y" => Some(KEY_Y),
            "z" => Some(KEY_Z),
            "1" => Some(KEY_1),
            "2" => Some(KEY_2),
            "3" => Some(KEY_3),
            "4" => Some(KEY_4),
            "5" => Some(KEY_5),
            "6" => Some(KEY_6),
            "7" => Some(KEY_7),
            "8" => Some(KEY_8),
            "9" => Some(KEY_9),
            "0" => Some(KEY_0),
            "enter" | "return" => Some(KEY_ENTER),
            "esc" | "escape" => Some(KEY_ESC),
            "backspace" => Some(KEY_BACKSPACE),
            "tab" => Some(KEY_TAB),
            "space" => Some(KEY_SPACE),
            "f1" => Some(KEY_F1),
            "f2" => Some(KEY_F2),
            "f3" => Some(KEY_F3),
            "f4" => Some(KEY_F4),
            "f5" => Some(KEY_F5),
            "f6" => Some(KEY_F6),
            "f7" => Some(KEY_F7),
            "f8" => Some(KEY_F8),
            "f9" => Some(KEY_F9),
            "f10" => Some(KEY_F10),
            "f11" => Some(KEY_F11),
            "f12" => Some(KEY_F12),
            "left" => Some(KEY_LEFT),
            "right" => Some(KEY_RIGHT),
            "up" => Some(KEY_UP),
            "down" => Some(KEY_DOWN),
            // Punctuation and special keys
            "minus" | "-" => Some(0x2D),
            "equal" | "=" => Some(0x2E),
            "leftbracket" | "[" => Some(0x2F),
            "rightbracket" | "]" => Some(0x30),
            "backslash" | "\\" => Some(0x31),
            "semicolon" | ";" => Some(0x33),
            "quote" | "'" => Some(0x34),
            "grave" | "`" => Some(0x35),
            "comma" | "," => Some(0x36),
            "period" | "." => Some(0x37),
            "slash" | "/" => Some(0x38),
            "capslock" => Some(0x39),
            _ => None,
        }
    }
}

#[derive(Parser)]
#[command(name = "savant")]
#[command(version)]
#[command(about = "Kinesis Savant Elite foot pedal programmer for macOS")]
#[command(
    long_about = "Native macOS programmer for the discontinued Kinesis Savant Elite USB foot pedal.\n\nProgram your foot pedals directly via USB—no Windows VM, no 32-bit compatibility hacks."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Detect and show info about connected Savant Elite pedals
    Info,

    /// Monitor foot pedal input in real-time (requires Input Monitoring permission)
    Monitor {
        /// Duration in seconds (0 = infinite)
        #[arg(short, long, default_value = "30")]
        duration: u64,
    },

    /// Program the pedals (requires device in programming mode)
    Program {
        /// Left pedal action (e.g., "cmd+c" for copy)
        #[arg(long, default_value = "cmd+c")]
        left: String,

        /// Middle pedal action (e.g., "cmd+a" for select all)
        #[arg(long, default_value = "cmd+a")]
        middle: String,

        /// Right pedal action (e.g., "cmd+v" for paste)
        #[arg(long, default_value = "cmd+v")]
        right: String,

        /// Dry run - don't actually write to device
        #[arg(long)]
        dry_run: bool,

        /// Start monitor mode after programming to test the pedals
        #[arg(long, short = 'm')]
        monitor: bool,
    },

    /// Check if device is in programming mode
    Status,

    /// Probe device for programming protocol (reverse engineering)
    Probe,

    /// Send raw HID command to device (expert mode)
    RawCmd {
        /// Command byte (hex, e.g., "b5" for generate data)
        #[arg(long)]
        cmd: String,

        /// Additional data bytes (hex, e.g., "00010203")
        #[arg(long, default_value = "")]
        data: String,

        /// Interface number (0=keyboard, 1=mouse)
        #[arg(long, default_value = "0")]
        interface: i32,
    },
}

#[derive(Debug, Clone)]
struct KeyAction {
    modifiers: u8,
    key: u8,
}

impl KeyAction {
    fn from_string(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split('+').collect();
        let mut modifiers = 0u8;
        let mut key = 0u8;

        for (i, part) in parts.iter().enumerate() {
            let part = part.trim().to_lowercase();
            if i == parts.len() - 1 {
                // Last part is the key
                key = usb_hid::parse_key_name(&part)
                    .ok_or_else(|| anyhow!("Unknown key: {}", part))?;
            } else {
                // Modifier
                match part.as_str() {
                    "cmd" | "command" | "gui" | "meta" | "super" => {
                        modifiers |= usb_hid::MOD_LEFT_GUI;
                    }
                    "ctrl" | "control" => {
                        modifiers |= usb_hid::MOD_LEFT_CTRL;
                    }
                    "shift" => {
                        modifiers |= usb_hid::MOD_LEFT_SHIFT;
                    }
                    "alt" | "option" | "opt" => {
                        modifiers |= usb_hid::MOD_LEFT_ALT;
                    }
                    _ => return Err(anyhow!("Unknown modifier: {}", part)),
                }
            }
        }

        Ok(Self { modifiers, key })
    }
}

struct SavantElite {
    console: Console,
}

struct UsbInterfaceGuard<'a> {
    handle: &'a rusb::DeviceHandle<GlobalContext>,
    interface_num: u8,
    detached_kernel_driver: bool,
    claimed: bool,
}

impl Drop for UsbInterfaceGuard<'_> {
    fn drop(&mut self) {
        if self.claimed {
            let _ = self.handle.release_interface(self.interface_num);
        }

        if self.detached_kernel_driver {
            // Best-effort: if we detached the kernel driver, try to restore it.
            let _ = self.handle.attach_kernel_driver(self.interface_num);
        }
    }
}

impl SavantElite {
    fn new() -> Result<Self> {
        // Verify HID API can be initialized
        let _ = HidApi::new().context("Failed to initialize HID API")?;
        Ok(Self {
            console: Console::new(),
        })
    }

    fn print_banner(&self) {
        // All lines are exactly 65 characters for proper alignment
        self.console.print("");
        self.console.print(
            "[bold #ff6b6b]╔═══════════════════════════════════════════════════════════════╗[/]",
        );
        self.console.print(
            "[bold #ff6b6b]║[/][bold #4ecdc4]  ███████╗ █████╗ ██╗   ██╗ █████╗ ███╗   ██╗████████╗         [/][bold #ff6b6b]║[/]",
        );
        self.console.print(
            "[bold #ff6b6b]║[/][bold #4ecdc4]  ██╔════╝██╔══██╗██║   ██║██╔══██╗████╗  ██║╚══██╔══╝         [/][bold #ff6b6b]║[/]",
        );
        self.console.print(
            "[bold #ff6b6b]║[/][bold #4ecdc4]  ███████╗███████║██║   ██║███████║██╔██╗ ██║   ██║            [/][bold #ff6b6b]║[/]",
        );
        self.console.print(
            "[bold #ff6b6b]║[/][bold #4ecdc4]  ╚════██║██╔══██║╚██╗ ██╔╝██╔══██║██║╚██╗██║   ██║            [/][bold #ff6b6b]║[/]",
        );
        self.console.print(
            "[bold #ff6b6b]║[/][bold #4ecdc4]  ███████║██║  ██║ ╚████╔╝ ██║  ██║██║ ╚████║   ██║            [/][bold #ff6b6b]║[/]",
        );
        self.console.print(
            "[bold #ff6b6b]║[/][bold #4ecdc4]  ╚══════╝╚═╝  ╚═╝  ╚═══╝  ╚═╝  ╚═╝╚═╝  ╚═══╝   ╚═╝            [/][bold #ff6b6b]║[/]",
        );
        self.console.print(
            "[bold #ff6b6b]║[/][bold #ffe66d]              ███████╗██╗     ██╗████████╗███████╗             [/][bold #ff6b6b]║[/]",
        );
        self.console.print(
            "[bold #ff6b6b]║[/][bold #ffe66d]              ██╔════╝██║     ██║╚══██╔══╝██╔════╝             [/][bold #ff6b6b]║[/]",
        );
        self.console.print(
            "[bold #ff6b6b]║[/][bold #ffe66d]              █████╗  ██║     ██║   ██║   █████╗               [/][bold #ff6b6b]║[/]",
        );
        self.console.print(
            "[bold #ff6b6b]║[/][bold #ffe66d]              ██╔══╝  ██║     ██║   ██║   ██╔══╝               [/][bold #ff6b6b]║[/]",
        );
        self.console.print(
            "[bold #ff6b6b]║[/][bold #ffe66d]              ███████╗███████╗██║   ██║   ███████╗             [/][bold #ff6b6b]║[/]",
        );
        self.console.print(
            "[bold #ff6b6b]║[/][bold #ffe66d]              ╚══════╝╚══════╝╚═╝   ╚═╝   ╚══════╝             [/][bold #ff6b6b]║[/]",
        );
        self.console.print(
            "[bold #ff6b6b]║[/]                                                               [bold #ff6b6b]║[/]",
        );
        self.console.print(
            "[bold #ff6b6b]║[/]   [dim #95a5a6]Kinesis Foot Pedal Programmer[/] [bold #e74c3c]◆[/] [dim #95a5a6]Native macOS USB[/]            [bold #ff6b6b]║[/]",
        );
        self.console.print(
            "[bold #ff6b6b]╚═══════════════════════════════════════════════════════════════╝[/]",
        );
        self.console.print("");
    }

    fn print_pedal_visualization(&self, left: &str, middle: &str, right: &str) {
        // Helper to center text in a fixed width
        fn center(s: &str, width: usize) -> String {
            let len = s.chars().count();
            if len >= width {
                s.chars().take(width).collect()
            } else {
                let pad = width - len;
                let left_pad = pad / 2;
                let right_pad = pad - left_pad;
                format!("{}{}{}", " ".repeat(left_pad), s, " ".repeat(right_pad))
            }
        }

        // Format key action for display (e.g., "cmd+c" -> "⌘C")
        fn format_key(s: &str) -> String {
            let s = s.to_lowercase();
            let parts: Vec<&str> = s.split('+').collect();
            let mut result = String::new();

            for (i, part) in parts.iter().enumerate() {
                let part = part.trim();
                if i < parts.len() - 1 {
                    // Modifier
                    match part {
                        "cmd" | "command" | "gui" | "meta" | "super" => result.push('⌘'),
                        "ctrl" | "control" => result.push('⌃'),
                        "shift" => result.push('⇧'),
                        "alt" | "option" | "opt" => result.push('⌥'),
                        _ => result.push_str(part),
                    }
                } else {
                    // Key - uppercase for display
                    result.push_str(&part.to_uppercase());
                }
            }
            result
        }

        let left_key = format_key(left);
        let middle_key = format_key(middle);
        let right_key = format_key(right);

        // Create centered key displays (max 7 chars for the box interior)
        let left_display = center(&left_key, 7);
        let middle_display = center(&middle_key, 7);
        let right_display = center(&right_key, 7);

        self.console.print("");
        self.console.print(
            "[bold #9b59b6]┌─────────────────────────────────────────────────────────────────┐[/]",
        );
        self.console.print(
            "[bold #9b59b6]│[/]              [bold white]YOUR PEDAL CONFIGURATION[/]                        [bold #9b59b6]│[/]",
        );
        self.console.print(
            "[bold #9b59b6]└─────────────────────────────────────────────────────────────────┘[/]",
        );
        self.console.print("");

        // Top of pedals
        self.console.print(
            "       [#e74c3c]╭─────────────╮[/]  [#f39c12]╭─────────────╮[/]  [#2ecc71]╭─────────────╮[/]",
        );
        self.console.print(
            "       [#e74c3c]│[/]             [#e74c3c]│[/]  [#f39c12]│[/]             [#f39c12]│[/]  [#2ecc71]│[/]             [#2ecc71]│[/]",
        );

        // Pedal labels
        self.console.print(
            "       [#e74c3c]│[/]  [bold #e74c3c]◀ LEFT[/]    [#e74c3c]│[/]  [#f39c12]│[/]  [bold #f39c12]● MIDDLE[/]  [#f39c12]│[/]  [#2ecc71]│[/]  [bold #2ecc71]RIGHT ▶[/]   [#2ecc71]│[/]",
        );

        self.console.print(
            "       [#e74c3c]│[/]             [#e74c3c]│[/]  [#f39c12]│[/]             [#f39c12]│[/]  [#2ecc71]│[/]             [#2ecc71]│[/]",
        );

        // Key box top
        self.console.print(
            "       [#e74c3c]│[/]  [bold #e74c3c]┌───────┐[/]  [#e74c3c]│[/]  [#f39c12]│[/]  [bold #f39c12]┌───────┐[/]  [#f39c12]│[/]  [#2ecc71]│[/]  [bold #2ecc71]┌───────┐[/]  [#2ecc71]│[/]",
        );

        // Key values
        self.console.print(&format!(
            "       [#e74c3c]│[/]  [bold #e74c3c]│[/][bold white]{}[/][bold #e74c3c]│[/]  [#e74c3c]│[/]  [#f39c12]│[/]  [bold #f39c12]│[/][bold white]{}[/][bold #f39c12]│[/]  [#f39c12]│[/]  [#2ecc71]│[/]  [bold #2ecc71]│[/][bold white]{}[/][bold #2ecc71]│[/]  [#2ecc71]│[/]",
            left_display, middle_display, right_display
        ));

        // Key box bottom
        self.console.print(
            "       [#e74c3c]│[/]  [bold #e74c3c]└───────┘[/]  [#e74c3c]│[/]  [#f39c12]│[/]  [bold #f39c12]└───────┘[/]  [#f39c12]│[/]  [#2ecc71]│[/]  [bold #2ecc71]└───────┘[/]  [#2ecc71]│[/]",
        );

        self.console.print(
            "       [#e74c3c]│[/]             [#e74c3c]│[/]  [#f39c12]│[/]             [#f39c12]│[/]  [#2ecc71]│[/]             [#2ecc71]│[/]",
        );

        // Bottom of pedals
        self.console.print(
            "       [#e74c3c]╰─────────────╯[/]  [#f39c12]╰─────────────╯[/]  [#2ecc71]╰─────────────╯[/]",
        );

        // Pedal "stems"
        self.console.print(
            "            [#e74c3c]│[/]                  [#f39c12]│[/]                  [#2ecc71]│[/]",
        );
        self.console
            .print("       [dim]═════╧══════════════════╧══════════════════╧═════[/]");
        self.console.print("");
    }

    fn find_device(&self) -> Result<()> {
        let api = HidApi::new().context("Failed to initialize HID API")?;

        self.print_banner();

        // (mode, vid, pid, path, serial, interface, usage_page, usage)
        type DeviceInfo = (String, String, String, String, String, i32, u16, u16);
        let mut found_any = false;
        let mut devices_info: Vec<DeviceInfo> = Vec::new();

        for device in api.device_list() {
            if device.vendor_id() == KINESIS_VID
                && (device.product_id() == SAVANT_ELITE_PID
                    || device.product_id() == PROGRAMMING_PID)
            {
                found_any = true;
                let mode = if device.product_id() == PROGRAMMING_PID {
                    "PROGRAM".to_string()
                } else {
                    "PLAY".to_string()
                };
                devices_info.push((
                    mode,
                    format!("0x{:04X}", device.vendor_id()),
                    format!("0x{:04X}", device.product_id()),
                    device.path().to_string_lossy().to_string(),
                    device.serial_number().unwrap_or("N/A").to_string(),
                    device.interface_number(),
                    device.usage_page(),
                    device.usage(),
                ));
            }
        }

        if found_any {
            self.console.print(
                "[bold #3498db]┌─────────────────────────────────────────────────────────────────┐[/]",
            );
            self.console.print(
                "[bold #3498db]│[/]  [bold #f39c12]⚡[/] [bold white]DEVICE DETECTED[/]                                              [bold #3498db]│[/]",
            );
            self.console.print(
                "[bold #3498db]└─────────────────────────────────────────────────────────────────┘[/]",
            );
            self.console.print("");

            // Create a table for device info
            let mut table = Table::new()
                .box_style(&ROUNDED)
                .header_style(Style::parse("bold #f1c40f").unwrap_or_default())
                .border_style(Style::parse("#3498db").unwrap_or_default())
                .with_column(Column::new("Mode"))
                .with_column(Column::new("VID"))
                .with_column(Column::new("PID"))
                .with_column(Column::new("Interface"))
                .with_column(Column::new("Usage"));

            // Deduplicate by interface
            let mut seen_interfaces = std::collections::HashSet::new();
            for (mode, vid, pid, _path, _serial, iface, usage_page, usage) in &devices_info {
                if seen_interfaces.insert((*iface, *usage_page, *usage)) {
                    let mode_styled = if mode == "PROGRAM" {
                        markup::render_or_plain("[bold #e74c3c]PROGRAM[/]")
                    } else {
                        markup::render_or_plain("[bold #2ecc71]PLAY[/]")
                    };
                    let usage_str = format!("0x{:04X}:0x{:04X}", usage_page, usage);
                    table.add_row_cells([
                        mode_styled,
                        markup::render_or_plain(vid),
                        markup::render_or_plain(pid),
                        markup::render_or_plain(&iface.to_string()),
                        markup::render_or_plain(&usage_str),
                    ]);
                }
            }

            self.console.print_renderable(&table);
            self.console.print("");

            // Show USB path
            if let Some((_, _, _, path, serial, _, _, _)) = devices_info.first() {
                self.console
                    .print(&format!("  [dim]Path:[/]   [#95a5a6]{}[/]", path));
                self.console
                    .print(&format!("  [dim]Serial:[/] [#95a5a6]{}[/]", serial));
            }
        } else {
            self.console.print(
                "[bold #e74c3c]┌─────────────────────────────────────────────────────────────────┐[/]",
            );
            self.console.print(
                "[bold #e74c3c]│[/]  [bold #e74c3c]✗[/] [bold white]NO DEVICE FOUND[/]                                             [bold #e74c3c]│[/]",
            );
            self.console.print(
                "[bold #e74c3c]└─────────────────────────────────────────────────────────────────┘[/]",
            );
            self.console.print("");
            self.console
                .print("  [#95a5a6]Make sure your Savant Elite is connected via USB.[/]");
        }

        self.console.print("");
        Ok(())
    }

    fn open_keyboard_interface(&self) -> Result<HidDevice> {
        let api = HidApi::new().context("Failed to initialize HID API")?;

        // Find the keyboard interface (usage page 1, usage 6)
        for device in api.device_list() {
            if device.vendor_id() == KINESIS_VID
                && device.product_id() == SAVANT_ELITE_PID
                && device.usage_page() == 0x01
                && device.usage() == 0x06
            {
                match device.open_device(&api) {
                    Ok(dev) => return Ok(dev),
                    Err(e) => {
                        let msg = e.to_string();
                        if msg.contains("privilege violation") || msg.contains("0xE00002C1") {
                            return Err(anyhow!(e).context(
                                "Failed to open device (macOS Input Monitoring permission is required; enable it in System Settings → Privacy & Security → Input Monitoring, then re-run)",
                            ));
                        }
                        return Err(anyhow!(e).context("Failed to open device"));
                    }
                }
            }
        }

        Err(anyhow!("Savant Elite keyboard interface not found"))
    }

    fn monitor(&self, duration_secs: u64) -> Result<()> {
        let device = self.open_keyboard_interface()?;

        self.print_banner();

        self.console.print(
            "[bold #9b59b6]┌─────────────────────────────────────────────────────────────────┐[/]",
        );
        self.console.print(
            "[bold #9b59b6]│[/]  [bold #f39c12]👁[/]  [bold white]LIVE MONITOR MODE[/]                                          [bold #9b59b6]│[/]",
        );
        self.console.print(
            "[bold #9b59b6]└─────────────────────────────────────────────────────────────────┘[/]",
        );
        self.console.print("");
        self.console
            .print("  [#95a5a6]Press pedals to see what keys they send.[/]");
        self.console
            .print("  [#95a5a6]Press[/] [bold #e74c3c]Ctrl+C[/] [#95a5a6]to stop.[/]");
        if duration_secs > 0 {
            self.console.print(&format!(
                "  [#95a5a6]Auto-stop in[/] [bold #f39c12]{}[/] [#95a5a6]seconds.[/]",
                duration_secs
            ));
        }
        self.console.print("");
        self.console.print(
            "[#3498db]─────────────────────────────────────────────────────────────────────[/]",
        );

        device.set_blocking_mode(false)?;

        let mut buf = [0u8; 64];
        let mut last_report = [0u8; 8];
        let start = std::time::Instant::now();

        loop {
            if duration_secs > 0 && start.elapsed().as_secs() >= duration_secs {
                self.console.print("");
                self.console.print(
                    "[#3498db]─────────────────────────────────────────────────────────────────────[/]",
                );
                self.console
                    .print("[bold #2ecc71]✓[/] [#95a5a6]Monitoring complete.[/]");
                break;
            }

            match device.read_timeout(&mut buf, 100) {
                Ok(len) if len > 0 => {
                    let Some(report) = usb_hid::normalize_boot_keyboard_report(&buf[..len]) else {
                        continue;
                    };

                    if report != last_report {
                        last_report = report;

                        let modifiers = report[0];
                        let keys: Vec<u8> =
                            report[2..8].iter().filter(|&&k| k != 0).copied().collect();

                        if modifiers != 0 || !keys.is_empty() {
                            let mod_names = usb_hid::modifier_names(modifiers);
                            let key_names: Vec<&str> =
                                keys.iter().map(|&k| usb_hid::key_name(k)).collect();

                            let combo = if !mod_names.is_empty() && !key_names.is_empty() {
                                format!("{}+{}", mod_names.join("+"), key_names.join("+"))
                            } else if !mod_names.is_empty() {
                                mod_names.join("+")
                            } else {
                                key_names.join("+")
                            };

                            self.console.print(&format!(
                                "  [bold #2ecc71]▶[/] [bold #f1c40f]PRESS[/]   [bold white]{}[/]  [dim #7f8c8d]({})[/]",
                                combo,
                                hex::encode(report)
                            ));
                        } else {
                            self.console.print(&format!(
                                "  [dim #e74c3c]◀[/] [dim #95a5a6]RELEASE[/] [dim #7f8c8d]({})[/]",
                                hex::encode(report)
                            ));
                        }
                    }
                }
                Ok(_) => {}
                Err(e) => {
                    if e.to_string().contains("timeout") {
                        continue;
                    }
                    return Err(anyhow!("Read error: {}", e));
                }
            }

            std::thread::sleep(Duration::from_millis(10));
        }

        self.console.print("");
        Ok(())
    }

    fn status(&self) -> Result<()> {
        self.print_banner();

        self.console.print(
            "[bold #1abc9c]┌─────────────────────────────────────────────────────────────────┐[/]",
        );
        self.console.print(
            "[bold #1abc9c]│[/]  [bold #f39c12]🔍[/] [bold white]DEVICE STATUS CHECK[/]                                        [bold #1abc9c]│[/]",
        );
        self.console.print(
            "[bold #1abc9c]└─────────────────────────────────────────────────────────────────┘[/]",
        );
        self.console.print("");

        // Check via libusb first (more reliable for programming mode)
        let mut found_play_usb = false;
        let mut found_program_usb = false;
        let mut libusb_error: Option<anyhow::Error> = None;
        let mut device_details: Vec<(String, String, String)> = Vec::new();

        match rusb::devices() {
            Ok(devices) => {
                for device in devices.iter() {
                    let desc = match device.device_descriptor() {
                        Ok(desc) => desc,
                        Err(_) => continue,
                    };
                    if desc.vendor_id() == KINESIS_VID {
                        match desc.product_id() {
                            SAVANT_ELITE_PID => {
                                found_play_usb = true;
                                device_details.push((
                                    "PLAY".to_string(),
                                    format!("0x{:04X}", SAVANT_ELITE_PID),
                                    format!(
                                        "Bus {:03} Device {:03}",
                                        device.bus_number(),
                                        device.address()
                                    ),
                                ));
                            }
                            PROGRAMMING_PID => {
                                found_program_usb = true;
                                let mut product = "Savant Elite".to_string();
                                let mut mfr = "Kinesis".to_string();

                                if let Ok(handle) = device.open() {
                                    if let Ok(langs) =
                                        handle.read_languages(Duration::from_millis(100))
                                    {
                                        if let Some(lang) = langs.first() {
                                            if let Ok(p) = handle.read_product_string(
                                                *lang,
                                                &desc,
                                                Duration::from_millis(100),
                                            ) {
                                                product = p;
                                            }
                                            if let Ok(m) = handle.read_manufacturer_string(
                                                *lang,
                                                &desc,
                                                Duration::from_millis(100),
                                            ) {
                                                mfr = m;
                                            }
                                        }
                                    }
                                }
                                device_details.push((
                                    "PROGRAM".to_string(),
                                    format!("0x{:04X}", PROGRAMMING_PID),
                                    format!("{mfr} - {product}"),
                                ));
                            }
                            _ => {}
                        }
                    }
                }
            }
            Err(e) => {
                libusb_error =
                    Some(anyhow!(e).context("Failed to enumerate USB devices via libusb"));
            }
        }

        // Also check HID (for play mode with interfaces)
        let api = HidApi::new().context("Failed to initialize HID API")?;
        let mut found_play_hid = false;
        let mut found_program_hid = false;

        for device_info in api.device_list() {
            if device_info.vendor_id() != KINESIS_VID {
                continue;
            }

            match device_info.product_id() {
                SAVANT_ELITE_PID => {
                    if !found_play_usb && !found_play_hid {
                        found_play_hid = true;
                        device_details.push((
                            "PLAY".to_string(),
                            format!("0x{:04X}", SAVANT_ELITE_PID),
                            format!("hidapi: {}", device_info.path().to_string_lossy()),
                        ));
                    }
                }
                PROGRAMMING_PID => {
                    if !found_program_usb && !found_program_hid {
                        found_program_hid = true;
                        device_details.push((
                            "PROGRAM".to_string(),
                            format!("0x{:04X}", PROGRAMMING_PID),
                            format!("hidapi: {}", device_info.path().to_string_lossy()),
                        ));
                    }
                }
                _ => {}
            }
        }

        let found_play = found_play_usb || found_play_hid;
        let found_program = found_program_usb || found_program_hid;

        if !found_play && !found_program {
            self.console.print(
                "  [bold #e74c3c]╭────────────────────────────────────────────────────────────╮[/]",
            );
            self.console.print(
                "  [bold #e74c3c]│[/]  [bold #e74c3c]✗[/]  [bold white]No Savant Elite device found[/]                          [bold #e74c3c]│[/]",
            );
            self.console.print(
                "  [bold #e74c3c]╰────────────────────────────────────────────────────────────╯[/]",
            );
            self.console.print("");
            self.console.print("  [bold #f39c12]Troubleshooting:[/]");
            self.console
                .print("    [#95a5a6]1.[/] Make sure the device is connected via USB");
            self.console
                .print("    [#95a5a6]2.[/] Try unplugging and replugging the cable");
        } else if found_play && !found_program {
            self.console.print(
                "  [bold #2ecc71]╭────────────────────────────────────────────────────────────╮[/]",
            );
            self.console.print(
                "  [bold #2ecc71]│[/]  [bold #2ecc71]●[/]  [bold white]Device is in[/] [bold #2ecc71]PLAY[/] [bold white]mode[/]                              [bold #2ecc71]│[/]",
            );
            self.console.print(
                "  [bold #2ecc71]╰────────────────────────────────────────────────────────────╯[/]",
            );
            self.console.print("");

            // Show table of details
            let mut table = Table::new()
                .box_style(&ROUNDED)
                .header_style(Style::parse("bold #f1c40f").unwrap_or_default())
                .border_style(Style::parse("#3498db").unwrap_or_default())
                .with_column(Column::new("Mode"))
                .with_column(Column::new("PID"))
                .with_column(Column::new("Location"));

            for (mode, pid, location) in &device_details {
                let mode_styled = if mode == "PROGRAM" {
                    markup::render_or_plain(&format!("[bold #e74c3c]{}[/]", mode))
                } else {
                    markup::render_or_plain(&format!("[bold #2ecc71]{}[/]", mode))
                };
                table.add_row_cells([
                    mode_styled,
                    markup::render_or_plain(pid),
                    markup::render_or_plain(location),
                ]);
            }
            self.console.print_renderable(&table);
            self.console.print("");

            self.console
                .print("  [bold #f39c12]To program the device, switch to PROGRAMMING mode:[/]");
            self.console.print("");
            self.console
                .print("    [bold #3498db]1.[/] Flip the pedal over");
            self.console
                .print("    [bold #3498db]2.[/] Find the recessed switch near the Kinesis sticker");
            self.console.print(
                "    [bold #3498db]3.[/] Use a paperclip to flip it from [#2ecc71]Play[/] → [#e74c3c]Program[/]",
            );
            self.console
                .print("    [bold #3498db]4.[/] Unplug and replug the USB cable");
            self.console
                .print("    [bold #3498db]5.[/] Run [bold #f1c40f]savant status[/] to verify");
        } else if found_program {
            self.console.print(
                "  [bold #e74c3c]╭────────────────────────────────────────────────────────────╮[/]",
            );
            self.console.print(
                "  [bold #e74c3c]│[/]  [bold #e74c3c]◉[/]  [bold white]Device is in[/] [bold #e74c3c]PROGRAMMING[/] [bold white]mode[/]                      [bold #e74c3c]│[/]",
            );
            self.console.print(
                "  [bold #e74c3c]╰────────────────────────────────────────────────────────────╯[/]",
            );
            self.console.print("");

            // Show table of details
            let mut table = Table::new()
                .box_style(&ROUNDED)
                .header_style(Style::parse("bold #f1c40f").unwrap_or_default())
                .border_style(Style::parse("#e74c3c").unwrap_or_default())
                .with_column(Column::new("Mode"))
                .with_column(Column::new("PID"))
                .with_column(Column::new("Info"));

            for (mode, pid, info) in &device_details {
                let mode_styled = if mode == "PROGRAM" {
                    markup::render_or_plain(&format!("[bold #e74c3c]{}[/]", mode))
                } else {
                    markup::render_or_plain(&format!("[bold #2ecc71]{}[/]", mode))
                };
                table.add_row_cells([
                    mode_styled,
                    markup::render_or_plain(pid),
                    markup::render_or_plain(info),
                ]);
            }
            self.console.print_renderable(&table);
            self.console.print("");

            self.console
                .print("  [bold #2ecc71]✓[/] [bold white]Ready to program![/]");
            self.console.print("");
            self.console.print("  [#95a5a6]Example command:[/]");
            self.console.print(
                "    [bold #f1c40f]savant program --left 'cmd+c' --middle 'cmd+a' --right 'cmd+v'[/]",
            );
        }

        if let Some(e) = libusb_error {
            self.console.print("");
            self.console.print(&format!(
                "  [bold #f39c12]⚠[/] [#f39c12]Note:[/] [#95a5a6]libusb scan failed[/] [dim]({})[/]",
                e
            ));
            self.console.print(
                "  [#95a5a6]If you need to program or reliably detect programming mode, try running with sudo.[/]",
            );
        }

        self.console.print("");
        Ok(())
    }

    fn probe(&self) -> Result<()> {
        self.print_banner();

        self.console.print(
            "[bold #9b59b6]┌─────────────────────────────────────────────────────────────────┐[/]",
        );
        self.console.print(
            "[bold #9b59b6]│[/]  [bold #f39c12]🔬[/] [bold white]PROTOCOL PROBE[/] [dim](Reverse Engineering Mode)[/]             [bold #9b59b6]│[/]",
        );
        self.console.print(
            "[bold #9b59b6]└─────────────────────────────────────────────────────────────────┘[/]",
        );
        self.console.print("");

        let api = HidApi::new()?;

        self.console
            .print("  [bold #3498db]Scanning for Kinesis devices...[/]");
        self.console.print("");

        for device_info in api.device_list() {
            if device_info.vendor_id() == KINESIS_VID {
                let pid = device_info.product_id();
                let mode_indicator = if pid == PROGRAMMING_PID {
                    "[bold #e74c3c]★ PROGRAMMING MODE[/]"
                } else {
                    "[bold #2ecc71]● PLAY MODE[/]"
                };

                self.console.print(&format!(
                    "  [bold #f1c40f]►[/] Device [bold white]VID=0x{:04X} PID=0x{:04X}[/]  {}",
                    device_info.vendor_id(),
                    pid,
                    mode_indicator
                ));
                self.console.print(&format!(
                    "    [dim]Path:[/] [#7f8c8d]{}[/]",
                    device_info.path().to_string_lossy()
                ));
                self.console.print(&format!(
                    "    [dim]Interface:[/] [#7f8c8d]{}[/]  [dim]Usage:[/] [#7f8c8d]0x{:04X}:0x{:04X}[/]",
                    device_info.interface_number(),
                    device_info.usage_page(),
                    device_info.usage()
                ));

                match device_info.open_device(&api) {
                    Ok(device) => {
                        self.console
                            .print("    [bold #2ecc71]✓[/] Opened successfully");

                        // Try PI Engineering X-keys commands
                        let commands = [
                            (0xB5, "Generate Data", "#3498db"),
                            (0xC1, "Get Descriptor", "#9b59b6"),
                            (0xCD, "Get Key Macro", "#1abc9c"),
                        ];

                        for (cmd, name, color) in commands {
                            let mut cmd_buf = [0u8; 36];
                            cmd_buf[0] = 0;
                            cmd_buf[1] = cmd;

                            match device.write(&cmd_buf) {
                                Ok(n) => {
                                    self.console.print(&format!(
                                        "    [{}]→[/] {} [dim](0x{:02X})[/]: {} bytes",
                                        color, name, cmd, n
                                    ));

                                    std::thread::sleep(Duration::from_millis(50));
                                    let mut response = [0u8; 64];
                                    match device.read_timeout(&mut response, 200) {
                                        Ok(len) if len > 0 => {
                                            self.console.print(&format!(
                                                "      [bold #2ecc71]←[/] [#7f8c8d]{}[/]",
                                                hex::encode(&response[..len])
                                            ));
                                        }
                                        _ => {
                                            self.console.print("      [dim]← No response[/]");
                                        }
                                    }
                                }
                                Err(e) => {
                                    self.console.print(&format!(
                                        "    [#e74c3c]✗[/] {} failed: [dim]{}[/]",
                                        name, e
                                    ));
                                }
                            }
                        }

                        // Try feature reports
                        self.console
                            .print("    [bold #f39c12]Checking feature reports...[/]");
                        for report_id in 0..10u8 {
                            let mut buf = [0u8; 65];
                            buf[0] = report_id;
                            match device.get_feature_report(&mut buf) {
                                Ok(len) if len > 0 => {
                                    self.console.print(&format!(
                                        "      [#2ecc71]Report {}:[/] [#7f8c8d]{}[/]",
                                        report_id,
                                        hex::encode(&buf[..len])
                                    ));
                                }
                                _ => {}
                            }
                        }
                    }
                    Err(e) => {
                        self.console
                            .print(&format!("    [bold #e74c3c]✗[/] Failed to open: {}", e));
                    }
                }
                self.console.print("");
            }
        }

        self.console.print(
            "[#3498db]─────────────────────────────────────────────────────────────────────[/]",
        );
        self.console.print("[bold #f39c12]Protocol Notes:[/]");
        self.console.print(
            "  [#95a5a6]•[/] Programming mode uses PID [bold]0x0232[/] (vs normal [bold]0x030C[/])",
        );
        self.console.print("");
        self.console
            .print("[bold #f39c12]To Enter Programming Mode:[/]");
        self.console
            .print("  [bold #3498db]1.[/] Flip the pedal over");
        self.console
            .print("  [bold #3498db]2.[/] Look for a recessed switch near the Kinesis sticker");
        self.console.print(
            "  [bold #3498db]3.[/] Use a paperclip to flip it from [#2ecc71]Play[/] → [#e74c3c]Program[/]",
        );
        self.console.print(
            "  [bold #3498db]4.[/] Unplug and replug USB, then run [bold #f1c40f]savant status[/]",
        );
        self.console.print("");

        Ok(())
    }

    fn raw_cmd(&self, cmd: &str, data: &str, interface: i32) -> Result<()> {
        self.console.print("");
        self.console.print(
            "[bold #e74c3c]┌─────────────────────────────────────────────────────────────────┐[/]",
        );
        self.console.print(
            "[bold #e74c3c]│[/]  [bold #f39c12]⚠[/]  [bold white]RAW COMMAND MODE[/] [dim](Expert Only)[/]                        [bold #e74c3c]│[/]",
        );
        self.console.print(
            "[bold #e74c3c]└─────────────────────────────────────────────────────────────────┘[/]",
        );
        self.console.print("");

        let cmd_byte =
            u8::from_str_radix(cmd, 16).context("Invalid command byte (use hex, e.g., 'b5')")?;

        let data_bytes: Vec<u8> = if data.is_empty() {
            vec![]
        } else {
            hex::decode(data).context("Invalid data bytes (use hex)")?
        };

        let api = HidApi::new().context("Failed to initialize HID API")?;

        let mut found = false;
        for device_info in api.device_list() {
            if device_info.vendor_id() == KINESIS_VID
                && (device_info.product_id() == SAVANT_ELITE_PID
                    || device_info.product_id() == PROGRAMMING_PID)
                && device_info.interface_number() == interface
            {
                found = true;
                let device = device_info
                    .open_device(&api)
                    .context("Failed to open device")?;

                let mut cmd_buf = [0u8; 36];
                cmd_buf[0] = 0;
                cmd_buf[1] = cmd_byte;
                for (i, b) in data_bytes.iter().enumerate() {
                    if i + 2 < cmd_buf.len() {
                        cmd_buf[i + 2] = *b;
                    }
                }

                self.console.print(&format!(
                    "  [bold #3498db]→[/] Sending: [bold #f1c40f]{}[/]",
                    hex::encode(&cmd_buf[..8])
                ));

                let n = device.write(&cmd_buf).context("Write error")?;
                self.console
                    .print(&format!("  [bold #2ecc71]✓[/] Sent [bold]{}[/] bytes", n));

                std::thread::sleep(Duration::from_millis(100));
                let mut response = [0u8; 64];
                match device.read_timeout(&mut response, 500) {
                    Ok(len) if len > 0 => {
                        self.console.print(&format!(
                            "  [bold #2ecc71]←[/] Response ([bold]{}[/] bytes): [#7f8c8d]{}[/]",
                            len,
                            hex::encode(&response[..len])
                        ));
                    }
                    Ok(_) => {
                        self.console.print("  [dim]← No response[/]");
                    }
                    Err(e) => {
                        self.console
                            .print(&format!("  [#e74c3c]✗[/] Read error: {}", e));
                    }
                }

                break;
            }
        }

        if !found {
            return Err(anyhow!(
                "No Savant Elite device found for interface {} (try `savant info` or `savant status`)",
                interface
            ));
        }

        self.console.print("");
        Ok(())
    }

    fn program(
        &self,
        left: &str,
        middle: &str,
        right: &str,
        dry_run: bool,
        auto_monitor: bool,
    ) -> Result<()> {
        self.print_banner();

        self.console.print(
            "[bold #2ecc71]┌─────────────────────────────────────────────────────────────────┐[/]",
        );
        self.console.print(
            "[bold #2ecc71]│[/]  [bold #f39c12]⚡[/] [bold white]PEDAL PROGRAMMING[/]                                           [bold #2ecc71]│[/]",
        );
        self.console.print(
            "[bold #2ecc71]└─────────────────────────────────────────────────────────────────┘[/]",
        );
        self.console.print("");

        // Check if device is in programming mode using libusb
        let mut programming_device: Option<Device<GlobalContext>> = None;
        let mut play_mode_found = false;

        let devices = rusb::devices()
            .context("Failed to list USB devices via libusb (try running with sudo)")?;
        for device in devices.iter() {
            let desc = match device.device_descriptor() {
                Ok(desc) => desc,
                Err(_) => continue,
            };
            if desc.vendor_id() == KINESIS_VID {
                match desc.product_id() {
                    PROGRAMMING_PID => {
                        programming_device = Some(device);
                        break;
                    }
                    SAVANT_ELITE_PID => {
                        play_mode_found = true;
                    }
                    _ => {}
                }
            }
        }

        if programming_device.is_none() {
            // Show what would be programmed (preview)
            self.print_pedal_visualization(left, middle, right);

            if play_mode_found {
                self.console.print(
                    "  [bold #e74c3c]╭────────────────────────────────────────────────────────────╮[/]",
                );
                self.console.print(
                    "  [bold #e74c3c]│[/]  [bold #e74c3c]✗[/]  [bold white]Device is in PLAY mode, not PROGRAMMING mode[/]      [bold #e74c3c]│[/]",
                );
                self.console.print(
                    "  [bold #e74c3c]╰────────────────────────────────────────────────────────────╯[/]",
                );
                self.console.print("");
                self.console
                    .print("  [bold #f39c12]To program these keys, enter programming mode:[/]");
                self.console
                    .print("    [bold #3498db]1.[/] Flip the pedal over");
                self.console.print(
                    "    [bold #3498db]2.[/] Find the recessed switch near the Kinesis sticker",
                );
                self.console.print(
                    "    [bold #3498db]3.[/] Use a paperclip to flip it from [#2ecc71]Play[/] → [#e74c3c]Program[/]",
                );
                self.console
                    .print("    [bold #3498db]4.[/] Unplug and replug the USB cable");
                self.console
                    .print("    [bold #3498db]5.[/] Run [bold #f1c40f]savant program[/] again");
            } else {
                self.console.print(
                    "  [bold #e74c3c]✗[/] Savant Elite not found. Make sure it's connected.",
                );
            }
            self.console.print("");
            return Ok(());
        }

        let device = programming_device.unwrap();

        // Parse key actions
        let left_action = KeyAction::from_string(left)?;
        let middle_action = KeyAction::from_string(middle)?;
        let right_action = KeyAction::from_string(right)?;

        // Show configuration table
        self.console
            .print("  [bold #f39c12]Configuration to program:[/]");
        self.console.print("");

        let mut config_table = Table::new()
            .box_style(&ROUNDED)
            .header_style(Style::parse("bold #f1c40f").unwrap_or_default())
            .border_style(Style::parse("#3498db").unwrap_or_default())
            .with_column(Column::new("Pedal"))
            .with_column(Column::new("Action"))
            .with_column(Column::new("Modifier"))
            .with_column(Column::new("Key Code"));

        let left_mod = format!("0x{:02X}", left_action.modifiers);
        let left_key = format!("0x{:02X}", left_action.key);
        let middle_mod = format!("0x{:02X}", middle_action.modifiers);
        let middle_key = format!("0x{:02X}", middle_action.key);
        let right_mod = format!("0x{:02X}", right_action.modifiers);
        let right_key = format!("0x{:02X}", right_action.key);

        config_table.add_row_cells([
            markup::render_or_plain("[bold #e74c3c]◀ LEFT[/]"),
            markup::render_or_plain(left),
            markup::render_or_plain(&left_mod),
            markup::render_or_plain(&left_key),
        ]);
        config_table.add_row_cells([
            markup::render_or_plain("[bold #f39c12]● MIDDLE[/]"),
            markup::render_or_plain(middle),
            markup::render_or_plain(&middle_mod),
            markup::render_or_plain(&middle_key),
        ]);
        config_table.add_row_cells([
            markup::render_or_plain("[bold #2ecc71]▶ RIGHT[/]"),
            markup::render_or_plain(right),
            markup::render_or_plain(&right_mod),
            markup::render_or_plain(&right_key),
        ]);

        self.console.print_renderable(&config_table);
        self.console.print("");

        if dry_run {
            // Show visual pedal configuration preview
            self.print_pedal_visualization(left, middle, right);

            self.console.print(
                "  [bold #f39c12]╭────────────────────────────────────────────────────────────╮[/]",
            );
            self.console.print(
                "  [bold #f39c12]│[/]  [bold #f39c12]⚠[/]  [bold white]DRY RUN - No changes will be made[/]                   [bold #f39c12]│[/]",
            );
            self.console.print(
                "  [bold #f39c12]╰────────────────────────────────────────────────────────────╯[/]",
            );
            self.console.print("");
            self.console
                .print("  [#95a5a6]Would send the following commands:[/]");
            self.console.print(&format!(
                "    [#3498db]→[/] SET_KEY_MACRO (0xCC) for pedal 0: mod=0x{:02X}, key=0x{:02X}",
                left_action.modifiers, left_action.key
            ));
            self.console.print(&format!(
                "    [#3498db]→[/] SET_KEY_MACRO (0xCC) for pedal 1: mod=0x{:02X}, key=0x{:02X}",
                middle_action.modifiers, middle_action.key
            ));
            self.console.print(&format!(
                "    [#3498db]→[/] SET_KEY_MACRO (0xCC) for pedal 2: mod=0x{:02X}, key=0x{:02X}",
                right_action.modifiers, right_action.key
            ));
            self.console
                .print("    [#3498db]→[/] SAVE_TO_EEPROM (0xCE)");
            self.console.print("");
            return Ok(());
        }

        // Open device
        let handle = device
            .open()
            .context("Failed to open device (try running with sudo)")?;

        // Get device config to find endpoints
        let config = device
            .active_config_descriptor()
            .or_else(|_| device.config_descriptor(0))
            .context("Failed to read USB configuration descriptor")?;
        self.console.print(&format!(
            "  [#95a5a6]Device has[/] [bold]{}[/] [#95a5a6]interface(s)[/]",
            config.num_interfaces()
        ));

        // Try to claim interface 0
        let interface_num = 0;
        let mut detached_kernel_driver = false;
        if handle.kernel_driver_active(interface_num).unwrap_or(false) {
            self.console.print(&format!(
                "  [#f39c12]→[/] Detaching kernel driver from interface {}...",
                interface_num
            ));
            handle.detach_kernel_driver(interface_num)?;
            detached_kernel_driver = true;
        }

        let mut interface_guard = UsbInterfaceGuard {
            handle: &handle,
            interface_num,
            detached_kernel_driver,
            claimed: false,
        };

        handle
            .claim_interface(interface_num)
            .context("Failed to claim interface - do you have permission?")?;
        interface_guard.claimed = true;

        self.console.print(&format!(
            "  [bold #2ecc71]✓[/] Claimed interface [bold]{}[/]",
            interface_num
        ));
        self.console.print("");

        // Log endpoint information for debugging
        self.console
            .print("  [bold #9b59b6]Endpoint Information:[/]");
        for interface in config.interfaces() {
            for desc in interface.descriptors() {
                self.console.print(&format!(
                    "    [dim]Interface {}:[/] class={} subclass={} protocol={}",
                    desc.interface_number(),
                    desc.class_code(),
                    desc.sub_class_code(),
                    desc.protocol_code()
                ));
                for ep in desc.endpoint_descriptors() {
                    let dir = match ep.direction() {
                        rusb::Direction::Out => "[#e74c3c]OUT[/]",
                        rusb::Direction::In => "[#2ecc71]IN[/]",
                    };
                    let transfer = match ep.transfer_type() {
                        rusb::TransferType::Control => "Control",
                        rusb::TransferType::Isochronous => "Isochronous",
                        rusb::TransferType::Bulk => "Bulk",
                        rusb::TransferType::Interrupt => "Interrupt",
                    };
                    self.console.print(&format!(
                        "      [#7f8c8d]Endpoint 0x{:02X}:[/] {} {} [dim](max: {})[/]",
                        ep.address(),
                        dir,
                        transfer,
                        ep.max_packet_size()
                    ));
                }
            }
        }
        self.console.print("");

        // Program each pedal using HID SET_REPORT
        let pedals = [
            (xkeys_protocol::PEDAL_LEFT, &left_action, "Left", "#e74c3c"),
            (
                xkeys_protocol::PEDAL_MIDDLE,
                &middle_action,
                "Middle",
                "#f39c12",
            ),
            (
                xkeys_protocol::PEDAL_RIGHT,
                &right_action,
                "Right",
                "#2ecc71",
            ),
        ];

        self.console.print(
            "[#3498db]─────────────────────────────────────────────────────────────────────[/]",
        );
        self.console.print("");

        let mut pedal_failures: Vec<&str> = Vec::new();
        for (pedal_idx, action, name, color) in pedals {
            self.console.print(&format!(
                "  [bold {}]▸[/] Programming [bold white]{}[/] pedal...",
                color, name
            ));

            // Try multiple data formats and transfer methods
            let mut success = false;
            let mut success_method = "";

            // Format 1: Command as first byte, pedal, mods, key
            let cmd1 = [
                xkeys_protocol::CMD_SET_KEY_MACRO,
                pedal_idx,
                action.modifiers,
                action.key,
                0,
                0,
                0,
                0,
            ];

            // Format 2: Report ID 0, then command in data
            let cmd2 = [
                0u8,
                xkeys_protocol::CMD_SET_KEY_MACRO,
                pedal_idx,
                action.modifiers,
                action.key,
                0,
                0,
                0,
            ];

            // Format 3: Report ID conveys the command, payload is pedal+mod+key.
            let cmd3_payload = [pedal_idx, action.modifiers, action.key, 0, 0, 0, 0, 0];

            // Try SET_REPORT with multiple (report-id, layout) combinations.
            //
            // Different firmware revisions appear to expect one of:
            // - report_id = 0 with the command as the first data byte (no leading report-id byte),
            // - report_id = 0 with a leading 0 report-id byte (hidapi-style),
            // - report_id = CMD with either data starting at CMD or with a compact payload.
            for (fmt_name, w_value, data) in [
                ("feat-rid0-cmd", 0x0300, &cmd1[..]),
                ("feat-rid0-prefix", 0x0300, &cmd2[..]),
                (
                    "feat-ridcmd",
                    0x0300 | (xkeys_protocol::CMD_SET_KEY_MACRO as u16),
                    &cmd1[..],
                ),
                (
                    "feat-ridcmd-payload",
                    0x0300 | (xkeys_protocol::CMD_SET_KEY_MACRO as u16),
                    &cmd3_payload[..],
                ),
                ("out-rid0-cmd", 0x0200, &cmd1[..]),
                ("out-rid0-prefix", 0x0200, &cmd2[..]),
                (
                    "out-ridcmd",
                    0x0200 | (xkeys_protocol::CMD_SET_KEY_MACRO as u16),
                    &cmd1[..],
                ),
                (
                    "out-ridcmd-payload",
                    0x0200 | (xkeys_protocol::CMD_SET_KEY_MACRO as u16),
                    &cmd3_payload[..],
                ),
            ] {
                let result = handle.write_control(
                    0x21,
                    0x09,
                    w_value,
                    interface_num as u16,
                    data,
                    Duration::from_millis(500),
                );
                if result.is_ok() {
                    success = true;
                    success_method = fmt_name;
                    break;
                }
            }

            // Try with longer buffer (36 bytes like PI Engineering)
            if !success {
                let mut long_prefixed = [0u8; 36];
                long_prefixed[0] = 0;
                long_prefixed[1] = xkeys_protocol::CMD_SET_KEY_MACRO;
                long_prefixed[2] = pedal_idx;
                long_prefixed[3] = action.modifiers;
                long_prefixed[4] = action.key;

                let mut long_unprefixed = [0u8; 36];
                long_unprefixed[0] = xkeys_protocol::CMD_SET_KEY_MACRO;
                long_unprefixed[1] = pedal_idx;
                long_unprefixed[2] = action.modifiers;
                long_unprefixed[3] = action.key;

                for (fmt_name, w_value, data) in [
                    ("36b-out-prefix", 0x0200, &long_prefixed[..]),
                    ("36b-out-cmd", 0x0200, &long_unprefixed[..]),
                    ("36b-feat-prefix", 0x0300, &long_prefixed[..]),
                    ("36b-feat-cmd", 0x0300, &long_unprefixed[..]),
                ] {
                    let result = handle.write_control(
                        0x21,
                        0x09,
                        w_value,
                        interface_num as u16,
                        data,
                        Duration::from_millis(500),
                    );
                    if result.is_ok() {
                        success = true;
                        success_method = fmt_name;
                        break;
                    }
                }
            }

            // Try vendor-specific request
            if !success {
                let result = handle.write_control(
                    0x40,
                    xkeys_protocol::CMD_SET_KEY_MACRO,
                    ((action.key as u16) << 8) | (action.modifiers as u16),
                    pedal_idx as u16,
                    &[],
                    Duration::from_millis(500),
                );
                if result.is_ok() {
                    success = true;
                    success_method = "vendor";
                }
            }

            if success {
                self.console.print(&format!(
                    "    [bold #2ecc71]✓[/] [#95a5a6]Success[/] [dim]({})[/]",
                    success_method
                ));
            } else {
                self.console
                    .print("    [bold #e74c3c]✗[/] [#e74c3c]Failed[/]");
                pedal_failures.push(name);
            }

            std::thread::sleep(Duration::from_millis(50));
        }

        self.console.print("");

        // Save to EEPROM
        self.console
            .print("  [bold #f1c40f]▸[/] Saving to EEPROM...");
        let save_cmd = [xkeys_protocol::CMD_SAVE_TO_EEPROM, 0, 0, 0, 0, 0, 0, 0];
        let save_alt = [0u8, xkeys_protocol::CMD_SAVE_TO_EEPROM, 0, 0, 0, 0, 0, 0];
        let save_payload = [0u8; 8];
        let mut save_success = false;

        for (_fmt_name, w_value, data, timeout_ms) in [
            ("out-rid0-cmd", 0x0200, &save_cmd[..], 1000),
            ("out-rid0-prefix", 0x0200, &save_alt[..], 500),
            (
                "out-ridcmd",
                0x0200 | (xkeys_protocol::CMD_SAVE_TO_EEPROM as u16),
                &save_cmd[..],
                500,
            ),
            (
                "out-ridcmd-payload",
                0x0200 | (xkeys_protocol::CMD_SAVE_TO_EEPROM as u16),
                &save_payload[..],
                500,
            ),
            ("feat-rid0-cmd", 0x0300, &save_cmd[..], 500),
            ("feat-rid0-prefix", 0x0300, &save_alt[..], 500),
            (
                "feat-ridcmd",
                0x0300 | (xkeys_protocol::CMD_SAVE_TO_EEPROM as u16),
                &save_cmd[..],
                500,
            ),
            (
                "feat-ridcmd-payload",
                0x0300 | (xkeys_protocol::CMD_SAVE_TO_EEPROM as u16),
                &save_payload[..],
                500,
            ),
        ] {
            let result = handle.write_control(
                0x21,
                0x09,
                w_value,
                interface_num as u16,
                data,
                Duration::from_millis(timeout_ms),
            );
            if result.is_ok() {
                save_success = true;
                break;
            }
        }

        if !save_success {
            // Try with longer buffer (36 bytes like PI Engineering)
            let mut long_prefixed = [0u8; 36];
            long_prefixed[0] = 0;
            long_prefixed[1] = xkeys_protocol::CMD_SAVE_TO_EEPROM;

            let mut long_unprefixed = [0u8; 36];
            long_unprefixed[0] = xkeys_protocol::CMD_SAVE_TO_EEPROM;

            for (w_value, data) in [
                (0x0200, &long_prefixed[..]),
                (0x0200, &long_unprefixed[..]),
                (0x0300, &long_prefixed[..]),
                (0x0300, &long_unprefixed[..]),
            ] {
                let result = handle.write_control(
                    0x21,
                    0x09,
                    w_value,
                    interface_num as u16,
                    data,
                    Duration::from_millis(500),
                );
                if result.is_ok() {
                    save_success = true;
                    break;
                }
            }
        }

        if save_success {
            std::thread::sleep(Duration::from_millis(200));
            self.console
                .print("    [bold #2ecc71]✓[/] [#95a5a6]EEPROM saved[/]");
        } else {
            self.console.print(
                "    [bold #f39c12]⚠[/] [#f39c12]Save command may have failed, but programming was done[/]",
            );
        }

        self.console.print("");
        self.console.print(
            "[#3498db]─────────────────────────────────────────────────────────────────────[/]",
        );
        self.console.print("");
        if pedal_failures.is_empty() && save_success {
            self.console.print(
                "  [bold #2ecc71]╭────────────────────────────────────────────────────────────╮[/]",
            );
            self.console.print(
                "  [bold #2ecc71]│[/]  [bold #2ecc71]✓[/]  [bold white]PROGRAMMING COMPLETE![/]                                 [bold #2ecc71]│[/]",
            );
            self.console.print(
                "  [bold #2ecc71]╰────────────────────────────────────────────────────────────╯[/]",
            );
        } else {
            self.console.print(
                "  [bold #f39c12]╭────────────────────────────────────────────────────────────╮[/]",
            );
            self.console.print(
                "  [bold #f39c12]│[/]  [bold #f39c12]⚠[/]  [bold white]PROGRAMMING FINISHED WITH WARNINGS[/]                   [bold #f39c12]│[/]",
            );
            self.console.print(
                "  [bold #f39c12]╰────────────────────────────────────────────────────────────╯[/]",
            );
            if !pedal_failures.is_empty() {
                self.console.print(&format!(
                    "  [#95a5a6]Failed pedals:[/] [bold #e74c3c]{}[/]",
                    pedal_failures.join(", ")
                ));
            }
            if !save_success {
                self.console.print(
                    "  [#95a5a6]EEPROM save may have failed; changes might not persist after unplug.[/]",
                );
            }
        }
        // Show visual pedal configuration
        self.print_pedal_visualization(left, middle, right);

        self.console
            .print("  [bold #f39c12]To use the new configuration:[/]");
        self.console
            .print("    [bold #3498db]1.[/] Flip the switch back to [bold #2ecc71]Play[/] mode");
        self.console
            .print("    [bold #3498db]2.[/] Unplug and replug the USB cable");
        self.console
            .print("    [bold #3498db]3.[/] Your pedals should now send the programmed keys!");
        self.console.print("");

        if auto_monitor {
            self.console.print(
                "[bold #9b59b6]┌─────────────────────────────────────────────────────────────────┐[/]",
            );
            self.console.print(
                "[bold #9b59b6]│[/]  [bold #f39c12]👁[/]  [bold white]STARTING MONITOR MODE[/] [dim](test your pedals!)[/]            [bold #9b59b6]│[/]",
            );
            self.console.print(
                "[bold #9b59b6]└─────────────────────────────────────────────────────────────────┘[/]",
            );
            self.console.print("");
            self.console
                .print("  [#95a5a6]Switch to Play mode, replug USB, then press pedals to test.[/]");
            self.console.print(
                "  [#95a5a6]Press[/] [bold #e74c3c]Ctrl+C[/] [#95a5a6]to stop monitoring.[/]",
            );
            self.console.print("");

            // Run monitor indefinitely (user presses Ctrl+C to stop)
            self.monitor(0)?;
        }

        Ok(())
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let savant = SavantElite::new()?;

    match cli.command {
        Commands::Info => {
            savant.find_device()?;
        }
        Commands::Monitor { duration } => {
            savant.monitor(duration)?;
        }
        Commands::Probe => {
            savant.probe()?;
        }
        Commands::RawCmd {
            cmd,
            data,
            interface,
        } => {
            savant.raw_cmd(&cmd, &data, interface)?;
        }
        Commands::Status => {
            savant.status()?;
        }
        Commands::Program {
            left,
            middle,
            right,
            dry_run,
            monitor,
        } => {
            savant.program(&left, &middle, &right, dry_run, monitor)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_key_action_cmd_c() {
        let action = KeyAction::from_string("cmd+c").unwrap();
        assert_eq!(action.modifiers, usb_hid::MOD_LEFT_GUI);
        assert_eq!(action.key, usb_hid::KEY_C);
    }

    #[test]
    fn parse_key_action_multi_modifiers() {
        let action = KeyAction::from_string("ctrl+shift+alt+f12").unwrap();
        assert_eq!(
            action.modifiers,
            usb_hid::MOD_LEFT_CTRL | usb_hid::MOD_LEFT_SHIFT | usb_hid::MOD_LEFT_ALT
        );
        assert_eq!(action.key, usb_hid::KEY_F12);
    }

    #[test]
    fn parse_key_action_aliases() {
        let a1 = KeyAction::from_string("option+a").unwrap();
        let a2 = KeyAction::from_string("opt+a").unwrap();
        let a3 = KeyAction::from_string("alt+a").unwrap();

        assert_eq!(a1.modifiers, usb_hid::MOD_LEFT_ALT);
        assert_eq!(a2.modifiers, usb_hid::MOD_LEFT_ALT);
        assert_eq!(a3.modifiers, usb_hid::MOD_LEFT_ALT);

        assert_eq!(a1.key, usb_hid::KEY_A);
        assert_eq!(a2.key, usb_hid::KEY_A);
        assert_eq!(a3.key, usb_hid::KEY_A);
    }

    #[test]
    fn parse_key_action_rejects_unknown_modifier() {
        let err = KeyAction::from_string("hyper+a").unwrap_err();
        assert!(err.to_string().to_lowercase().contains("unknown modifier"));
    }

    #[test]
    fn parse_key_action_rejects_unknown_key() {
        let err = KeyAction::from_string("cmd+notakey").unwrap_err();
        assert!(err.to_string().to_lowercase().contains("unknown key"));
    }

    #[test]
    fn parse_key_name_punctuation() {
        assert_eq!(usb_hid::parse_key_name("-"), Some(0x2D));
        assert_eq!(usb_hid::parse_key_name("="), Some(0x2E));
        assert_eq!(usb_hid::parse_key_name("escape"), Some(usb_hid::KEY_ESC));
    }

    #[test]
    fn normalize_boot_keyboard_report_unprefixed() {
        let data = [usb_hid::MOD_LEFT_GUI, 0, usb_hid::KEY_C, 0, 0, 0, 0, 0];
        let report = usb_hid::normalize_boot_keyboard_report(&data).unwrap();
        assert_eq!(report, data);
    }

    #[test]
    fn normalize_boot_keyboard_report_prefixed() {
        let data = [0, usb_hid::MOD_LEFT_GUI, 0, usb_hid::KEY_C, 0, 0, 0, 0, 0];
        let report = usb_hid::normalize_boot_keyboard_report(&data).unwrap();
        assert_eq!(
            report,
            [usb_hid::MOD_LEFT_GUI, 0, usb_hid::KEY_C, 0, 0, 0, 0, 0]
        );
    }

    #[test]
    fn normalize_boot_keyboard_report_padded() {
        let mut data = [0u8; 64];
        data[..8].copy_from_slice(&[0, 0, usb_hid::KEY_A, 0, 0, 0, 0, 0]);
        let report = usb_hid::normalize_boot_keyboard_report(&data).unwrap();
        assert_eq!(report, [0, 0, usb_hid::KEY_A, 0, 0, 0, 0, 0]);

        let mut prefixed = [0u8; 64];
        prefixed[..9].copy_from_slice(&[0, 0, 0, usb_hid::KEY_A, 0, 0, 0, 0, 0]);
        let report = usb_hid::normalize_boot_keyboard_report(&prefixed).unwrap();
        assert_eq!(report, [0, 0, usb_hid::KEY_A, 0, 0, 0, 0, 0]);
    }
}
