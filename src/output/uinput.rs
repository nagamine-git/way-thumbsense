//! uinput仮想デバイス

use crate::core::{MouseButton, OutputAction};
use evdev::{uinput::VirtualDeviceBuilder, AttributeSet, InputEvent, Key};
use std::io;

/// 仮想マウス + キーボードデバイス
pub struct VirtualDevice {
    mouse: evdev::uinput::VirtualDevice,
    keyboard: evdev::uinput::VirtualDevice,
}

impl VirtualDevice {
    /// 仮想デバイスを作成
    pub fn new() -> io::Result<Self> {
        // マウス
        let mut mouse_keys = AttributeSet::<Key>::new();
        mouse_keys.insert(Key::BTN_LEFT);
        mouse_keys.insert(Key::BTN_RIGHT);
        mouse_keys.insert(Key::BTN_MIDDLE);

        let mouse = VirtualDeviceBuilder::new()?
            .name("way-thumbsense mouse")
            .with_keys(&mouse_keys)?
            .build()?;

        // キーボード（全キー対応）
        let mut kb_keys = AttributeSet::<Key>::new();
        for code in 1..=566u16 {
            kb_keys.insert(Key::new(code));
        }

        let keyboard = VirtualDeviceBuilder::new()?
            .name("way-thumbsense keyboard")
            .with_keys(&kb_keys)?
            .build()?;

        // デバイスがシステムに認識されるまで少し待つ
        std::thread::sleep(std::time::Duration::from_millis(100));

        Ok(Self { mouse, keyboard })
    }

    /// マウスクリック
    pub fn mouse_click(&mut self, button: MouseButton) -> io::Result<()> {
        let key = match button {
            MouseButton::Left => Key::BTN_LEFT,
            MouseButton::Right => Key::BTN_RIGHT,
        };

        self.mouse.emit(&[
            InputEvent::new(evdev::EventType::KEY, key.code(), 1),
            InputEvent::new(evdev::EventType::SYNCHRONIZATION, 0, 0),
        ])
    }

    /// マウスリリース
    pub fn mouse_release(&mut self, button: MouseButton) -> io::Result<()> {
        let key = match button {
            MouseButton::Left => Key::BTN_LEFT,
            MouseButton::Right => Key::BTN_RIGHT,
        };

        self.mouse.emit(&[
            InputEvent::new(evdev::EventType::KEY, key.code(), 0),
            InputEvent::new(evdev::EventType::SYNCHRONIZATION, 0, 0),
        ])
    }

    /// キーイベントをそのまま転送
    pub fn forward_key(&mut self, key: Key, value: i32) -> io::Result<()> {
        self.keyboard.emit(&[
            InputEvent::new(evdev::EventType::KEY, key.code(), value),
            InputEvent::new(evdev::EventType::SYNCHRONIZATION, 0, 0),
        ])
    }

    /// OutputActionを実行
    pub fn execute(&mut self, action: OutputAction) -> io::Result<()> {
        match action {
            OutputAction::MouseClick(button) => self.mouse_click(button),
            OutputAction::MouseRelease(button) => self.mouse_release(button),
            OutputAction::PassThrough(_) => Ok(()), // パススルーは別途forward_keyで処理
        }
    }
}

// 後方互換性のためのエイリアス
pub type VirtualMouse = VirtualDevice;
