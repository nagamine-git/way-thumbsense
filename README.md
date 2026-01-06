# way-thumbsense

ThumbSense for Linux/Wayland - タッチパッドに触れながらキーを押すとマウスクリックに変換

## 機能

| 入力 | 出力（タッチ中） |
|------|-----------------|
| J | 左クリック |
| K | 右クリック |

タッチパッドに触れていない時は通常のキー入力。

## 必要環境

- Linux (Arch/EndeavourOS etc.)
- Wayland (Hyprland/Sway) or X11
- evdev対応タッチパッド

## セットアップ

```bash
# inputグループに追加
sudo usermod -aG input $USER

# udevルール設定
sudo tee /etc/udev/rules.d/99-uinput.rules << 'EOF'
KERNEL=="uinput", MODE="0660", GROUP="input"
EOF

sudo udevadm control --reload-rules
sudo udevadm trigger

# ログアウト/ログインして反映
```

## ビルド・実行

```bash
cargo build --release
cargo run --release
```

## 動作確認ツール

```bash
# デバイス一覧
cargo run --example check_devices

# イベント監視
cargo run --example watch_events
```

## アーキテクチャ

```
src/
├── core/       # キーマッピングロジック（純粋関数、テスト可能）
├── input/      # evdevデバイス読み取り
└── output/     # uinput仮想マウス
```

## テスト

```bash
cargo test
```

## ライセンス

MIT
