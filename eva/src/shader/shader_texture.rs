use crate::prelude::*;
use image::{io::Reader, DynamicImage};

#[derive(Debug, Clone)]
pub struct ShaderTexture {
    image: DynamicImage,
}

impl ShaderTexture {
    pub fn from_path<P: Into<PathBuf>>(path: P) -> Option<Self> {
        let path = path.into();
        println!("PATH: {:?}", path);
        let image = Reader::open(path).ok()?.decode().ok()?;

        Some(Self { image })
    }
}

impl ShaderTexture {
    pub fn as_bytes(&self) -> Vec<u16> {
        self.image.clone().into_rgba16().into_raw()
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
            format: TextureFormat::Rgba16Float,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[],
        }
    }
}
