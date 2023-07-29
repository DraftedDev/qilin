use crate::render::canvas::Canvas;
use crate::render::color::Color;
use crate::render::sketch::Drawable;
use fontdue::layout::{CoordinateSystem, Layout, TextStyle};
use fontdue::Font;
use mint::Vector2;

/// Sketch to draw text on a canvas.
pub struct TextSketch {
    layout: Layout,
    font: Font,
    color: Color,
    pos: Vector2<u32>,
}

impl TextSketch {
    /// Create a new empty [TextSketch] with font data.
    #[inline]
    pub fn new(pos: Vector2<u32>, font: &[u8]) -> Self {
        Self {
            layout: Layout::new(CoordinateSystem::PositiveYDown),
            color: Color::BLACK,
            font: Font::from_bytes(font, Default::default()).expect("Failed to load font"),
            pos,
        }
    }

    /// Set the color of the text.
    #[inline]
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Append new text to the layout with `text` as text and `px` as size in pixels.
    #[inline]
    pub fn with_text(mut self, text: &str, px: f32) -> Self {
        self.layout
            .append(&[self.font.clone()], &TextStyle::new(text, px, 0));
        self
    }
}

impl Drawable for TextSketch {
    #[inline]
    fn apply(&self, canvas: &mut Canvas) {
        self.layout.glyphs().iter().for_each(|glyph| {
            let (metrics, buffer) = self.font.rasterize_config(glyph.key);

            let width = metrics.width;
            let height = metrics.height;

            // Calculate the starting position to draw the glyph
            let x0 = self.pos.x + glyph.x as u32;
            let y0 = self.pos.y + glyph.y as u32;

            // Draw the glyph to the canvas using the specified color
            for y in 0..height {
                for x in 0..width {
                    let alpha = buffer[y * width + x] as u8;
                    if alpha > 0 {
                        let x_coord = x0 + x as u32;
                        let y_coord = y0 + y as u32;
                        canvas.set_pixel(x_coord as usize, y_coord as usize, &self.color);
                    }
                }
            }
        });
    }
}
