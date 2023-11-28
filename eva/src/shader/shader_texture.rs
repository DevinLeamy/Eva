use crate::{asset_loader::AssetLoader, prelude::*};
use image::DynamicImage;

#[derive(Debug, Clone)]
pub struct ShaderTexture {
    image: DynamicImage,
}

impl ShaderTexture {
    pub fn from_path<P: Into<PathBuf>>(path: P) -> Option<Self> {
        let image = AssetLoader::load_texture_image(&path.into());

        Some(Self { image })
    }
}

impl ShaderTexture {
    pub fn as_bytes(&self) -> Vec<f32> {
        self.image.clone().into_rgba32f().into_raw()
    }

    pub fn width(&self) -> u32 {
        self.image.width()
    }

    pub fn height(&self) -> u32 {
        self.image.height()
    }
}

impl<'a> Into<TextureDescriptor<'a>> for ShaderTexture {
    fn into(self) -> TextureDescriptor<'a> {
        TextureDescriptor {
            label: None,
            size: Extent3d {
                width: self.image.width(),
                height: self.image.height(),
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba32Float,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[],
        }
    }
}
