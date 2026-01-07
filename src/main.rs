//! way-thumbsense - ThumbSense for Linux/Wayland
//!
//! タッチパッドに触れている間、仮想キー(F24)を押し続ける
//! keydでF24をmousenavレイヤーのトリガーにすることでThumbSenseを実現

use evdev::{InputEventKind, Key};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use way_thumbsense::input::find_touchpad;
use way_thumbsense::output::VirtualDevice;

fn main() -> anyhow::Result<()> {
    println!("way-thumbsense starting...");

    // タッチパッドを検出
    let mut touchpad = find_touchpad()?;
    println!("Touchpad: {}", touchpad.name().unwrap_or("unknown"));

    // 仮想キーボードを作成
    let mut vdev = VirtualDevice::new()?;
    println!("Virtual keyboard created");

    println!("\nRunning... (Ctrl+C to exit)");
    println!("Touch trackpad to activate F24 (mousenav layer)\n");

    let mut is_touching = false;

    loop {
        match touchpad.fetch_events() {
            Ok(events) => {
                for ev in events {
                    if let InputEventKind::Key(key) = ev.kind() {
                        if key == Key::BTN_TOUCH {
                            let now_touching = ev.value() == 1;

                            if now_touching != is_touching {
                                is_touching = now_touching;

                                if is_touching {
                                    // タッチ開始 → F24を押す
                                    println!("[Touch] -> F24 press");
                                    vdev.forward_key(Key::KEY_F24, 1)?;
                                } else {
                                    // タッチ終了 → F24を離す
                                    println!("[Touch] -> F24 release");
                                    vdev.forward_key(Key::KEY_F24, 0)?;
                                }
                            }
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

    Ok(())
}
