use image::{DynamicImage, Pixel, Rgb};
use image::buffer::Pixels;
use crate::render::color::Color;
use crate::types::Image;

pub fn dynamic_to_img(dynamic: DynamicImage) -> Image {
    rgb_to_img(dynamic.to_rgb8().pixels())
}

pub fn rgb_to_img(rgb: Pixels<Rgb<u8>>) -> Image {
    let mut vec: Image = Vec::with_capacity(rgb.len());
    for px in rgb {
        let rgb = px.0;
        vec.push(Color::from_rgb(rgb[0], rgb[1], rgb[2]));
    }
    vec
}