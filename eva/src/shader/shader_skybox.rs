use crate::prelude::*;
use image::{io::Reader, DynamicImage};
use wgpu::util::DeviceExt;

pub trait SkyboxExtension: DeviceExt {
    fn create_skybox_view(&self, queue: &Queue, skybox: &ShaderSkybox) -> TextureView;
}

impl SkyboxExtension for Device {
    fn create_skybox_view(&self, queue: &Queue, skybox: &ShaderSkybox) -> TextureView {
        let skybox_texture = self.create_texture(&skybox.clone().into());
        let mut encoder = self.create_command_encoder(&CommandEncoderDescriptor::default());

        for i in 0..6 {
            let image = &skybox.images[i];
            assert!(image.width() == image.height());
            let size = image.width();

            let buffer = self.create_buffer_init(&util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&image.clone().into_rgba32f()),
                usage: BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
            });

            let bytes = 4 * 4 * size;
            encoder.copy_buffer_to_texture(
                ImageCopyBuffer {
                    buffer: &buffer,
                    layout: ImageDataLayout {
                        offset: 0,
                        // bytes_per_row: Some(bytes + 256 - (bytes % 256)),
                        bytes_per_row: Some(bytes),
                        rows_per_image: Some(size),
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
                    width: size,
                    height: size,
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
            .map(|path| Reader::open(path).unwrap().decode().unwrap())
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
