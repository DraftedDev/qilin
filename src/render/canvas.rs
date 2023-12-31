use crate::math::buffer::Buffer;
use crate::render::color::Color;
use crate::render::sketch::{Drawable, Sketch};

/// Canvas of a game, containing a buffer of pixels to draw to the window.
#[derive(Clone)]
pub struct Canvas {
    buffer: Buffer,
}

impl Canvas {
    /// Create a new canvas from width and height.
    #[inline]
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            buffer: Buffer::new(width, height),
        }
    }

    /// Returns the buffer as [Vec] containing 0RGB pixels.
    #[inline]
    pub fn buffer(self) -> Buffer { self.buffer }

    /// Resize the canvas.
    #[inline(never)]
    pub fn resize(&mut self, new_width: usize, new_height: usize) {
        let new_size = new_width * new_height;

        let mut new_buffer: Vec<u32> = vec![0; new_size];

        let copy_width = usize::min(self.width(), new_width);
        let copy_height = usize::min(self.height(), new_height);

        for y in 0..copy_height {
            let source_offset = y * self.width();
            let target_offset = y * new_width;
            new_buffer[target_offset..target_offset + copy_width]
                .copy_from_slice(&self.buffer.buffer[source_offset..source_offset + copy_width]);
        }

        self.buffer.width = new_width;
        self.buffer.height = new_height;
        self.buffer.buffer = new_buffer;
    }

    /// Set pixel at `x` and `y` to `color`.
    #[inline]
    pub fn set_pixel(&mut self, x: usize, y: usize, color: &Color) {
        // check for overflows
        if let Some(cy) = y.checked_mul(self.width()) {
            // check for overflows
            if let Some(cx) = cy.checked_add(x) {
                // check for index out of bounds
                if let Some(px) = self.buffer.buffer.get_mut(cx) {
                    *px = color.0;
                }
            }
        }
    }

    /// Get pixel at `x` and `y` to `color`.
    #[inline]
    pub fn get_pixel(&self, x: usize, y: usize) -> Option<Color> {
        if let Some(cy) = y.checked_mul(self.width()) {
            if let Some(cx) = cy.checked_add(x) {
                if let Some(px) = self.buffer.buffer.get(cx) {
                    return Some(Color(*px));
                }
            }
        }
        None
    }

    /// Clear the canvas with `color`.\
    /// **NOTE**: Canvas does not always match window dimensions. To change the background color of the window, use [crate::game::context::GameContext].
    #[inline(never)]
    pub fn clear(&mut self, color: &Color) {
        for y in 0..self.height() {
            for x in 0..self.width() {
                self.set_pixel(x, y, color);
            }
        }
    }

    /// Clears the canvas with black pixels.\
    /// Shorthand for [Canvas::clear(&Color::BLACK)].
    #[inline]
    pub fn cleanse(&mut self) { self.clear(&Color(0)) }

    /// Draw a [Sketch] to the canvas.
    #[inline]
    pub fn draw(&mut self, sketch: &Sketch) {
        for op in &sketch.0 {
            op.apply(self);
        }
    }

    /// Draw a [Drawable] to the canvas.
    #[inline]
    pub fn drawable<T: Drawable>(&mut self, drawable: &T) { drawable.apply(self); }

    /// Get window width.
    #[inline]
    pub fn width(&self) -> usize { self.buffer.width }

    /// Get window height.
    #[inline]
    pub fn height(&self) -> usize { self.buffer.height }
}
