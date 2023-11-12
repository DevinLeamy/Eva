use std::path::PathBuf;

use image::{Rgb, RgbImage};
use nalgebra::Vector3;

pub struct PatternedBackground {
    width: u32,
    height: u32,
    buffer: RgbImage,
    row_shift: u32,
    pattern_width: u32,
    pattern_height: u32,
}

impl PatternedBackground {
    pub fn new<P: Into<PathBuf>>(png_path: P, width: u32, height: u32, pattern_width: u32) -> Self {
        let buffer = image::open(png_path.into()).unwrap().to_rgb8();
        let aspect = buffer.height() as f32 / buffer.width() as f32;
        Self {
            pattern_height: (pattern_width as f32 * aspect) as u32,
            width,
            height,
            buffer,
            row_shift: 0,
            pattern_width,
        }
    }

    pub fn color(&self, x: u32, y: u32) -> Vector3<f32> {
        let normalized_x = x as f32 / self.width as f32;
        let normalized_y = 1.0 - y as f32 / self.height as f32;

        let pixel = self.image_pixel(x, y);
        if pixel[0] != 0 && pixel[0] != 255 {
            return Vector3::new(0.3 * normalized_x + 0.4, 0.4, 0.4) * normalized_y;
        }
        Vector3::new(
            pixel[0] as f32 / 255.0,
            pixel[1] as f32 / 255.0,
            pixel[2] as f32 / 255.0,
        )
    }

    fn image_pixel(&self, x: u32, y: u32) -> &Rgb<u8> {
        let row = y / self.pattern_height;

        let pattern_x = x % self.pattern_width;
        let pattern_y = (y + row * self.row_shift) % self.pattern_height;

        let norm_pattern_x = pattern_x as f32 / self.pattern_width as f32;
        let norm_pattern_y = pattern_y as f32 / self.pattern_height as f32;

        let image_x = (self.buffer.width() as f32 * norm_pattern_x) as u32;
        let image_y = (self.buffer.height() as f32 * norm_pattern_y) as u32;

        self.buffer.get_pixel(image_x, image_y)
    }
}
