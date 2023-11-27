use crate::prelude::*;
use std::collections::HashMap;

const FILLER_TEXTURE: &'static str = "filler.png";

#[derive(Clone)]
pub struct TextureLoader {
    texture_ids: HashMap<String, u32>,
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
    pub async fn load(&mut self, path: String) -> u32 {
        if let Some(id) = self.texture_ids.get(&path) {
            return *id;
        }

        let texture = ShaderTexture::from_path(path.clone()).await.unwrap();
        let id = self.textures.add_texture(texture);
        self.texture_ids.insert(path, id);

        id
    }

    pub async fn textures(mut self) -> ShaderTextures {
        while (self.textures.textures().len() as u32) < TEXTURE_2D_COUNT {
            self.textures.add_texture(
                ShaderTexture::from_path(FILLER_TEXTURE.to_string())
                    .await
                    .unwrap(),
            );
        }

        self.textures
    }
}
