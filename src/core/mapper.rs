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
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // === タッチ中のテスト ===

    #[test]
    fn j_press_to_left_click_when_touching() {
        let touch = TouchState { finger_count: 1 };
        let event = KeyEvent::Press(KeyCode::J);

        assert_eq!(
            map_key_event(event, &touch),
            OutputAction::MouseClick(MouseButton::Left)
        );
    }

    #[test]
    fn j_release_to_left_release_when_touching() {
        let touch = TouchState { finger_count: 1 };
        let event = KeyEvent::Release(KeyCode::J);

        assert_eq!(
            map_key_event(event, &touch),
            OutputAction::MouseRelease(MouseButton::Left)
        );
    }

    #[test]
    fn k_press_to_right_click_when_touching() {
        let touch = TouchState { finger_count: 1 };
        let event = KeyEvent::Press(KeyCode::K);

        assert_eq!(
            map_key_event(event, &touch),
            OutputAction::MouseClick(MouseButton::Right)
        );
    }

    #[test]
    fn k_release_to_right_release_when_touching() {
        let touch = TouchState { finger_count: 1 };
        let event = KeyEvent::Release(KeyCode::K);

        assert_eq!(
            map_key_event(event, &touch),
            OutputAction::MouseRelease(MouseButton::Right)
        );
    }

    // === 非タッチ時のテスト ===

    #[test]
    fn j_passthrough_when_not_touching() {
        let touch = TouchState { finger_count: 0 };
        let event = KeyEvent::Press(KeyCode::J);

        assert_eq!(
            map_key_event(event, &touch),
            OutputAction::PassThrough(event)
        );
    }

    #[test]
    fn k_passthrough_when_not_touching() {
        let touch = TouchState { finger_count: 0 };
        let event = KeyEvent::Press(KeyCode::K);

        assert_eq!(
            map_key_event(event, &touch),
            OutputAction::PassThrough(event)
        );
    }

    // === 複数指のテスト ===

    #[test]
    fn j_to_left_click_with_multiple_fingers() {
        let touch = TouchState { finger_count: 2 };
        let event = KeyEvent::Press(KeyCode::J);

        assert_eq!(
            map_key_event(event, &touch),
            OutputAction::MouseClick(MouseButton::Left)
        );
    }
}
