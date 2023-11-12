use std::path::PathBuf;

use image::RgbImage;
use nalgebra::Vector3;

use super::Buffer;

/// PNG image. Each pixel in the image is an rgb vector
/// in the range [0, 1]. Value are indexed (x, y) where
/// x goes left to right and y goes top to bottom.
#[derive(Clone)]
pub struct PngImage {
    width: u32,
    height: u32,
    buffer: RgbImage,
}

impl PngImage {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            buffer: RgbImage::new(width, height),
        }
    }
}

impl PngImage {
    /// Save the image to the given path.
    pub fn save(&self, path: impl Into<PathBuf>) {
        self.buffer.save(path.into()).unwrap()
    }
}

impl Buffer for PngImage {
    type Value = Vector3<f32>;

    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }

    /// Sets the value of a pixel.
    ///
    /// Pixel values outside of [0, 1] are clamped.
    fn set(&mut self, x: u32, y: u32, value: Self::Value) {
        let rgb = image::Rgb([
            (value.x.clamp(0.0, 1.0) * 255.0) as u8,
            (value.y.clamp(0.0, 1.0) * 255.0) as u8,
            (value.z.clamp(0.0, 1.0) * 255.0) as u8,
        ]);
        *self.buffer.get_pixel_mut(x, y) = rgb;
    }

    fn get(&self, x: u32, y: u32) -> Self::Value {
        let rgb = self.buffer.get_pixel(x, y).0;
        Self::Value::new(
            rgb[0] as f32 / 255.0,
            rgb[1] as f32 / 255.0,
            rgb[2] as f32 / 255.0,
        )
    }
}

#[test]
fn test() {
    // Completely unnecessary test but hey - who doesn't like rainbow squares!
    let mut image = PngImage::new(500, 500);

    for i in 0..500_u32 {
        for j in 0..500_u32 {
            image.set(i, j, Vector3::new(i as f32 / 500.0, j as f32 / 500.0, 0.5));
        }
    }

    image.save("./assets/image.png");
}
