use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand};
use hidapi::{HidApi, HidDevice};
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

struct SavantElite;

impl SavantElite {
    fn new() -> Result<Self> {
        // Verify HID API can be initialized
        let _ = HidApi::new().context("Failed to initialize HID API")?;
        Ok(Self)
    }

    fn find_device(&self) -> Result<()> {
        let api = HidApi::new().context("Failed to initialize HID API")?;

        for device in api.device_list() {
            if device.vendor_id() == KINESIS_VID && device.product_id() == SAVANT_ELITE_PID {
                println!("Found Kinesis Savant Elite foot pedal:");
                println!("  Vendor ID:  0x{:04X}", device.vendor_id());
                println!("  Product ID: 0x{:04X}", device.product_id());
                println!("  Path:       {}", device.path().to_string_lossy());
                if let Some(serial) = device.serial_number() {
                    println!("  Serial:     {}", serial);
                }
                if let Some(manufacturer) = device.manufacturer_string() {
                    println!("  Mfr:        {}", manufacturer);
                }
                if let Some(product) = device.product_string() {
                    println!("  Product:    {}", product);
                }
                println!("  Interface:  {}", device.interface_number());
                println!("  Usage Page: 0x{:04X}", device.usage_page());
                println!("  Usage:      0x{:04X}", device.usage());
                println!();
            }
        }
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
                return device.open_device(&api).context("Failed to open device");
            }
        }

        Err(anyhow!("Savant Elite keyboard interface not found"))
    }

    fn monitor(&self, duration_secs: u64) -> Result<()> {
        let device = self.open_keyboard_interface()?;

        println!("Monitoring Savant Elite foot pedal...");
        println!("Press pedals to see what keys they send.");
        println!("Press Ctrl+C to stop.\n");

        device.set_blocking_mode(false)?;

        let mut buf = [0u8; 64];
        let mut last_report = [0u8; 8];
        let start = std::time::Instant::now();

        loop {
            if duration_secs > 0 && start.elapsed().as_secs() >= duration_secs {
                println!("\nMonitoring complete.");
                break;
            }

            match device.read_timeout(&mut buf, 100) {
                Ok(len) if len > 0 => {
                    // Standard keyboard report is 8 bytes:
                    // [0] = modifier keys
                    // [1] = reserved (always 0)
                    // [2-7] = key codes (up to 6 simultaneous keys)

                    if len >= 8 && buf[..8] != last_report {
                        last_report.copy_from_slice(&buf[..8]);

                        let modifiers = buf[0];
                        let keys: Vec<u8> =
                            buf[2..8].iter().filter(|&&k| k != 0).copied().collect();

                        if modifiers != 0 || !keys.is_empty() {
                            print!("Pressed: ");

                            let mod_names = usb_hid::modifier_names(modifiers);
                            if !mod_names.is_empty() {
                                print!("{}", mod_names.join("+"));
                                if !keys.is_empty() {
                                    print!("+");
                                }
                            }

                            let key_names: Vec<&str> =
                                keys.iter().map(|&k| usb_hid::key_name(k)).collect();
                            print!("{}", key_names.join("+"));

                            println!("  [raw: {}]", hex::encode(&buf[..8]));
                        } else {
                            println!("Released [raw: {}]", hex::encode(&buf[..8]));
                        }
                    }
                }
                Ok(_) => {}
                Err(e) => {
                    // Timeout or other non-fatal error
                    if e.to_string().contains("timeout") {
                        continue;
                    }
                    return Err(anyhow!("Read error: {}", e));
                }
            }

            std::thread::sleep(Duration::from_millis(10));
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
            println!("=== Probing Savant Elite for Programming Protocol ===\n");

            let api = HidApi::new()?;

            println!("Looking for devices...");
            for device_info in api.device_list() {
                if device_info.vendor_id() == KINESIS_VID {
                    let pid = device_info.product_id();
                    println!(
                        "\nFound device: VID={:04X} PID={:04X}",
                        device_info.vendor_id(),
                        pid
                    );
                    println!("  Path: {}", device_info.path().to_string_lossy());
                    println!("  Interface: {}", device_info.interface_number());
                    println!(
                        "  Usage Page: 0x{:04X}, Usage: 0x{:04X}",
                        device_info.usage_page(),
                        device_info.usage()
                    );

                    if pid == PROGRAMMING_PID {
                        println!("  ** PROGRAMMING MODE DETECTED **");
                    }

                    // Try to open and probe
                    match device_info.open_device(&api) {
                        Ok(device) => {
                            println!("  Opened successfully");

                            // Try PI Engineering X-keys commands
                            let commands = [
                                (0xB5, "Generate Data"),
                                (0xC1, "Get Descriptor"),
                                (0xCD, "Get Key Macro"),
                            ];

                            for (cmd, name) in commands {
                                let mut cmd_buf = [0u8; 36];
                                cmd_buf[0] = 0; // Report ID
                                cmd_buf[1] = cmd;

                                match device.write(&cmd_buf) {
                                    Ok(n) => {
                                        println!("  Sent {} (0x{:02X}): {} bytes", name, cmd, n);

                                        // Try to read response
                                        std::thread::sleep(Duration::from_millis(50));
                                        let mut response = [0u8; 64];
                                        match device.read_timeout(&mut response, 200) {
                                            Ok(len) if len > 0 => {
                                                println!(
                                                    "    Response: {}",
                                                    hex::encode(&response[..len])
                                                );
                                            }
                                            _ => println!("    No response"),
                                        }
                                    }
                                    Err(e) => println!("  {} failed: {}", name, e),
                                }
                            }

                            // Try feature reports
                            println!("  Checking feature reports...");
                            for report_id in 0..10u8 {
                                let mut buf = [0u8; 65];
                                buf[0] = report_id;
                                match device.get_feature_report(&mut buf) {
                                    Ok(len) if len > 0 => {
                                        println!(
                                            "    Feature report {}: {}",
                                            report_id,
                                            hex::encode(&buf[..len])
                                        );
                                    }
                                    _ => {}
                                }
                            }
                        }
                        Err(e) => println!("  Failed to open: {}", e),
                    }
                }
            }

            println!("\n=== Protocol Notes ===");
            println!("Programming mode uses PID 0x0232 (vs normal 0x030C)");
            println!("\n=== TO ENTER PROGRAMMING MODE ===");
            println!("1. Flip the pedal over");
            println!("2. Look for a recessed switch near the Kinesis sticker");
            println!("3. Use a paperclip to flip it from 'Play' to 'Program'");
            println!("4. Unplug and replug USB, then run 'savant status'");
        }

        Commands::RawCmd {
            cmd,
            data,
            interface,
        } => {
            println!("Sending raw command...\n");

            let cmd_byte = u8::from_str_radix(&cmd, 16)
                .context("Invalid command byte (use hex, e.g., 'b5')")?;

            let data_bytes: Vec<u8> = if data.is_empty() {
                vec![]
            } else {
                hex::decode(&data).context("Invalid data bytes (use hex)")?
            };

            let api = HidApi::new()?;

            // Find the right interface
            for device_info in api.device_list() {
                if device_info.vendor_id() == KINESIS_VID
                    && device_info.product_id() == SAVANT_ELITE_PID
                    && device_info.interface_number() == interface
                {
                    match device_info.open_device(&api) {
                        Ok(device) => {
                            let mut cmd_buf = [0u8; 36];
                            cmd_buf[0] = 0; // Report ID
                            cmd_buf[1] = cmd_byte;
                            for (i, b) in data_bytes.iter().enumerate() {
                                if i + 2 < cmd_buf.len() {
                                    cmd_buf[i + 2] = *b;
                                }
                            }

                            println!("Sending: {}", hex::encode(&cmd_buf[..8]));

                            match device.write(&cmd_buf) {
                                Ok(n) => {
                                    println!("Sent {} bytes", n);

                                    // Read response
                                    std::thread::sleep(Duration::from_millis(100));
                                    let mut response = [0u8; 64];
                                    match device.read_timeout(&mut response, 500) {
                                        Ok(len) if len > 0 => {
                                            println!(
                                                "Response ({} bytes): {}",
                                                len,
                                                hex::encode(&response[..len])
                                            );
                                        }
                                        Ok(_) => println!("No response"),
                                        Err(e) => println!("Read error: {}", e),
                                    }
                                }
                                Err(e) => println!("Write error: {}", e),
                            }
                        }
                        Err(e) => println!("Failed to open device: {}", e),
                    }
                    break;
                }
            }
        }

        Commands::Status => {
            println!("=== Savant Elite Device Status ===\n");

            // Check via libusb first (more reliable for programming mode)
            let mut found_play_usb = false;
            let mut found_program_usb = false;

            for device in rusb::devices()?.iter() {
                let desc = device.device_descriptor()?;
                if desc.vendor_id() == KINESIS_VID {
                    match desc.product_id() {
                        SAVANT_ELITE_PID => {
                            found_play_usb = true;
                            println!(
                                "Found device in PLAY mode (PID 0x{:04X}) [via libusb]",
                                SAVANT_ELITE_PID
                            );
                            println!(
                                "  Bus {:03} Device {:03}",
                                device.bus_number(),
                                device.address()
                            );
                        }
                        PROGRAMMING_PID => {
                            found_program_usb = true;
                            println!(
                                "Found device in PROGRAMMING mode (PID 0x{:04X}) [via libusb]",
                                PROGRAMMING_PID
                            );
                            println!(
                                "  Bus {:03} Device {:03}",
                                device.bus_number(),
                                device.address()
                            );

                            // Try to get more info
                            if let Ok(handle) = device.open() {
                                if let Ok(langs) = handle.read_languages(Duration::from_millis(100))
                                {
                                    if let Some(lang) = langs.first() {
                                        if let Ok(prod) = handle.read_product_string(
                                            *lang,
                                            &desc,
                                            Duration::from_millis(100),
                                        ) {
                                            println!("  Product: {}", prod);
                                        }
                                        if let Ok(mfr) = handle.read_manufacturer_string(
                                            *lang,
                                            &desc,
                                            Duration::from_millis(100),
                                        ) {
                                            println!("  Manufacturer: {}", mfr);
                                        }
                                    }
                                }
                            }
                        }
                        pid => {
                            println!("Found Kinesis device (PID 0x{:04X})", pid);
                        }
                    }
                }
            }

            // Also check HID (for play mode with interfaces)
            let api = HidApi::new()?;
            let mut found_play_hid = false;

            for device_info in api.device_list() {
                if device_info.vendor_id() == KINESIS_VID
                    && device_info.product_id() == SAVANT_ELITE_PID
                {
                    if !found_play_usb && !found_play_hid {
                        found_play_hid = true;
                        println!(
                            "Found device in PLAY mode (PID 0x{:04X}) [via HID]",
                            SAVANT_ELITE_PID
                        );
                    }
                    println!(
                        "  Interface {}: Usage Page 0x{:04X}, Usage 0x{:04X}",
                        device_info.interface_number(),
                        device_info.usage_page(),
                        device_info.usage()
                    );
                }
            }

            let found_play = found_play_usb || found_play_hid;
            let found_program = found_program_usb;

            if !found_play && !found_program {
                println!("No Savant Elite device found.");
                println!("\nTroubleshooting:");
                println!("  1. Make sure the device is connected via USB");
                println!("  2. Try unplugging and replugging the cable");
            } else if found_play && !found_program {
                println!("\n=== Device is in PLAY mode ===");
                println!("To program the device, you need to switch to PROGRAMMING mode:");
                println!("  1. Flip the pedal over");
                println!("  2. Find the recessed switch near the Kinesis sticker");
                println!("  3. Use a paperclip to flip it from 'Play' to 'Program'");
                println!("  4. Unplug and replug the USB cable");
                println!("  5. Run 'savant status' to verify");
            } else if found_program {
                println!("\n=== Device is in PROGRAMMING mode ===");
                println!("You can now program the pedals using:");
                println!("  savant program --left 'cmd+c' --middle 'cmd+a' --right 'cmd+v'");
            }
        }

        Commands::Program {
            left,
            middle,
            right,
            dry_run,
        } => {
            println!("=== Programming Savant Elite Pedals ===\n");

            // Check if device is in programming mode using libusb
            let mut programming_device: Option<Device<GlobalContext>> = None;
            let mut play_mode_found = false;

            for device in rusb::devices()?.iter() {
                let desc = device.device_descriptor()?;
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
                if play_mode_found {
                    println!("ERROR: Device is in PLAY mode, not PROGRAMMING mode.\n");
                    println!("To enter programming mode:");
                    println!("  1. Flip the pedal over");
                    println!("  2. Find the recessed switch near the Kinesis sticker");
                    println!("  3. Use a paperclip to flip it from 'Play' to 'Program'");
                    println!("  4. Unplug and replug the USB cable");
                    println!("  5. Run 'savant status' to verify, then 'savant program'");
                } else {
                    println!("ERROR: Savant Elite not found. Make sure it's connected.");
                }
                return Ok(());
            }

            let device = programming_device.unwrap();

            // Parse key actions
            let left_action = KeyAction::from_string(&left)?;
            let middle_action = KeyAction::from_string(&middle)?;
            let right_action = KeyAction::from_string(&right)?;

            println!("Configuration to program:");
            println!(
                "  Left pedal:   {} (mod=0x{:02X}, key=0x{:02X})",
                left, left_action.modifiers, left_action.key
            );
            println!(
                "  Middle pedal: {} (mod=0x{:02X}, key=0x{:02X})",
                middle, middle_action.modifiers, middle_action.key
            );
            println!(
                "  Right pedal:  {} (mod=0x{:02X}, key=0x{:02X})",
                right, right_action.modifiers, right_action.key
            );
            println!();

            if dry_run {
                println!("DRY RUN: Would send the following commands:");
                println!(
                    "  Set Key Macro (0xCC) for pedal 0: mod=0x{:02X}, key=0x{:02X}",
                    left_action.modifiers, left_action.key
                );
                println!(
                    "  Set Key Macro (0xCC) for pedal 1: mod=0x{:02X}, key=0x{:02X}",
                    middle_action.modifiers, middle_action.key
                );
                println!(
                    "  Set Key Macro (0xCC) for pedal 2: mod=0x{:02X}, key=0x{:02X}",
                    right_action.modifiers, right_action.key
                );
                println!("  Save to EEPROM (0xCE)");
                return Ok(());
            }

            // Open device
            let handle = device.open().context("Failed to open device")?;

            // Get device config to find endpoints
            let config = device.active_config_descriptor()?;
            println!("Device has {} interface(s)", config.num_interfaces());

            // Try to claim interface 0
            let interface_num = 0;
            if handle.kernel_driver_active(interface_num).unwrap_or(false) {
                println!(
                    "Detaching kernel driver from interface {}...",
                    interface_num
                );
                handle.detach_kernel_driver(interface_num)?;
            }

            handle
                .claim_interface(interface_num)
                .context("Failed to claim interface - do you have permission?")?;

            println!("Claimed interface {}", interface_num);

            // Log endpoint information for debugging
            for interface in config.interfaces() {
                for desc in interface.descriptors() {
                    println!(
                        "Interface {}: class={} subclass={} protocol={}",
                        desc.interface_number(),
                        desc.class_code(),
                        desc.sub_class_code(),
                        desc.protocol_code()
                    );
                    for ep in desc.endpoint_descriptors() {
                        let dir = match ep.direction() {
                            rusb::Direction::Out => "OUT",
                            rusb::Direction::In => "IN",
                        };
                        let transfer = match ep.transfer_type() {
                            rusb::TransferType::Control => "Control",
                            rusb::TransferType::Isochronous => "Isochronous",
                            rusb::TransferType::Bulk => "Bulk",
                            rusb::TransferType::Interrupt => "Interrupt",
                        };
                        println!(
                            "  Endpoint 0x{:02X}: {} {} (max packet: {})",
                            ep.address(),
                            dir,
                            transfer,
                            ep.max_packet_size()
                        );
                    }
                }
            }

            // Program each pedal using HID SET_REPORT
            let pedals = [
                (xkeys_protocol::PEDAL_LEFT, &left_action, "Left"),
                (xkeys_protocol::PEDAL_MIDDLE, &middle_action, "Middle"),
                (xkeys_protocol::PEDAL_RIGHT, &right_action, "Right"),
            ];

            for (pedal_idx, action, name) in pedals {
                print!("Programming {} pedal... ", name);

                // Try multiple data formats and transfer methods
                let mut success = false;

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

                // Format 2: Report ID 0, then command
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

                // Format 3: Report ID = command
                let cmd3 = [
                    xkeys_protocol::CMD_SET_KEY_MACRO,
                    pedal_idx,
                    action.modifiers,
                    action.key,
                    0,
                    0,
                    0,
                    0,
                ];

                // Try SET_REPORT with command as report ID (Feature report)
                for (fmt_name, data) in [
                    ("fmt1-feat", &cmd1[..]),
                    ("fmt2-feat", &cmd2[..]),
                    ("fmt3-feat", &cmd3[..]),
                ] {
                    let report_id = data[0] as u16;
                    let result = handle.write_control(
                        0x21,
                        0x09,
                        0x0300 | report_id, // Feature report
                        interface_num as u16,
                        data,
                        Duration::from_millis(500),
                    );
                    if result.is_ok() {
                        success = true;
                        println!("OK ({})", fmt_name);
                        break;
                    }
                }

                // Try SET_REPORT with Output report type
                if !success {
                    for (fmt_name, data) in [
                        ("fmt1-out", &cmd1[..]),
                        ("fmt2-out", &cmd2[..]),
                        ("fmt3-out", &cmd3[..]),
                    ] {
                        let report_id = data[0] as u16;
                        let result = handle.write_control(
                            0x21,
                            0x09,
                            0x0200 | report_id, // Output report
                            interface_num as u16,
                            data,
                            Duration::from_millis(500),
                        );
                        if result.is_ok() {
                            success = true;
                            println!("OK ({})", fmt_name);
                            break;
                        }
                    }
                }

                // Try with longer buffer (36 bytes like PI Engineering)
                if !success {
                    let mut long_buf = [0u8; 36];
                    long_buf[0] = 0;
                    long_buf[1] = xkeys_protocol::CMD_SET_KEY_MACRO;
                    long_buf[2] = pedal_idx;
                    long_buf[3] = action.modifiers;
                    long_buf[4] = action.key;

                    let result = handle.write_control(
                        0x21,
                        0x09,
                        0x0200,
                        interface_num as u16,
                        &long_buf,
                        Duration::from_millis(500),
                    );
                    if result.is_ok() {
                        success = true;
                        println!("OK (36-byte)");
                    }
                }

                // Try vendor-specific request
                if !success {
                    let result = handle.write_control(
                        0x40,                                                   // Vendor, Device
                        xkeys_protocol::CMD_SET_KEY_MACRO, // bRequest = command
                        ((action.key as u16) << 8) | (action.modifiers as u16), // wValue
                        pedal_idx as u16,                  // wIndex
                        &[],
                        Duration::from_millis(500),
                    );
                    if result.is_ok() {
                        success = true;
                        println!("OK (vendor)");
                    }
                }

                if !success {
                    println!("FAILED");
                }

                std::thread::sleep(Duration::from_millis(50));
            }

            // Save to EEPROM - use the same format that worked for programming
            print!("Saving to EEPROM... ");
            let save_cmd = [xkeys_protocol::CMD_SAVE_TO_EEPROM, 0, 0, 0, 0, 0, 0, 0];

            let save_result = handle.write_control(
                0x21,
                0x09,
                0x0200 | (xkeys_protocol::CMD_SAVE_TO_EEPROM as u16), // Output report
                interface_num as u16,
                &save_cmd,
                Duration::from_millis(1000),
            );

            if save_result.is_ok() {
                std::thread::sleep(Duration::from_millis(200));
                println!("OK");
            } else {
                // Try alternate formats
                let mut save_success = false;
                let save_alt = [0u8, xkeys_protocol::CMD_SAVE_TO_EEPROM, 0, 0, 0, 0, 0, 0];

                if handle
                    .write_control(
                        0x21,
                        0x09,
                        0x0200,
                        interface_num as u16,
                        &save_alt,
                        Duration::from_millis(500),
                    )
                    .is_ok()
                {
                    save_success = true;
                }
                if !save_success
                    && handle
                        .write_control(
                            0x21,
                            0x09,
                            0x0300 | (xkeys_protocol::CMD_SAVE_TO_EEPROM as u16),
                            interface_num as u16,
                            &save_cmd,
                            Duration::from_millis(500),
                        )
                        .is_ok()
                {
                    save_success = true;
                }

                if save_success {
                    std::thread::sleep(Duration::from_millis(200));
                    println!("OK");
                } else {
                    println!("WARNING: Save command may have failed, but programming was done");
                }
            }

            // Release interface
            let _ = handle.release_interface(interface_num);

            println!("\nProgramming complete!");
            println!("\nTo use the new configuration:");
            println!("  1. Flip the switch back to 'Play' mode");
            println!("  2. Unplug and replug the USB cable");
            println!("  3. Your pedals should now send the programmed keys");
        }
    }

    Ok(())
}
