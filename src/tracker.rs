//! タッチ位置の追跡と除外領域の判定

use crate::input::TouchpadDimensions;

/// 除外領域の設定(パーセンテージ)
#[derive(Debug, Clone, Copy)]
pub struct ExclusionZones {
    pub top: f32,    // 上端の除外割合 (0.0 - 100.0)
    pub bottom: f32, // 下端の除外割合 (0.0 - 100.0)
    pub left: f32,   // 左端の除外割合 (0.0 - 100.0)
    pub right: f32,  // 右端の除外割合 (0.0 - 100.0)
}

impl ExclusionZones {
    pub fn new(top: f32, bottom: f32, left: f32, right: f32) -> Self {
        Self {
            top,
            bottom,
            left,
            right,
        }
    }

    /// 除外領域なし
    pub fn none() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }
}

/// タッチ追跡と除外領域判定
#[derive(Debug)]
pub struct TouchTracker {
    dimensions: TouchpadDimensions,
    exclusion_zones: ExclusionZones,
    current_x: Option<i32>,
    current_y: Option<i32>,
}

impl TouchTracker {
    pub fn new(dimensions: TouchpadDimensions, exclusion_zones: ExclusionZones) -> Self {
        Self {
            dimensions,
            exclusion_zones,
            current_x: None,
            current_y: None,
        }
    }

    /// X座標を更新
    pub fn update_x(&mut self, x: i32) {
        self.current_x = Some(x);
    }

    /// Y座標を更新
    pub fn update_y(&mut self, y: i32) {
        self.current_y = Some(y);
    }

    /// 座標をリセット
    pub fn reset(&mut self) {
        self.current_x = None;
        self.current_y = None;
    }

    /// 現在のタッチ位置が除外領域にあるかチェック
    pub fn is_in_exclusion_zone(&self) -> bool {
        let Some(x) = self.current_x else {
            return false;
        };
        let Some(y) = self.current_y else {
            return false;
        };

        // 各端からの距離を計算
        let left_threshold = (self.dimensions.max_x as f32 * self.exclusion_zones.left / 100.0) as i32;
        let right_threshold =
            (self.dimensions.max_x as f32 * (100.0 - self.exclusion_zones.right) / 100.0) as i32;
        let top_threshold = (self.dimensions.max_y as f32 * self.exclusion_zones.top / 100.0) as i32;
        let bottom_threshold =
            (self.dimensions.max_y as f32 * (100.0 - self.exclusion_zones.bottom) / 100.0) as i32;

        // 除外領域内かどうか判定
        x < left_threshold || x > right_threshold || y < top_threshold || y > bottom_threshold
    }

    /// デバッグ用: 現在の座標と除外判定を表示
    pub fn debug_info(&self) -> String {
        match (self.current_x, self.current_y) {
            (Some(x), Some(y)) => {
                format!(
                    "pos: ({}, {}) / max: ({}, {}) / excluded: {}",
                    x,
                    y,
                    self.dimensions.max_x,
                    self.dimensions.max_y,
                    self.is_in_exclusion_zone()
                )
            }
            _ => "pos: unknown".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_exclusion() {
        let dims = TouchpadDimensions {
            max_x: 1000,
            max_y: 1000,
        };
        let mut tracker = TouchTracker::new(dims, ExclusionZones::none());

        tracker.update_x(500);
        tracker.update_y(500);
        assert!(!tracker.is_in_exclusion_zone());

        tracker.update_x(0);
        tracker.update_y(0);
        assert!(!tracker.is_in_exclusion_zone());
    }

    #[test]
    fn test_with_exclusion() {
        let dims = TouchpadDimensions {
            max_x: 1000,
            max_y: 1000,
        };
        // 各端から10%を除外
        let zones = ExclusionZones::new(10.0, 10.0, 10.0, 10.0);
        let mut tracker = TouchTracker::new(dims, zones);

        // 中央は除外されない
        tracker.update_x(500);
        tracker.update_y(500);
        assert!(!tracker.is_in_exclusion_zone());

        // 左端は除外される (x < 100)
        tracker.update_x(50);
        tracker.update_y(500);
        assert!(tracker.is_in_exclusion_zone());

        // 右端は除外される (x > 900)
        tracker.update_x(950);
        tracker.update_y(500);
        assert!(tracker.is_in_exclusion_zone());

        // 上端は除外される (y < 100)
        tracker.update_x(500);
        tracker.update_y(50);
        assert!(tracker.is_in_exclusion_zone());

        // 下端は除外される (y > 900)
        tracker.update_x(500);
        tracker.update_y(950);
        assert!(tracker.is_in_exclusion_zone());
    }
}
