use encase::{ArrayLength, ShaderType};
use nalgebra::Vector3;

use super::ShaderStruct;

#[derive(ShaderType)]
pub struct ShaderPointLight {
    pub position: Vector3<f32>,
    pub colour: Vector3<f32>,
}

impl ShaderStruct for ShaderPointLight {
    fn as_bytes(&self) -> Option<Vec<u8>> {
        let mut buffer = encase::UniformBuffer::new(Vec::new());
        buffer.write(self).ok()?;

        Some(buffer.into_inner())
    }
}

#[derive(ShaderType)]
pub struct ShaderPointLights {
    pub length: ArrayLength,
    #[size(runtime)]
    pub lights: Vec<ShaderPointLight>,
}

impl ShaderStruct for ShaderPointLights {
    fn as_bytes(&self) -> Option<Vec<u8>> {
        let mut buffer = encase::StorageBuffer::new(Vec::new());
        buffer.write(self).ok()?;
        Some(buffer.into_inner())
    }
}
