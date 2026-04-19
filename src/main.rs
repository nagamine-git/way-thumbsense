//! way-thumbsense - ThumbSense for Linux/Wayland
//!
//! タッチパッドに触れている間、仮想キー(F24)を押し続ける
//! keydでF24をmousenavレイヤーのトリガーにすることでThumbSenseを実現

use clap::Parser;
use evdev::{AbsoluteAxisType, InputEventKind, Key};
use std::thread;
use std::time::Duration;
use way_thumbsense::input::{find_touchpad, get_touchpad_dimensions};
use way_thumbsense::output::VirtualDevice;
use way_thumbsense::tracker::{ExclusionZones, TouchTracker};

/// ThumbSense implementation for Linux/Wayland
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 上端の除外割合 (0.0 - 100.0)
    #[arg(long, default_value_t = 0.0)]
    exclude_top: f32,

    /// 下端の除外割合 (0.0 - 100.0)
    #[arg(long, default_value_t = 0.0)]
    exclude_bottom: f32,

    /// 左端の除外割合 (0.0 - 100.0)
    #[arg(long, default_value_t = 0.0)]
    exclude_left: f32,

    /// 右端の除外割合 (0.0 - 100.0)
    #[arg(long, default_value_t = 0.0)]
    exclude_right: f32,

    /// デバッグ情報を表示
    #[arg(long, default_value_t = false)]
    debug: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    println!("way-thumbsense starting...");

    // タッチパッドを検出（見つからなければリトライ）
    let mut touchpad = wait_for_touchpad();
    println!("Touchpad: {}", touchpad.name().unwrap_or("unknown"));

    // タッチパッドの寸法を取得
    let mut dimensions = get_touchpad_dimensions(&touchpad)
        .ok_or_else(|| anyhow::anyhow!("Failed to get touchpad dimensions"))?;
    println!(
        "Touchpad dimensions: X({} to {}), Y({} to {})",
        dimensions.min_x, dimensions.max_x, dimensions.min_y, dimensions.max_y
    );

    // 除外領域の設定
    let exclusion_zones = ExclusionZones::new(
        args.exclude_top,
        args.exclude_bottom,
        args.exclude_left,
        args.exclude_right,
    );

    if exclusion_zones.top > 0.0
        || exclusion_zones.bottom > 0.0
        || exclusion_zones.left > 0.0
        || exclusion_zones.right > 0.0
    {
        println!(
            "Exclusion zones: top={}%, bottom={}%, left={}%, right={}%",
            exclusion_zones.top, exclusion_zones.bottom, exclusion_zones.left, exclusion_zones.right
        );
    } else {
        println!("Exclusion zones: disabled");
    }

    // タッチトラッカーを初期化
    let mut tracker = TouchTracker::new(dimensions, exclusion_zones);

    // 仮想キーボードを作成
    let mut vdev = VirtualDevice::new()?;
    println!("Virtual keyboard created");

    println!("\nRunning... (Ctrl+C to exit)");
    println!("Touch trackpad to activate F24 (mousenav layer)\n");

    let mut is_touching = false;
    let mut f24_active = false; // F24が実際に押されているか

    loop {
        let fetch_result = touchpad
            .fetch_events()
            .map(|events| events.collect::<Vec<_>>());
        match fetch_result {
            Ok(events_vec) => {

                // 1st pass: 座標を更新
                for ev in &events_vec {
                    match ev.kind() {
                        InputEventKind::AbsAxis(AbsoluteAxisType::ABS_X)
                        | InputEventKind::AbsAxis(AbsoluteAxisType::ABS_MT_POSITION_X) => {
                            tracker.update_x(ev.value());
                        }
                        InputEventKind::AbsAxis(AbsoluteAxisType::ABS_Y)
                        | InputEventKind::AbsAxis(AbsoluteAxisType::ABS_MT_POSITION_Y) => {
                            tracker.update_y(ev.value());
                        }
                        _ => {}
                    }
                }

                // 2nd pass: BTN_TOUCHを処理
                for ev in &events_vec {
                    if let InputEventKind::Key(key) = ev.kind() {
                        if key == Key::BTN_TOUCH {
                            let now_touching = ev.value() == 1;

                            if now_touching != is_touching {
                                is_touching = now_touching;

                                if is_touching {
                                    // タッチ開始
                                    let in_exclusion = tracker.is_in_exclusion_zone();

                                    if args.debug {
                                        println!(
                                            "[Touch] {} -> excluded: {}",
                                            tracker.debug_info(),
                                            in_exclusion
                                        );
                                    }

                                    if !in_exclusion {
                                        // 除外領域外でのタッチ → F24を押す
                                        if args.debug {
                                            println!("[Touch] -> F24 press");
                                        }
                                        vdev.forward_key(Key::KEY_F24, 1)?;
                                        f24_active = true;
                                    } else if args.debug {
                                        println!("[Touch] -> F24 not pressed (in exclusion zone)");
                                    }
                                } else {
                                    // タッチ終了
                                    if f24_active {
                                        // F24を離す
                                        if args.debug {
                                            println!("[Touch] -> F24 release");
                                        }
                                        vdev.forward_key(Key::KEY_F24, 0)?;
                                        f24_active = false;
                                    }
                                    tracker.reset();
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Touchpad error: {} — reconnecting...", e);

                // 押しっぱなしのF24を解放
                if f24_active {
                    let _ = vdev.forward_key(Key::KEY_F24, 0);
                    f24_active = false;
                }
                is_touching = false;
                tracker.reset();

                // タッチパッドの再接続を待つ
                touchpad = wait_for_touchpad();
                println!("Touchpad reconnected: {}", touchpad.name().unwrap_or("unknown"));

                // 寸法を再取得（接続し直しで変わる可能性は低いが念のため）
                if let Some(d) = get_touchpad_dimensions(&touchpad) {
                    if d.min_x != dimensions.min_x
                        || d.max_x != dimensions.max_x
                        || d.min_y != dimensions.min_y
                        || d.max_y != dimensions.max_y
                    {
                        dimensions = d;
                        tracker = TouchTracker::new(dimensions, exclusion_zones);
                        println!(
                            "Touchpad dimensions updated: X({} to {}), Y({} to {})",
                            dimensions.min_x, dimensions.max_x, dimensions.min_y, dimensions.max_y
                        );
                    }
                }
            }
        }
    }
}

/// タッチパッドが見つかるまで待機する
fn wait_for_touchpad() -> evdev::Device {
    let mut logged = false;
    loop {
        match find_touchpad() {
            Ok(dev) => return dev,
            Err(e) => {
                if !logged {
                    eprintln!("Waiting for touchpad: {}", e);
                    logged = true;
                }
                thread::sleep(Duration::from_secs(1));
            }
        }
    }
}
