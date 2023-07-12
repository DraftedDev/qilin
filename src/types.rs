use crate::render::color::Color;
use minifb::{Scale, ScaleMode, WindowOptions};
use std::time::Duration;

/// Game Configuration containing title, max fps, width, height and window options.
pub struct GameConfig {
    /// Window title. Default is "New Game".
    pub title: String,
    /// Maximum FPS/Update Rate. Default is 60 fps.
    pub update_rate_limit: UpdateRate,
    /// Width of the window. Default is 800.
    pub width: usize,
    /// Height of the window. Default is 600.
    pub height: usize,
    /// How many times the [crate::scene::Scene::fixed_update] function is called in one second. Default is 50.
    pub fixed_update_interval: usize,
    /// Window options.
    pub window: WindowOptions,
}

impl Default for GameConfig {
    #[inline]
    fn default() -> Self {
        Self {
            title: String::from("New Game"),
            update_rate_limit: FPS60,
            width: 800,
            height: 600,
            fixed_update_interval: 50,
            window: WindowOptions {
                borderless: false,
                title: true,
                resize: true,
                scale: Scale::X1,
                scale_mode: ScaleMode::AspectRatioStretch,
                topmost: false,
                transparency: false,
                none: false,
            },
        }
    }
}

/// Update Rate as Duration. Use `types::FPS30`, `types::FPS60` or `types::FPS120` for FPS 30, 60 or 120 fps.
pub type UpdateRate = Duration;
pub const FPS30: Duration = Duration::from_nanos(33333333);
pub const FPS60: Duration = Duration::from_nanos(16666666);
pub const FPS120: Duration = Duration::from_nanos(8333333);

/// Vector of colors to represent an image.
pub type Image = Vec<Color>;
