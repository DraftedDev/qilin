use crate::render::color::Color;
use crate::render::sketch::{Operation, Sketch};
use crate::types::Button;
use minifb::{Key, KeyRepeat, MouseButton, MouseMode, Window};

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

    /// Check if mouse button is down.
    #[inline]
    pub fn is_mouse_down(&self, button: MouseButton) -> bool { self.window.get_mouse_down(button) }

    pub fn is_button_down(&self, button: Button) -> bool {
        match button {
            Button::Keyboard(key) => self.is_key_down(key),
            Button::Mouse(btn) => self.is_mouse_down(btn),
        }
    }

    /// Get mouse position.
    ///
    /// Returns `None` if the mouse is outside of the window.
    pub fn get_mouse_pos(&self) -> Option<(f32, f32)> {
        self.window.get_mouse_pos(MouseMode::Clamp)
    }

    /// Check if the mouse is hovering on top of (Drawing)-Operation.
    #[inline]
    pub fn hovering_on(&self, operation: &Operation) -> bool {
        let pos = self.get_mouse_pos();
        if pos.is_none() {
            return false;
        }

        let (mouse_x, mouse_y) = pos.unwrap();

        match operation {
            Operation::Line { from, to, color } => {
                // Calculate the bounding box of the line
                let min_x = from.x.min(to.x);
                let max_x = from.x.max(to.x);
                let min_y = from.y.min(to.y);
                let max_y = from.y.max(to.y);

                // Check if the mouse position is within the bounding box of the line
                mouse_x >= min_x as f32
                    && mouse_x <= max_x as f32
                    && mouse_y >= min_y as f32
                    && mouse_y <= max_y as f32
            }
            Operation::Circle { radius, color, pos } => {
                // Calculate the distance between the mouse position and the circle center
                let dist_x = mouse_x - pos.x as f32;
                let dist_y = mouse_y - pos.y as f32;
                let distance_squared = dist_x * dist_x + dist_y * dist_y;

                // Check if the distance is less than or equal to the square of the circle radius
                distance_squared <= (radius * radius) as f32
            }
            Operation::Rect {
                color,
                pos,
                width,
                height,
            } => {
                // Check if the mouse position is within the rectangle boundaries
                mouse_x >= pos.x as f32
                    && mouse_x <= (pos.x + width) as f32
                    && mouse_y >= pos.y as f32
                    && mouse_y <= (pos.y + height) as f32
            }
            Operation::Image {
                pos,
                width,
                height,
                data,
            } => {
                // Check if the mouse position is within the image boundaries
                mouse_x >= pos.x as f32
                    && mouse_x <= (pos.x + width) as f32
                    && mouse_y >= pos.y as f32
                    && mouse_y <= (pos.y + height) as f32
            }
            Operation::Oval {
                color,
                pos,
                width,
                height,
            } => {
                // Calculate the distance between the mouse position and the oval center
                let center_x = pos.x + width / 2;
                let center_y = pos.y + height / 2;
                let dist_x = mouse_x - center_x as f32;
                let dist_y = mouse_y - center_y as f32;
                let distance_squared = (dist_x * dist_x * (*height as f32) * (*height as f32))
                    + (dist_y * dist_y * (*width as f32) * (*width as f32));

                // Check if the distance is less than or equal to the square of the largest radius (half of the width or height)
                distance_squared <= ((width * width * height * height) / 4) as f32
            }
        }
    }

    /// Check if the mouse is hovering on a sketch.
    #[inline]
    pub fn hovering_on_sketch(&self, sketch: &Sketch) -> bool {
        for op in sketch.get_operations() {
            if self.hovering_on(op) {
                return true;
            }
        }
        return false;
    }

    /// Check if the mouse has clicked on an operation
    #[inline]
    pub fn clicked_on(&self, operation: &Operation, btn: Button) -> bool {
        if self.hovering_on(operation) && self.is_button_down(btn) {
            true
        } else {
            false
        }
    }

    /// Check if the mouse has clicked on a sketch
    #[inline]
    pub fn clicked_on_sketch(&self, sketch: &Sketch, btn: Button) -> bool {
        if self.hovering_on_sketch(sketch) && self.is_button_down(btn) {
            true
        } else {
            false
        }
    }
}
