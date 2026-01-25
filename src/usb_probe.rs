//! USB protocol probing for Kinesis Savant Elite
//! This module attempts to discover the programming protocol

use anyhow::{anyhow, Result};
use hidapi::HidApi;

pub const KINESIS_VID: u16 = 0x05F3;
pub const SAVANT_ELITE_PID: u16 = 0x030C;
pub const PROGRAMMING_PID: u16 = 0x0232; // PID when in programming mode (from driver INF)

/// Known PI Engineering / X-keys command bytes
/// Based on analysis of PI Engineering SDK documentation
pub mod xkeys_commands {
    // Output report commands (sent to device)
    pub const CMD_GENERATE_DATA: u8 = 0xB5;      // Request device to send current state
    pub const CMD_SET_LED: u8 = 0xB6;            // Set LED state
    pub const CMD_SET_FLASH_FREQ: u8 = 0xB7;    // Set flash frequency
    pub const CMD_SET_TIMESTAMP: u8 = 0xB8;     // Enable/disable timestamp
    pub const CMD_GET_DESCRIPTOR: u8 = 0xC1;    // Request device descriptor
    pub const CMD_SET_UNIT_ID: u8 = 0xC9;       // Set unit ID
    pub const CMD_SET_PID: u8 = 0xCA;           // Set product ID (change mode)
    pub const CMD_REBOOT: u8 = 0xCB;            // Reboot device
    pub const CMD_SET_KEY_MACRO: u8 = 0xCC;     // Set key macro (program key)
    pub const CMD_GET_KEY_MACRO: u8 = 0xCD;     // Get key macro
    pub const CMD_SAVE_TO_EEPROM: u8 = 0xCE;    // Save settings to EEPROM
    
    // These are common across PI Engineering devices but may vary for Kinesis
}

/// Probe result structure
#[derive(Debug)]
pub struct ProbeResult {
    pub device_found: bool,
    pub interfaces: Vec<InterfaceInfo>,
    pub descriptor_response: Option<Vec<u8>>,
    pub feature_reports: Vec<(u8, Vec<u8>)>,
    pub output_responses: Vec<(u8, Vec<u8>)>,
}

#[derive(Debug)]
pub struct InterfaceInfo {
    pub interface_number: i32,
    pub usage_page: u16,
    pub usage: u16,
    pub path: String,
}

/// Probe the device for programming capabilities
pub fn probe_device() -> Result<ProbeResult> {
    let api = HidApi::new()?;
    
    let mut result = ProbeResult {
        device_found: false,
        interfaces: Vec::new(),
        descriptor_response: None,
        feature_reports: Vec::new(),
        output_responses: Vec::new(),
    };
    
    // Find all Savant Elite interfaces
    for device_info in api.device_list() {
        if device_info.vendor_id() == KINESIS_VID {
            let pid = device_info.product_id();
            
            // Check for both normal and programming mode PIDs
            if pid == SAVANT_ELITE_PID || pid == PROGRAMMING_PID {
                result.device_found = true;
                
                result.interfaces.push(InterfaceInfo {
                    interface_number: device_info.interface_number(),
                    usage_page: device_info.usage_page(),
                    usage: device_info.usage(),
                    path: device_info.path().to_string_lossy().to_string(),
                });
            }
        }
    }
    
    if !result.device_found {
        return Err(anyhow!("Savant Elite not found"));
    }
    
    // Try to open each interface and probe it
    for info in &result.interfaces {
        println!("Probing interface {} (usage page: 0x{:04X}, usage: 0x{:04X})", 
                 info.interface_number, info.usage_page, info.usage);
        
        // Try to open the device
        match api.open_path(std::ffi::CString::new(info.path.as_str())?.as_ref()) {
            Ok(device) => {
                println!("  Opened successfully");
                
                // Try to read feature reports
                for report_id in 0..=255u8 {
                    let mut buf = [0u8; 65];
                    buf[0] = report_id;
                    
                    match device.get_feature_report(&mut buf) {
                        Ok(len) if len > 0 => {
                            println!("  Feature report {}: {} bytes", report_id, len);
                            result.feature_reports.push((report_id, buf[..len].to_vec()));
                        }
                        _ => {}
                    }
                }
                
                // Try sending PI Engineering commands
                for cmd in [
                    xkeys_commands::CMD_GENERATE_DATA,
                    xkeys_commands::CMD_GET_DESCRIPTOR,
                    xkeys_commands::CMD_GET_KEY_MACRO,
                ] {
                    let mut cmd_buf = [0u8; 36];
                    cmd_buf[0] = 0; // Report ID
                    cmd_buf[1] = cmd;
                    
                    match device.write(&cmd_buf) {
                        Ok(_) => {
                            println!("  Sent command 0x{:02X}", cmd);
                            
                            // Try to read response
                            let mut response = [0u8; 64];
                            match device.read_timeout(&mut response, 500) {
                                Ok(len) if len > 0 => {
                                    println!("  Response: {} bytes", len);
                                    result.output_responses.push((cmd, response[..len].to_vec()));
                                }
                                _ => {}
                            }
                        }
                        Err(e) => {
                            println!("  Command 0x{:02X} failed: {}", cmd, e);
                        }
                    }
                }
            }
            Err(e) => {
                println!("  Failed to open: {}", e);
            }
        }
    }
    
    Ok(result)
}

/// Check if device is in programming mode (different PID)
pub fn check_programming_mode() -> Result<bool> {
    let api = HidApi::new()?;
    
    for device_info in api.device_list() {
        if device_info.vendor_id() == KINESIS_VID 
           && device_info.product_id() == PROGRAMMING_PID {
            return Ok(true);
        }
    }
    
    Ok(false)
}
