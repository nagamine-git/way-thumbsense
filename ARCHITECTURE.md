# way-thumbsense アーキテクチャ設計

## ディレクトリ構成

```
way-thumbsense/
├── Cargo.toml
├── src/
│   ├── main.rs          # エントリーポイント
│   ├── lib.rs           # ライブラリルート
│   ├── core/
│   │   ├── mod.rs
│   │   ├── types.rs     # 共通型定義
│   │   └── mapper.rs    # キーマッピングロジック（純粋関数）
│   ├── input/
│   │   ├── mod.rs
│   │   └── evdev.rs     # evdev実装
│   └── output/
│       ├── mod.rs
│       └── uinput.rs    # uinput実装
└── tests/
    └── mapper_test.rs   # コアロジックのテスト
```

## 型定義

```rust
// === core/types.rs ===

/// タッチパッドの状態
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TouchState {
    pub finger_count: u8,  // 0 = 触れていない, 1+ = 触れている
}

impl TouchState {
    pub fn is_touching(&self) -> bool {
        self.finger_count > 0
    }
}

/// 入力イベント（キーボードから）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyEvent {
    Press(KeyCode),
    Release(KeyCode),
}

/// キーコード
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyCode {
    J,
    K,
    // 拡張用に他のキーも追加可能
}

/// 出力アクション
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputAction {
    MouseClick(MouseButton),
    MouseRelease(MouseButton),
    PassThrough(KeyEvent),  // そのまま通す
}

/// マウスボタン
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
}
```

## コアロジック（純粋関数）

```rust
// === core/mapper.rs ===

use super::types::*;

/// キーイベントをタッチ状態に基づいて変換
///
/// - タッチ中: J → 左クリック, K → 右クリック
/// - 非タッチ: そのままパススルー
pub fn map_key_event(event: KeyEvent, touch: &TouchState) -> OutputAction {
    if !touch.is_touching() {
        return OutputAction::PassThrough(event);
    }

    match event {
        KeyEvent::Press(KeyCode::J) => OutputAction::MouseClick(MouseButton::Left),
        KeyEvent::Release(KeyCode::J) => OutputAction::MouseRelease(MouseButton::Left),
        KeyEvent::Press(KeyCode::K) => OutputAction::MouseClick(MouseButton::Right),
        KeyEvent::Release(KeyCode::K) => OutputAction::MouseRelease(MouseButton::Right),
        _ => OutputAction::PassThrough(event),
    }
}
```

## テスト例

```rust
// === tests/mapper_test.rs ===

use way_thumbsense::core::{map_key_event, KeyEvent, KeyCode, TouchState, OutputAction, MouseButton};

#[test]
fn test_j_to_left_click_when_touching() {
    let touch = TouchState { finger_count: 1 };
    let event = KeyEvent::Press(KeyCode::J);

    assert_eq!(
        map_key_event(event, &touch),
        OutputAction::MouseClick(MouseButton::Left)
    );
}

#[test]
fn test_j_passthrough_when_not_touching() {
    let touch = TouchState { finger_count: 0 };
    let event = KeyEvent::Press(KeyCode::J);

    assert_eq!(
        map_key_event(event, &touch),
        OutputAction::PassThrough(event)
    );
}

#[test]
fn test_k_to_right_click_when_touching() {
    let touch = TouchState { finger_count: 1 };
    let event = KeyEvent::Press(KeyCode::K);

    assert_eq!(
        map_key_event(event, &touch),
        OutputAction::MouseClick(MouseButton::Right)
    );
}
```

## I/O抽象化（将来の拡張用）

```rust
// 入力ソースの抽象化
pub trait InputSource {
    fn next_event(&mut self) -> Option<InputEvent>;
}

pub enum InputEvent {
    Touch(TouchState),
    Key(KeyEvent),
}

// 出力シンクの抽象化
pub trait OutputSink {
    fn send(&mut self, action: OutputAction) -> Result<(), Error>;
}
```

## 実行フロー

```
1. [Startup]
   └─ デバイス検出（タッチパッド、キーボード）
   └─ uinput仮想デバイス作成

2. [Event Loop]
   ┌─────────────────────────────────────────┐
   │  select! {                              │
   │    touchpad_event => update TouchState  │
   │    keyboard_event => {                  │
   │      action = map_key_event(ev, touch)  │
   │      output.send(action)                │
   │    }                                    │
   │  }                                      │
   └─────────────────────────────────────────┘
```

## evdev/uinput実装メモ

### タッチパッド読み取り
```rust
// BTN_TOUCH イベントを監視
// value: 0 = 離れた, 1 = 触れた
```

### キーボード読み取り
```rust
// KEY_* イベントを監視
// value: 0 = release, 1 = press, 2 = repeat
```

### uinput出力
```rust
// VirtualDeviceBuilder で仮想マウス作成
// BTN_LEFT, BTN_RIGHT をサポートするよう設定
```
