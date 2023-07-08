use crate::render::color::Color;
use crate::render::sketch::Sketch;

#[derive(Clone)]
pub struct Canvas {
    width: usize,
    height: usize,
    buffer: Vec<u32>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let buffer = vec![0; width * height];
        Self {
            width,
            height,
            buffer,
        }
    }

    pub fn buffer(self) -> Vec<u32> {
        self.buffer
    }

    pub fn resize(&mut self, new_width: usize, new_height: usize) {
        let new_size = new_width * new_height;
        let mut new_buffer = vec![0; new_size];
        let copy_width = usize::min(self.width, new_width);
        let copy_height = usize::min(self.height, new_height);

        for y in 0..copy_height {
            let source_offset = y * self.width;
            let target_offset = y * new_width;
            new_buffer[target_offset..target_offset + copy_width]
                .copy_from_slice(&self.buffer[source_offset..source_offset + copy_width]);
        }

        self.width = new_width;
        self.height = new_height;
        self.buffer = new_buffer;
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: &Color) {
        if let Some(px) = self.buffer.get_mut(y * self.width + x) {
            *px = color.0;
        }
    }

    pub fn clear(&mut self, color: &Color) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.set_pixel(x, y, color);
            }
        }
    }

    pub fn draw(&mut self, sketch: &Sketch) {
        for op in &sketch.0 {
            op.apply(self);
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}
