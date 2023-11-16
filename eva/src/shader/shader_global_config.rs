use encase::ShaderType;
use nalgebra::Vector3;

use super::ShaderStruct;

#[derive(ShaderType)]
pub struct ShaderGlobalConfig {
    pub ambient: Vector3<f32>,
}

impl ShaderStruct for ShaderGlobalConfig {
    fn as_bytes(&self) -> Option<Vec<u8>> {
        let mut buffer = encase::UniformBuffer::new(Vec::new());
        buffer.write(self).ok()?;

        Some(buffer.into_inner())
    }
}
