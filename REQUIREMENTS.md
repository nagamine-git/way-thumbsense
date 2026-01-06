# way-thumbsense 要件定義

## 概要

タッチパッドに指が触れている間、特定のキーをマウスクリックに変換するLinux用ツール。
ThumbSenseのRust実装。

## 対象環境

- Linux (EndeavourOS / Arch)
- Hyprland / Sway (Wayland)
- evdev / uinput

## 機能要件

### 基本動作

1. タッチパッドに指が触れている状態で特定キーを押すと、マウスクリックに変換
2. 指が触れていない場合は、キー入力をそのままパススルー

### キーマッピング（最小構成）

| 入力キー | 出力（タッチ中） |
|---------|-----------------|
| J | 左クリック (BTN_LEFT) |
| K | 右クリック (BTN_RIGHT) |

## 技術要件

### 入力検出（evdev）

- タッチパッド: `BTN_TOUCH` イベントで指の接触状態を監視
- キーボード: `KEY_J`, `KEY_K` イベントを監視

### 出力生成（uinput）

- 仮想マウスデバイスを作成
- `BTN_LEFT`, `BTN_RIGHT` イベントを発行

### 必要権限

```bash
# inputグループに追加
sudo usermod -aG input $USER

# udevルール (/etc/udev/rules.d/99-uinput.rules)
KERNEL=="uinput", MODE="0660", GROUP="input"
```

## アーキテクチャ方針

### テスト可能な設計

```
┌─────────────────────────────────────────────────────┐
│                  way-thumbsense                      │
├─────────────────────────────────────────────────────┤
│  Input Layer      │  Core Logic      │  Output Layer │
│  (evdev)          │  (pure, testable)│  (uinput)     │
│                   │                  │               │
│  TouchpadReader   │  KeyMapper       │  VirtualMouse │
│  KeyboardReader   │                  │               │
└─────────────────────────────────────────────────────┘
```

### モジュール構成

- `input`: evdevからのイベント読み取り（トレイトで抽象化）
- `core`: キーマッピングロジック（純粋関数、テスト対象）
- `output`: uinputへのイベント出力（トレイトで抽象化）

## 依存クレート

- `evdev`: evdev読み取り + uinput出力
- `tokio` or `async-std`: 非同期I/O（複数デバイス監視）

## 参考

- [evdev crate](https://docs.rs/evdev)
- [Linux Multi-Touch Protocol](https://docs.kernel.org/input/multi-touch-protocol.html)
- [fusuma-plugin-thumbsense](https://github.com/iberianpig/fusuma-plugin-thumbsense)
