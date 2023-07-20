use crate::render::canvas::Canvas;
use crate::render::color::Color;
use crate::types::Image;
use mint::Vector2;

#[derive(Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq)]
pub struct Sketch(pub(crate) Vec<Operation>);

impl Sketch {
    /// Create a new empty sketch
    #[inline]
    pub fn new() -> Self { Self(Vec::new()) }

    /// Draw a line from `from` to `to` with color `color`
    #[inline]
    pub fn line(&mut self, from: Vector2<u32>, to: Vector2<u32>, color: Color) -> &mut Sketch {
        self.0.push(Operation::Line { from, to, color });
        self
    }

    /// Draw a circle at `pos` with radius `radius` and color `color`.
    #[inline]
    pub fn circle(&mut self, pos: Vector2<u32>, radius: u32, color: Color) -> &mut Sketch {
        self.0.push(Operation::Circle { pos, radius, color });
        self
    }

    /// Draw a rectangle at `pos` with width `width` and height `height` and color `color`.
    #[inline]
    pub fn rect(
        &mut self,
        pos: Vector2<u32>,
        width: u32,
        height: u32,
        color: Color,
    ) -> &mut Sketch {
        self.0.push(Operation::Rect {
            pos,
            width,
            height,
            color,
        });
        self
    }

    /// Draw an image at `pos` with width `width` and height `height` and data `data`.
    #[inline]
    pub fn image(
        &mut self,
        pos: Vector2<u32>,
        width: u32,
        height: u32,
        data: Vec<Color>,
    ) -> &mut Sketch {
        self.0.push(Operation::Image {
            pos,
            width,
            height,
            data,
        });
        self
    }

    /// Draw an oval at `pos` with width `width` and height `height` and color `color`.
    #[inline]
    pub fn oval(
        &mut self,
        pos: Vector2<u32>,
        width: u32,
        height: u32,
        color: Color,
    ) -> &mut Sketch {
        self.0.push(Operation::Oval {
            pos,
            width,
            height,
            color,
        });
        self
    }

    /// Just returns [Sketch] as &mut.\
    /// Only existent for example and testing purposes.
    #[inline]
    pub fn empty(&mut self) -> &mut Sketch { self }
}

/// A drawing operation to apply to a [Canvas] using a [Sketch].
#[derive(Clone, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub enum Operation {
    Line {
        from: Vector2<u32>,
        to: Vector2<u32>,
        color: Color,
    },

    Circle {
        pos: Vector2<u32>,
        radius: u32,
        color: Color,
    },

    Rect {
        pos: Vector2<u32>,
        width: u32,
        height: u32,
        color: Color,
    },

    Image {
        pos: Vector2<u32>,
        width: u32,
        height: u32,
        data: Image,
    },

    Oval {
        pos: Vector2<u32>,
        width: u32,
        height: u32,
        color: Color,
    },
}

impl Operation {
    /// Apply operation to a [Canvas].
    #[inline(never)]
    pub fn apply(&self, canvas: &mut Canvas) {
        match self {
            Operation::Oval {
                pos,
                width,
                height,
                color,
            } => {
                let start_x = pos.x;
                let start_y = pos.y;
                let end_x = start_x + *width;
                let end_y = start_y + *height;

                let a = *width as f32 / 2.0;
                let b = *height as f32 / 2.0;
                let cx = start_x as f32 + a;
                let cy = start_y as f32 + b;

                for y in start_y..end_y {
                    for x in start_x..end_x {
                        let px = x as f32 + 0.5;
                        let py = y as f32 + 0.5;
                        let dx = px - cx;
                        let dy = py - cy;

                        let distance_squared = (dx * dx / (a * a) + dy * dy / (b * b)).abs();

                        if distance_squared <= 1.0 {
                            canvas.set_pixel(x as usize, y as usize, color);
                        }
                    }
                }
            }

            Operation::Line { to, from, color } => {
                let dx = to.x as isize - from.x as isize;
                let dy = to.y as isize - from.y as isize;
                let steps = usize::max(dx.abs() as usize, dy.abs() as usize);

                if steps == 0 {
                    canvas.set_pixel(from.x as usize, from.y as usize, color);
                    return;
                }

                let x_increment = dx as f32 / steps as f32;
                let y_increment = dy as f32 / steps as f32;

                let mut x = from.x as f32;
                let mut y = from.y as f32;

                for _ in 0..=steps {
                    canvas.set_pixel(x as usize, y as usize, color);
                    x += x_increment;
                    y += y_increment;
                }
            }

            Operation::Circle { pos, radius, color } => {
                let radius_squared = (*radius as f32).powi(2);

                let (cx, cy) = (pos.x as isize, pos.y as isize);
                let (start_x, start_y) = (cx - *radius as isize, cy - *radius as isize);
                let (end_x, end_y) = (cx + *radius as isize, cy + *radius as isize);

                for y in start_y..=end_y {
                    for x in start_x..=end_x {
                        let dx = x - cx;
                        let dy = y - cy;
                        let distance_squared = (dx * dx + dy * dy) as f32;

                        if distance_squared <= radius_squared {
                            canvas.set_pixel(x as usize, y as usize, color);
                        }
                    }
                }
            }

            Operation::Rect {
                pos,
                width,
                height,
                color,
            } => {
                let start_x = pos.x as usize;
                let start_y = pos.y as usize;
                let end_x = start_x + *width as usize;
                let end_y = start_y + *height as usize;

                for y in start_y..end_y {
                    for x in start_x..end_x {
                        canvas.set_pixel(x, y, color);
                    }
                }
            }

            Operation::Image {
                pos,
                width,
                height,
                data,
            } => {
                let start_x = pos.x as usize;
                let start_y = pos.y as usize;
                let end_x = start_x + *width as usize;
                let end_y = start_y + *height as usize;

                let mut index = 0;

                for y in start_y..end_y {
                    for x in start_x..end_x {
                        canvas.set_pixel(x, y, &data[index]);
                        index += 1;
                    }
                }
            }
        }
    }
}
