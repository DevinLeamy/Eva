use image::DynamicImage;

pub fn create_aligned_image_bytes(image: &DynamicImage, alignment: u32) -> Vec<u8> {
    let image = image.to_rgba32f();
    let (width, height) = image.dimensions();
    let row_size = width * 16;
    let padded_row_size = align(row_size, alignment);
    let bytes = bytemuck::cast_slice(&image);

    let mut padded_image_data = vec![0u8; (padded_row_size * height) as usize];

    for (i, row) in padded_image_data
        .chunks_exact_mut(padded_row_size as usize)
        .enumerate()
    {
        let start = i * row_size as usize;
        let end = start + row_size as usize;
        row[..row_size as usize].copy_from_slice(&bytes[start..end]);
    }

    padded_image_data
}

pub fn align(n: u32, alignment: u32) -> u32 {
    if n % alignment == 0 {
        n
    } else {
        n + alignment - n % alignment
    }
}
