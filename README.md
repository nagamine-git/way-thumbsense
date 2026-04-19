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

## 配布

- **Arch / AUR**: `paru -S way-thumbsense-git` (https://aur.archlinux.org/packages/way-thumbsense-git)
- **その他 Linux (x86_64 / aarch64)**: [GitHub Releases](https://github.com/nagamine-git/way-thumbsense/releases) のプリビルド tarball (musl 静的リンク)。展開して `./install.sh`。
- **Nix**: `nix run github:nagamine-git/way-thumbsense`

## systemd user service

AUR パッケージおよびプリビルド tarball には `way-thumbsense.service` (user unit) が含まれます。自動起動と落ちたときの再起動:

```bash
systemctl --user enable --now way-thumbsense.service
```

## macOS / Windows

- macOS: 未対応。計画は [docs/platform-support.md](docs/platform-support.md) を参照。
- Windows: 未対応。AutoHotkey で同等を実現するレシピが [docs/windows-ahk.md](docs/windows-ahk.md) にあります。

## ライセンス

MIT
