//! way-thumbsense - ThumbSense for Linux/Wayland
//!
//! タッチパッドに触れている間、J/Kキーをマウスクリックに変換

use evdev::{InputEventKind, Key};
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;
use std::thread;
use way_thumbsense::core::{map_key_event, KeyCode, KeyEvent, OutputAction, TouchState};
use way_thumbsense::input::{find_keyboard, find_touchpad};
use way_thumbsense::output::VirtualMouse;

fn main() -> anyhow::Result<()> {
    println!("way-thumbsense starting...");

    // デバイスを検出
    let mut touchpad = find_touchpad()?;
    let mut keyboard = find_keyboard()?;

    println!("Touchpad: {}", touchpad.name().unwrap_or("unknown"));
    println!("Keyboard: {}", keyboard.name().unwrap_or("unknown"));

    // 仮想マウスを作成
    let mut virtual_mouse = VirtualMouse::new()?;
    println!("Virtual mouse created");

    // 共有のタッチ状態 (finger_count)
    let finger_count = Arc::new(AtomicU8::new(0));
    let finger_count_clone = Arc::clone(&finger_count);

    // タッチパッド監視スレッド
    thread::spawn(move || {
        loop {
            match touchpad.fetch_events() {
                Ok(events) => {
                    for ev in events {
                        if let InputEventKind::Key(key) = ev.kind() {
                            if key == Key::BTN_TOUCH {
                                let count = if ev.value() == 1 { 1 } else { 0 };
                                finger_count_clone.store(count, Ordering::SeqCst);
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

    println!("\nRunning... (Ctrl+C to exit)");
    println!("Touch trackpad + press J (left click) or K (right click)\n");

    // キーボード監視（メインスレッド）
    loop {
        match keyboard.fetch_events() {
            Ok(events) => {
                for ev in events {
                    if let InputEventKind::Key(key) = ev.kind() {
                        // J/Kキーのみ処理
                        let key_code = match key {
                            Key::KEY_J => Some(KeyCode::J),
                            Key::KEY_K => Some(KeyCode::K),
                            _ => None,
                        };

                        if let Some(code) = key_code {
                            // Press (1) または Release (0) のみ処理（Repeat (2) は無視）
                            let key_event = match ev.value() {
                                1 => Some(KeyEvent::Press(code)),
                                0 => Some(KeyEvent::Release(code)),
                                _ => None,
                            };

                            if let Some(event) = key_event {
                                let touch = TouchState {
                                    finger_count: finger_count.load(Ordering::SeqCst),
                                };

                                let action = map_key_event(event, &touch);

                                // デバッグ出力
                                match &action {
                                    OutputAction::MouseClick(btn) => {
                                        println!("[ThumbSense] {:?} -> {:?} click", code, btn);
                                    }
                                    OutputAction::MouseRelease(btn) => {
                                        println!("[ThumbSense] {:?} -> {:?} release", code, btn);
                                    }
                                    OutputAction::PassThrough(_) => {
                                        // パススルーは何も表示しない
                                    }
                                }

                                // アクションを実行
                                if let Err(e) = virtual_mouse.execute(action) {
                                    eprintln!("Virtual mouse error: {}", e);
                                }
                            }
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

    Ok(())
}
