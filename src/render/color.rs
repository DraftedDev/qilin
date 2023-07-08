#[derive(Clone, Copy)]
pub struct Color(pub u32);

impl Color {

    pub const RED: Color = Color::from_rgb(255, 0, 0);
    pub const GREEN: Color = Color::from_rgb(0, 255, 0);
    pub const BLUE: Color = Color::from_rgb(0, 0, 255);
    pub const YELLOW: Color = Color::from_rgb(255, 255, 0);
    pub const CYAN: Color = Color::from_rgb(0, 255, 255);
    pub const MAGENTA: Color = Color::from_rgb(255, 0, 255);
    pub const BLACK: Color = Color::from_rgb(0, 0, 0);
    pub const WHITE: Color = Color::from_rgb(255, 255, 255);
    pub const GRAY: Color = Color::from_rgb(128, 128, 128);
    pub const PURPLE: Color = Color::from_rgb(128, 0, 128);
    pub const AQUA: Color = Color::from_rgb(0, 255, 255);
    pub const ORANGE: Color = Color::from_rgb(255, 165, 0);
    pub const PINK: Color = Color::from_rgb(238, 130, 238);
    pub const TURQUOISE: Color = Color::from_rgb(64, 224, 208);

    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Color {
        let (r, g, b) = (r as u32, g as u32, b as u32);
        Color((r << 16) | (g << 8) | b)
    }

    pub fn to_rgb(self) -> (u8, u8, u8) {

        let r = ((self.0 >> 16) & 0xFF) as u8;
        let g = ((self.0 >> 8) & 0xFF) as u8;
        let b = (self.0 & 0xFF) as u8;

        (r, g, b)
    }

}