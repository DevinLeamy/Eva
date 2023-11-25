use encase::ShaderType;
use eva_macros::ShaderStructMacro;
use nalgebra::Vector3;

const TEXTURE_MARKER: f32 = -1.0;

#[derive(Clone, Debug, ShaderType, ShaderStructMacro)]
pub struct PbrMaterial {
    /// Varies [0-1].
    pub roughness: f32,
    /// Varies [0-1].
    pub metallic: f32,
    /// Varies [0-1] for rgb channels.
    pub albedo: Vector3<f32>,
}

impl PbrMaterial {
    pub fn set_texture(&mut self, id: u32) {
        self.albedo[0] = TEXTURE_MARKER;
        self.albedo[1] = id as f32;
    }
}

impl Default for PbrMaterial {
    fn default() -> Self {
        Self {
            albedo: Vector3::new(1.0, 1.0, 1.0),
            roughness: 0.0,
            metallic: 0.0,
        }
    }
}
