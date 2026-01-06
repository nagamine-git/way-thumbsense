/// タッチパッドの状態
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TouchState {
    pub finger_count: u8,
}

impl TouchState {
    pub fn is_touching(&self) -> bool {
        self.finger_count > 0
    }
}

/// キーコード
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyCode {
    J,
    K,
}

/// 入力イベント（キーボードから）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyEvent {
    Press(KeyCode),
    Release(KeyCode),
}

/// マウスボタン
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
}

/// 出力アクション
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputAction {
    MouseClick(MouseButton),
    MouseRelease(MouseButton),
    PassThrough(KeyEvent),
}
