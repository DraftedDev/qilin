use std::time::Duration;
use minifb::{Scale, ScaleMode, WindowOptions};
use crate::render::color::Color;

pub struct GameConfig {
    pub title: String,
    pub update_rate_limit: UpdateRate,
    pub width: usize,
    pub height: usize,
    pub window: WindowOptions,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            title: String::from("New Game"),
            update_rate_limit: FPS60,
            width: 800,
            height: 600,
            window: WindowOptions {
                borderless: false,
                title: true,
                resize: true,
                scale: Scale::X1,
                scale_mode: ScaleMode::AspectRatioStretch,
                topmost: false,
                transparency: false,
                none: false,
            }
        }
    }
}

pub type UpdateRate = Duration;
pub const FPS30: Duration = Duration::from_nanos(33333333);
pub const FPS60: Duration = Duration::from_nanos(16666666);
pub const FPS120: Duration = Duration::from_nanos(8333333);

pub type Image = Vec<Color>;