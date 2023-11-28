use crate::{asset_loader::AssetLoader, prelude::*};
use image::{io::Reader, DynamicImage};
use wgpu::util::DeviceExt;

pub trait SkyboxExtension: DeviceExt {
    fn create_skybox_view(&self, queue: &Queue, skybox: &ShaderSkybox) -> TextureView;
}

impl SkyboxExtension for Device {
    fn create_skybox_view(&self, queue: &Queue, skybox: &ShaderSkybox) -> TextureView {
        let skybox_texture = self.create_texture(&skybox.clone().into());
        let mut encoder = self.create_command_encoder(&CommandEncoderDescriptor::default());
        let image_size = skybox.images[0].width();
        let bytes_per_row = align(4 * 4 * image_size, 256);

        for i in 0..6 {
            // Note: Buffers needs to be created for every time because the write does not happen immediately,
            // meaning that on the next frame if a new buffer is used, the existing data gets overritten.
            let buffer = self.create_buffer(&BufferDescriptor {
                label: None,
                size: (bytes_per_row * image_size) as u64,
                usage: BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });

            let image = &skybox.images[i];
            queue.write_buffer(&buffer, 0, &create_aligned_image_bytes(image, 256));
            encoder.copy_buffer_to_texture(
                ImageCopyBuffer {
                    buffer: &buffer,
                    layout: ImageDataLayout {
                        offset: 0,
                        bytes_per_row: Some(bytes_per_row),
                        rows_per_image: Some(image_size),
                    },
                },
                ImageCopyTexture {
                    texture: &skybox_texture,
                    mip_level: 0,
                    origin: Origin3d {
                        x: 0,
                        y: 0,
                        z: i as u32,
                    },
                    aspect: TextureAspect::All,
                },
                Extent3d {
                    width: image_size,
                    height: image_size,
                    depth_or_array_layers: 1,
                },
            );
        }

        queue.submit(Some(encoder.finish()));

        skybox_texture.create_view(&TextureViewDescriptor {
            label: None,
            dimension: Some(TextureViewDimension::Cube),
            ..Default::default()
        })
    }
}

#[derive(Debug, Clone)]
pub struct ShaderSkybox {
    images: Vec<DynamicImage>,
}

impl ShaderSkybox {
    pub fn create_skybox(images: Vec<PathBuf>) -> Option<Self> {
        assert!(images.len() == 6);
        let images: Vec<DynamicImage> = images
            .iter()
            .map(|path| AssetLoader::load_skybox_image(path))
            .collect();

        Some(Self { images })
    }
}

impl ShaderSkybox {
    pub fn extents(&self) -> Extent3d {
        Extent3d {
            width: self.images[0].width(),
            height: self.images[0].height(),
            depth_or_array_layers: 6,
        }
    }
}

impl<'a> Into<TextureDescriptor<'a>> for ShaderSkybox {
    fn into(self) -> TextureDescriptor<'a> {
        TextureDescriptor {
            label: None,
            size: self.extents(),
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba32Float,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[],
        }
    }
}
