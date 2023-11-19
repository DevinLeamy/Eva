use super::ShaderTexture;

#[derive(Debug, Clone)]
pub struct ShaderTextures {
    textures: Vec<ShaderTexture>,
}

impl ShaderTextures {
    pub fn new() -> Self {
        Self { textures: vec![] }
    }
}

impl ShaderTextures {
    pub fn add_texture(&mut self, texture: ShaderTexture) -> u32 {
        self.textures.push(texture);

        (self.textures.len() - 1) as u32
    }

    pub fn textures(&self) -> &Vec<ShaderTexture> {
        &self.textures
    }
}
