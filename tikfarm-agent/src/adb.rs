use serde::{Deserialize, Serialize};
use std::process::Command;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    pub real_serial: String,
    pub serial: String,
    pub status: String,
    pub transport_id: String,
    pub product: String,
    pub mode: String,
    pub name: String,
    pub forward_port: String,
    pub connect_type: u32,
    pub group_id: u32,
    pub group_ids: Vec<u32>,
}

pub fn find_adb() -> PathBuf {
    // Check common ADB locations
    let candidates = vec![
        dirs::data_dir().map(|d| d.join("com.tikfarm").join("platform-tools").join("adb.exe")),
        dirs::data_dir().map(|d| d.join("com.tikmatrix").join("platform-tools").join("adb.exe")),
        Some(PathBuf::from(r"C:\Users\Usuario\adb.exe")),
        Some(PathBuf::from(r"C:\Users\Usuario\AppData\Roaming\com.tikmatrix\platform-tools\adb.exe")),
    ];

    for candidate in candidates.into_iter().flatten() {
        if candidate.exists() {
            log::info!("Found ADB at: {:?}", candidate);
            return candidate;
        }
    }

    // Fallback to PATH
    PathBuf::from("adb")
}

pub fn list_devices(adb_path: &PathBuf) -> Vec<Device> {
    let output = match Command::new(adb_path).arg("devices").arg("-l").output() {
        Ok(o) => o,
        Err(e) => {
            log::error!("Failed to run adb devices: {}", e);
            return vec![];
        }
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut devices = Vec::new();

    for line in stdout.lines().skip(1) {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 {
            continue;
        }

        let serial = parts[0].to_string();
        let status = parts[1].to_string();

        if status != "device" {
            continue;
        }

        // Parse properties like product:xxx model:xxx transport_id:xxx
        let mut product = String::new();
        let mut model = String::new();
        let mut transport_id = String::new();
        let mut device_name = String::new();

        for part in &parts[2..] {
            if let Some(val) = part.strip_prefix("product:") {
                product = val.to_string();
            } else if let Some(val) = part.strip_prefix("model:") {
                model = val.to_string();
            } else if let Some(val) = part.strip_prefix("transport_id:") {
                transport_id = val.to_string();
            } else if let Some(val) = part.strip_prefix("device:") {
                device_name = val.to_string();
            }
        }

        devices.push(Device {
            real_serial: serial.clone(),
            serial: serial.clone(),
            status,
            transport_id,
            product: product.clone(),
            mode: model,
            name: if device_name.is_empty() { product } else { device_name },
            forward_port: String::new(),
            connect_type: 0,
            group_id: 0,
            group_ids: vec![],
        });
    }

    devices
}

pub fn run_adb_command(adb_path: &PathBuf, serial: &str, args: &[&str]) -> Result<String, String> {
    let mut cmd = Command::new(adb_path);
    cmd.arg("-s").arg(serial);
    for arg in args {
        cmd.arg(arg);
    }

    match cmd.output() {
        Ok(output) => {
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).to_string())
            } else {
                Err(String::from_utf8_lossy(&output.stderr).to_string())
            }
        }
        Err(e) => Err(format!("Failed to execute adb: {}", e)),
    }
}

/// Get device screen resolution
pub fn get_screen_size(adb_path: &PathBuf, serial: &str) -> (u32, u32) {
    match run_adb_command(adb_path, serial, &["shell", "wm", "size"]) {
        Ok(output) => {
            // Output: "Physical size: 1440x2960"
            for line in output.lines() {
                if let Some(size_str) = line.strip_prefix("Physical size: ") {
                    let parts: Vec<&str> = size_str.trim().split('x').collect();
                    if parts.len() == 2 {
                        let w = parts[0].parse().unwrap_or(1080);
                        let h = parts[1].parse().unwrap_or(1920);
                        return (w, h);
                    }
                }
            }
            (1080, 1920)
        }
        Err(_) => (1080, 1920),
    }
}

/// Get device model name
pub fn get_device_name(adb_path: &PathBuf, serial: &str) -> String {
    match run_adb_command(adb_path, serial, &["shell", "getprop", "ro.product.model"]) {
        Ok(name) => name.trim().to_string(),
        Err(_) => "Unknown".to_string(),
    }
}

/// Push scrcpy-server to device
pub fn push_scrcpy_server(adb_path: &PathBuf, serial: &str, server_path: &PathBuf) -> Result<(), String> {
    run_adb_command(
        adb_path,
        serial,
        &["push", &server_path.to_string_lossy(), "/data/local/tmp/scrcpy-server.jar"],
    )?;
    Ok(())
}

/// Forward a local TCP port to a device port
pub fn forward_port(adb_path: &PathBuf, serial: &str, local_port: u16, remote_port: u16) -> Result<(), String> {
    run_adb_command(
        adb_path,
        serial,
        &["forward", &format!("tcp:{}", local_port), &format!("tcp:{}", remote_port)],
    )?;
    Ok(())
}
