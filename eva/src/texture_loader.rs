use crate::prelude::*;
use std::collections::HashMap;

#[rustfmt::skip]
const FILLER_TEXTURE_PATH: &'static str = "/Users/Devin/Desktop/Github/DevinLeamy/eva/eva/assets/textures/filler.png";

#[derive(Clone)]
pub struct TextureLoader {
    texture_ids: HashMap<PathBuf, u32>,
    textures: ShaderTextures,
}

impl TextureLoader {
    pub fn new() -> Self {
        Self {
            texture_ids: HashMap::new(),
            textures: ShaderTextures::new(),
        }
    }
}

impl TextureLoader {
    pub fn load(&mut self, path: PathBuf) -> u32 {
        if let Some(id) = self.texture_ids.get(&path) {
            return *id;
        }
        let texture = ShaderTexture::from_path(&path).unwrap();
        let id = self.textures.add_texture(texture);
        self.texture_ids.insert(path, id);

        id
    }

    pub fn textures(mut self) -> ShaderTextures {
        while (self.textures.textures().len() as u32) < TEXTURE_2D_COUNT {
            self.textures
                .add_texture(ShaderTexture::from_path(FILLER_TEXTURE_PATH).unwrap());
        }

        self.textures
    }
}
