//! evdevデバイスの検出と読み取り

use evdev::{AbsoluteAxisType, Device, EventType, Key};
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceType {
    Touchpad,
    Keyboard,
}

/// タッチパッドの寸法情報
#[derive(Debug, Clone, Copy)]
pub struct TouchpadDimensions {
    pub min_x: i32,
    pub max_x: i32,
    pub min_y: i32,
    pub max_y: i32,
}

impl TouchpadDimensions {
    /// X座標の範囲幅
    pub fn width(&self) -> i32 {
        self.max_x - self.min_x
    }

    /// Y座標の範囲幅
    pub fn height(&self) -> i32 {
        self.max_y - self.min_y
    }
}

/// タッチパッドの座標範囲を取得
pub fn get_touchpad_dimensions(device: &Device) -> Option<TouchpadDimensions> {
    let abs_state = device.get_abs_state().ok()?;

    let x_info = abs_state.get(AbsoluteAxisType::ABS_X.0 as usize)?;
    let y_info = abs_state.get(AbsoluteAxisType::ABS_Y.0 as usize)?;

    Some(TouchpadDimensions {
        min_x: x_info.minimum,
        max_x: x_info.maximum,
        min_y: y_info.minimum,
        max_y: y_info.maximum,
    })
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
            // 自分自身の仮想デバイスを除外
            if let Some(name) = device.name() {
                if name.contains("way-thumbsense") {
                    continue;
                }
            }

            // BTN_TOUCHとABS_X/ABS_Yを持つデバイスのみ（実際のタッチパッド）
            let has_btn_touch = device
                .supported_keys()
                .map(|keys| keys.contains(Key::BTN_TOUCH))
                .unwrap_or(false);

            let has_abs_axes = device
                .supported_absolute_axes()
                .map(|axes| {
                    axes.contains(AbsoluteAxisType::ABS_X)
                        && axes.contains(AbsoluteAxisType::ABS_Y)
                })
                .unwrap_or(false);

            if has_btn_touch && has_abs_axes {
                return Ok(device);
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
