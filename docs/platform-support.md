# Platform support plan

## Current

- **Linux (Wayland / X11 agnostic)**: fully supported. Reads raw events via `evdev` and emits a virtual keyboard via `uinput`. Compositor-agnostic — works under sway, Hyprland, KDE, GNOME, plain X11.

## Planned crate layout (not yet applied)

When we add macOS support, the code will be restructured as:

```
thumbsense-core/    # platform-agnostic: TouchTracker, ExclusionZones, CLI
thumbsense-linux/   # evdev + uinput
thumbsense-macos/   # IOKit HID + CGEventPost
way-thumbsense/     # thin binary crate selecting the backend via #[cfg]
```

The current single-crate layout already isolates the platform-specific code in `src/input/evdev_input.rs` and `src/output/` behind traits, so the split should be mechanical.

## macOS

Blocked on someone with a Mac to write the IOKit/CGEvent backend. Tracked in a separate issue — no committed timeline.

Constraints worth knowing ahead of time:
- Requires Accessibility permission (granted per-binary in System Settings → Privacy & Security).
- `CGEventPost` can post synthetic key events; no uinput-equivalent device required.
- `IOHIDManager` can observe trackpad contact count via the `kIOHIDDigitizerContactCountKey` usage.

Well-established alternatives on macOS ([Karabiner-Elements](https://karabiner-elements.pqrs.org/), [BetterTouchTool](https://folivora.ai/)) already cover this use case, so a native port is a "nice to have," not a priority.

## Windows

No native port planned. See [docs/windows-ahk.md](./windows-ahk.md) for an AutoHotkey-based alternative that achieves the same outcome.

## BSDs

Not supported. `evdev` in particular is Linux-specific. FreeBSD has `evdev` via `cuse` but `uinput` is incomplete; no work planned.
