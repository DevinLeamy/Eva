use encase::{ArrayLength, ShaderType};
use nalgebra::Vector3;

use super::ShaderStruct;

#[derive(ShaderType, Debug)]
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

#[derive(ShaderType, Debug)]
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

impl ShaderPointLights {
    pub fn new() -> Self {
        Self {
            length: ArrayLength,
            lights: Vec::new(),
        }
    }

    pub fn add(&mut self, light: ShaderPointLight) {
        self.lights.push(light);
    }
}
