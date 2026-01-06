//! uinput仮想マウスデバイス

use crate::core::{MouseButton, OutputAction};
use evdev::{uinput::VirtualDeviceBuilder, AttributeSet, InputEvent, Key};
use std::io;

/// 仮想マウスデバイス
pub struct VirtualMouse {
    device: evdev::uinput::VirtualDevice,
}

impl VirtualMouse {
    /// 仮想マウスデバイスを作成
    pub fn new() -> io::Result<Self> {
        let mut keys = AttributeSet::<Key>::new();
        keys.insert(Key::BTN_LEFT);
        keys.insert(Key::BTN_RIGHT);
        keys.insert(Key::BTN_MIDDLE);

        let device = VirtualDeviceBuilder::new()?
            .name("way-thumbsense virtual mouse")
            .with_keys(&keys)?
            .build()?;

        Ok(Self { device })
    }

    /// マウスボタンを押す
    pub fn click(&mut self, button: MouseButton) -> io::Result<()> {
        let key = match button {
            MouseButton::Left => Key::BTN_LEFT,
            MouseButton::Right => Key::BTN_RIGHT,
        };

        let events = [
            InputEvent::new(evdev::EventType::KEY, key.code(), 1),
            InputEvent::new(evdev::EventType::SYNCHRONIZATION, 0, 0),
        ];

        self.device.emit(&events)
    }

    /// マウスボタンを離す
    pub fn release(&mut self, button: MouseButton) -> io::Result<()> {
        let key = match button {
            MouseButton::Left => Key::BTN_LEFT,
            MouseButton::Right => Key::BTN_RIGHT,
        };

        let events = [
            InputEvent::new(evdev::EventType::KEY, key.code(), 0),
            InputEvent::new(evdev::EventType::SYNCHRONIZATION, 0, 0),
        ];

        self.device.emit(&events)
    }

    /// OutputActionを実行
    pub fn execute(&mut self, action: OutputAction) -> io::Result<()> {
        match action {
            OutputAction::MouseClick(button) => self.click(button),
            OutputAction::MouseRelease(button) => self.release(button),
            OutputAction::PassThrough(_) => Ok(()), // パススルーは何もしない
        }
    }
}
