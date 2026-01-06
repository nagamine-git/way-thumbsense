//! evdevデバイスの検出と読み取り

use evdev::{Device, EventType, Key};
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceType {
    Touchpad,
    Keyboard,
}

/// デバイス名の一部を指定してデバイスを検索
pub fn find_device(name_contains: &str) -> Result<Device, FindDeviceError> {
    for entry in fs::read_dir("/dev/input").map_err(|_| FindDeviceError::CannotReadInputDir)? {
        let entry = entry.map_err(|_| FindDeviceError::CannotReadInputDir)?;
        let path = entry.path();

        if !path.to_string_lossy().contains("event") {
            continue;
        }

        if let Ok(device) = Device::open(&path) {
            if let Some(name) = device.name() {
                if name.contains(name_contains) {
                    if device.supported_events().contains(EventType::KEY) {
                        return Ok(device);
                    }
                }
            }
        }
    }

    Err(FindDeviceError::NotFound(name_contains.to_string()))
}

/// BTN_TOUCH対応のタッチパッドを自動検出
pub fn find_touchpad() -> Result<Device, FindDeviceError> {
    for entry in fs::read_dir("/dev/input").map_err(|_| FindDeviceError::CannotReadInputDir)? {
        let entry = entry.map_err(|_| FindDeviceError::CannotReadInputDir)?;
        let path = entry.path();

        if !path.to_string_lossy().contains("event") {
            continue;
        }

        if let Ok(device) = Device::open(&path) {
            if let Some(keys) = device.supported_keys() {
                if keys.contains(Key::BTN_TOUCH) {
                    return Ok(device);
                }
            }
        }
    }

    Err(FindDeviceError::NotFound("touchpad with BTN_TOUCH".to_string()))
}

/// キーボードを自動検出（KEY_J対応デバイス）
pub fn find_keyboard() -> Result<Device, FindDeviceError> {
    // 優先順位: keyd virtual keyboard > 物理キーボード
    if let Ok(device) = find_device("keyd virtual keyboard") {
        return Ok(device);
    }

    // keydがない場合はKEY_J対応のキーボードを探す
    for entry in fs::read_dir("/dev/input").map_err(|_| FindDeviceError::CannotReadInputDir)? {
        let entry = entry.map_err(|_| FindDeviceError::CannotReadInputDir)?;
        let path = entry.path();

        if !path.to_string_lossy().contains("event") {
            continue;
        }

        if let Ok(device) = Device::open(&path) {
            if let Some(keys) = device.supported_keys() {
                if keys.contains(Key::KEY_J) && keys.contains(Key::KEY_A) {
                    return Ok(device);
                }
            }
        }
    }

    Err(FindDeviceError::NotFound("keyboard with KEY_J".to_string()))
}

#[derive(Debug)]
pub enum FindDeviceError {
    CannotReadInputDir,
    NotFound(String),
}

impl std::fmt::Display for FindDeviceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FindDeviceError::CannotReadInputDir => write!(f, "Cannot read /dev/input directory"),
            FindDeviceError::NotFound(name) => write!(f, "Device '{}' not found", name),
        }
    }
}

impl std::error::Error for FindDeviceError {}

#[cfg(test)]
mod tests {
    use super::*;

    // 注: これらのテストは実際のデバイスが必要なので、
    // CI環境では #[ignore] を付けて実行をスキップ

    #[test]
    #[ignore]
    fn test_find_touchpad() {
        let result = find_touchpad();
        assert!(result.is_ok(), "Touchpad should be found");
    }

    #[test]
    #[ignore]
    fn test_find_keyboard() {
        let result = find_keyboard();
        assert!(result.is_ok(), "Keyboard should be found");
    }
}
