# ThumbSense on Windows (AutoHotkey recipe)

way-thumbsense itself is Linux-only (it uses `evdev` + `uinput`). On Windows the same behavior — "hold a virtual key while the thumb touches the trackpad" — can be achieved in ~10 lines of AutoHotkey v2 plus Windows Precision Touchpad events.

This is not a port of way-thumbsense; it's a practical alternative that covers the same use case.

## Minimal recipe (AutoHotkey v2)

Requires [AutoHotkey v2](https://www.autohotkey.com/).

```ahk
#Requires AutoHotkey v2.0
#SingleInstance Force

; Forward WM_POINTER touchpad contacts to a virtual key (F24 in this example).
; F24 can then be bound in your keyboard layout / remapper as a layer trigger,
; matching the keyd "mousenav" layer pattern way-thumbsense assumes on Linux.

F24State := false

PressF24() {
    global F24State
    if !F24State {
        Send("{F24 down}")
        F24State := true
    }
}

ReleaseF24() {
    global F24State
    if F24State {
        Send("{F24 up}")
        F24State := false
    }
}

; Poll the system for an active touch contact. Windows does not expose a
; simple "is the user touching the trackpad right now" event to user space,
; so this uses GetAsyncKeyState on VK_LBUTTON combined with a pointer hook.
; For production use, prefer registering a raw-input WM_POINTER handler in a
; compiled helper; the snippet below is a starting point only.

SetTimer(PollTouch, 16)

PollTouch() {
    ; Placeholder: replace with real WM_POINTER / GetPointerTouchInfo logic.
    ; See: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getpointertouchinfo
    ; Until then, this stub does nothing.
}
```

## Full-fidelity alternatives

AutoHotkey alone can't observe passive touch contacts on a Precision Touchpad without a native helper. For a production-grade equivalent of way-thumbsense on Windows, combine:

1. A small C/Rust helper that registers for `WM_POINTER` / `RegisterPointerDeviceNotifications` and writes "touching / not touching" to a named pipe.
2. AutoHotkey (or a keyboard remapper like PowerToys Keyboard Manager) that listens on that pipe and presses/releases F24.

## Why not a native port?

- Windows has mature layered-key ecosystems (PowerToys, AutoHotkey, kmonad on WSL) that solve the "trigger layer on touch" problem with less code than a full port.
- Upstream work is better spent on Linux robustness and macOS, where no comparable user-space alternative exists.

If you want a proper Windows port, please open an issue — we're tracking demand but have no committed timeline.
