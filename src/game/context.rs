use minifb::{Key, KeyRepeat, Window};
use crate::render::color::Color;

pub struct GameContext<'a> {
    window: &'a mut Window,
}

impl<'a> GameContext<'a> {
    pub fn new(window: &'a mut Window) -> GameContext<'a> {
        Self {
            window
        }
    }

    pub fn set_on_top(&mut self, on_top: bool) {
        self.window.topmost(on_top);
    }

    pub fn window(&mut self) -> &Window {
        &self.window
    }

    pub fn set_background_color(&mut self, color: Color) {
        let rgb = color.to_rgb();
        self.window.set_background_color(rgb.0 as usize, rgb.1 as usize, rgb.2 as usize);
    }

    pub fn set_title(&mut self, title: &str) {
        self.window.set_title(title);
    }

    pub fn set_cursor_visible(&mut self, visible: bool) {
        self.window.set_cursor_visibility(visible);
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }

    pub fn exit(&mut self) {
        // TODO: better shutdown
        std::process::exit(0);
    }

    pub fn is_key_down(&self, key: Key) -> bool {
        self.window.is_key_down(key)
    }

    pub fn is_key_pressed(&self, key: Key, repeat: KeyRepeat) -> bool {
        self.window.is_key_pressed(key, repeat)
    }

    pub fn is_key_released(&self, key: Key) -> bool {
        self.window.is_key_released(key)
    }
}