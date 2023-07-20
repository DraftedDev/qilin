use crate::render::color::Color;
use minifb::{Key, KeyRepeat, Window};

/// Game controller to execute game logic and get input.
#[derive(Debug)]
pub struct GameContext<'a> {
    window: &'a mut Window,
}

impl<'a> GameContext<'a> {
    /// Create context from a [Window].
    #[inline]
    pub fn new(window: &'a mut Window) -> GameContext<'a> { Self { window } }

    /// Toggle if the window is topmost.
    #[inline]
    pub fn set_on_top(&mut self, on_top: bool) { self.window.topmost(on_top); }

    /// Get the minifb [Window].
    #[inline]
    pub fn window(&mut self) -> &Window { &self.window }

    /// Set the background color of the window.\
    /// Use [crate::render::canvas::Canvas::clear] to clear canvas color.
    #[inline]
    pub fn set_background_color(&mut self, color: Color) {
        let rgb = color.to_rgb();
        self.window
            .set_background_color(rgb.0 as usize, rgb.1 as usize, rgb.2 as usize);
    }

    /// Set the window title.
    #[inline]
    pub fn set_title(&mut self, title: &str) { self.window.set_title(title); }

    /// Set cursor visibility.
    #[inline]
    pub fn set_cursor_visible(&mut self, visible: bool) {
        self.window.set_cursor_visibility(visible);
    }

    /// Check if the window is open.
    pub fn is_open(&self) -> bool { self.window.is_open() }

    /// Exit game.
    /// You can also just use [std::process::exit], since this doesn't do anything else (in the moment).
    #[inline]
    pub fn exit(&mut self) {
        // TODO: better shutdown
        std::process::exit(0);
    }

    /// Check if a key is down.
    #[inline]
    pub fn is_key_down(&self, key: Key) -> bool { self.window.is_key_down(key) }

    /// Check if a key is pressed.
    #[inline]
    pub fn is_key_pressed(&self, key: Key, repeat: KeyRepeat) -> bool {
        self.window.is_key_pressed(key, repeat)
    }

    /// Check if a key is released
    #[inline]
    pub fn is_key_released(&self, key: Key) -> bool { self.window.is_key_released(key) }
}
