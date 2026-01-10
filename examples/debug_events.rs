//! すべてのタッチパッドイベントをデバッグ出力するツール
//!
//! 使い方:
//!   cargo run --example debug_events

use evdev::{Device, InputEventKind};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // BTN_TOUCH対応のタッチパッドを検出
    let mut touchpad = find_touchpad()?;
    println!("Touchpad: {}", touchpad.name().unwrap_or("unknown"));
    println!("\nTouch the trackpad to see events...\n");

    loop {
        match touchpad.fetch_events() {
            Ok(events) => {
                for ev in events {
                    println!("{:?}", ev);
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        }
    }

    Ok(())
}

fn find_touchpad() -> Result<Device, Box<dyn std::error::Error>> {
    use evdev::{EventType, Key};

    for entry in fs::read_dir("/dev/input")? {
        let entry = entry?;
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

    Err("Touchpad not found".into())
}
