//! デバイス確認ツール
//!
//! 使い方:
//!   sudo cargo run --example check_devices
//!
//! タッチパッドに触れる/離す、キーを押す/離すしてイベントを確認

use evdev::{Device, EventType, InputEventKind, Key};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== 入力デバイス一覧 ===\n");

    // /dev/input/event* を列挙
    let mut devices: Vec<_> = fs::read_dir("/dev/input")?
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_name()
                .to_string_lossy()
                .starts_with("event")
        })
        .collect();

    devices.sort_by_key(|e| e.path());

    for entry in devices {
        let path = entry.path();
        match Device::open(&path) {
            Ok(device) => {
                let name = device.name().unwrap_or("(unknown)");

                // タッチパッドかキーボードを判定
                let supported = device.supported_events();
                let has_abs = supported.contains(EventType::ABSOLUTE);
                let has_key = supported.contains(EventType::KEY);

                let device_type = if has_abs && has_key {
                    // ABSとKEY両方あればタッチパッド候補
                    if let Some(keys) = device.supported_keys() {
                        if keys.contains(Key::BTN_TOUCH) {
                            "TOUCHPAD"
                        } else if keys.contains(Key::KEY_A) {
                            "KEYBOARD"
                        } else {
                            ""
                        }
                    } else {
                        ""
                    }
                } else if has_key {
                    if let Some(keys) = device.supported_keys() {
                        if keys.contains(Key::KEY_A) {
                            "KEYBOARD"
                        } else {
                            ""
                        }
                    } else {
                        ""
                    }
                } else {
                    ""
                };

                let marker = if !device_type.is_empty() {
                    format!(" <-- {}", device_type)
                } else {
                    String::new()
                };

                println!("{}: {}{}", path.display(), name, marker);
            }
            Err(_) => {
                println!("{}: (permission denied)", path.display());
            }
        }
    }

    println!("\n=== BTN_TOUCH対応デバイス ===\n");

    for entry in fs::read_dir("/dev/input")? {
        let entry = entry?;
        let path = entry.path();
        if !path.to_string_lossy().contains("event") {
            continue;
        }

        if let Ok(device) = Device::open(&path) {
            if let Some(keys) = device.supported_keys() {
                if keys.contains(Key::BTN_TOUCH) {
                    println!(
                        "{}: {} (BTN_TOUCH対応)",
                        path.display(),
                        device.name().unwrap_or("unknown")
                    );
                }
            }
        }
    }

    Ok(())
}
