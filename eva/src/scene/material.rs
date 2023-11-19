use encase::ShaderType;
use nalgebra::Vector3;

const MISSING_TEXTURE_ID: u32 = 0;

#[derive(Clone, Debug, ShaderType)]
pub struct PhongMaterial {
    pub diffuse: Vector3<f32>,
    pub specular: Vector3<f32>,
    pub shininess: f32,
    pub texture_id: u32,
}

impl PhongMaterial {
    pub fn new(diffuse: Vector3<f32>, specular: Vector3<f32>, shininess: f32) -> Self {
        Self {
            diffuse,
            specular,
            shininess,
            texture_id: MISSING_TEXTURE_ID,
        }
    }

    pub fn diffuse(&self) -> &Vector3<f32> {
        &self.diffuse
    }

    pub fn specular(&self) -> &Vector3<f32> {
        &self.specular
    }

    pub fn shininess(&self) -> f32 {
        self.shininess
    }

    pub fn set_texture(&mut self, id: u32) {
        self.texture_id = id;
    }
}

impl Default for PhongMaterial {
    fn default() -> Self {
        Self {
            diffuse: Vector3::zeros(),
            specular: Vector3::zeros(),
            shininess: 0.0,
            texture_id: MISSING_TEXTURE_ID,
        }
    }
}
