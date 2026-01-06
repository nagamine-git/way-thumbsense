//! イベント監視ツール
//!
//! 使い方:
//!   cargo run --example watch_events
//!
//! タッチパッドのBTN_TOUCHとキーボードのJ/Kイベントを監視

use evdev::{Device, EventType, InputEventKind, Key};
use std::os::unix::io::AsRawFd;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // デバイスを開く
    let mut touchpad = find_device_by_name("Magic Trackpad")?;
    let mut keyboard = find_device_by_name("keyd virtual keyboard")?;

    println!("=== イベント監視開始 ===");
    println!("タッチパッド: {}", touchpad.name().unwrap_or("unknown"));
    println!("キーボード: {}", keyboard.name().unwrap_or("unknown"));
    println!("\n[タッチパッドに触れる/離す、J/Kキーを押す/離すしてください]");
    println!("[Ctrl+C で終了]\n");

    // 共有のタッチ状態
    let touch_state = Arc::new(AtomicBool::new(false));
    let touch_state_clone = Arc::clone(&touch_state);

    // タッチパッド監視スレッド
    let touchpad_handle = thread::spawn(move || {
        loop {
            match touchpad.fetch_events() {
                Ok(events) => {
                    for ev in events {
                        if let InputEventKind::Key(key) = ev.kind() {
                            if key == Key::BTN_TOUCH {
                                let touching = ev.value() == 1;
                                touch_state_clone.store(touching, Ordering::SeqCst);
                                println!(
                                    "[TOUCH] {}",
                                    if touching { "TOUCHING" } else { "RELEASED" }
                                );
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Touchpad error: {}", e);
                    break;
                }
            }
        }
    });

    // キーボード監視（メインスレッド）
    loop {
        match keyboard.fetch_events() {
            Ok(events) => {
                for ev in events {
                    if let InputEventKind::Key(key) = ev.kind() {
                        let key_name = match key {
                            Key::KEY_J => Some("J"),
                            Key::KEY_K => Some("K"),
                            _ => None,
                        };

                        if let Some(name) = key_name {
                            let action = match ev.value() {
                                0 => "RELEASE",
                                1 => "PRESS",
                                2 => "REPEAT",
                                _ => "UNKNOWN",
                            };
                            let touching = touch_state.load(Ordering::SeqCst);
                            println!(
                                "[KEY] {} {} (touch: {})",
                                name,
                                action,
                                if touching { "YES" } else { "NO" }
                            );
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Keyboard error: {}", e);
                break;
            }
        }
    }

    touchpad_handle.join().ok();
    Ok(())
}

fn find_device_by_name(name_contains: &str) -> Result<Device, Box<dyn std::error::Error>> {
    use std::fs;

    for entry in fs::read_dir("/dev/input")? {
        let entry = entry?;
        let path = entry.path();

        if !path.to_string_lossy().contains("event") {
            continue;
        }

        if let Ok(device) = Device::open(&path) {
            if let Some(name) = device.name() {
                if name.contains(name_contains) {
                    // KEYイベント対応のものを選ぶ
                    if device.supported_events().contains(EventType::KEY) {
                        return Ok(device);
                    }
                }
            }
        }
    }

    Err(format!("Device containing '{}' not found", name_contains).into())
}
