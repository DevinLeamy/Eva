use std::path::PathBuf;

use crate::prelude::align;

use half::f16;
use image::{DynamicImage, Rgba, RgbaImage};
use wgpu::*;

pub fn screenshot_rgba16f_buffer(
    device: &Device,
    buffer: &Buffer,
    path: PathBuf,
    width: u32,
    height: u32,
) {
    let buffer_slice = buffer.slice(..);
    buffer_slice.map_async(wgpu::MapMode::Read, |_| {});
    device.poll(wgpu::Maintain::Wait);
    let data = buffer_slice.get_mapped_range();
    let screenshot = rgbaf16_to_image(&data.to_vec(), width, height, align(width, 256));
    screenshot.save(path).unwrap();
}

fn gamma_correction(colour: f32) -> f32 {
    colour.powf(1.0 / 2.2)
}

fn rgbaf16_to_image(buffer: &[u8], width: u32, height: u32, aligned_width: u32) -> DynamicImage {
    let mut image_buffer = RgbaImage::new(width, height);

    for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
        let idx = ((y * aligned_width + x) * 8) as usize;

        let r = f16::from_ne_bytes([buffer[idx + 0], buffer[idx + 1]]).to_f32();
        let g = f16::from_ne_bytes([buffer[idx + 2], buffer[idx + 3]]).to_f32();
        let b = f16::from_ne_bytes([buffer[idx + 4], buffer[idx + 5]]).to_f32();
        let a = f16::from_ne_bytes([buffer[idx + 6], buffer[idx + 7]]).to_f32();

        *pixel = Rgba([
            (gamma_correction(r) * 255.0) as u8,
            (gamma_correction(g) * 255.0) as u8,
            (gamma_correction(b) * 255.0) as u8,
            (a * 255.0) as u8,
        ]);
    }

    DynamicImage::ImageRgba8(image_buffer)
}
