use encase::{ArrayLength, ShaderType};

use crate::prelude::{PhongMaterial, Sphere};

use super::{ShaderStruct, ShaderTransform};

#[derive(ShaderType, Debug)]
pub struct ShaderSphereModel {
    pub sphere: Sphere,
    pub transform: ShaderTransform,
    pub material: PhongMaterial,
}

impl ShaderStruct for ShaderSphereModel {
    fn as_bytes(&self) -> Option<Vec<u8>> {
        let mut buffer = encase::UniformBuffer::new(Vec::new());
        buffer.write(self).ok()?;

        Some(buffer.into_inner())
    }
}

#[derive(ShaderType, Debug)]
pub struct ShaderSphereModels {
    pub length: ArrayLength,
    #[size(runtime)]
    pub spheres: Vec<ShaderSphereModel>,
}

impl ShaderStruct for ShaderSphereModels {
    fn as_bytes(&self) -> Option<Vec<u8>> {
        let mut buffer = encase::StorageBuffer::new(Vec::new());
        buffer.write(self).ok()?;
        Some(buffer.into_inner())
    }
}

impl ShaderSphereModels {
    pub fn new() -> Self {
        Self {
            length: ArrayLength,
            spheres: Vec::new()
        }
    }

    pub fn add(&mut self, light: ShaderSphereModel) {
        self.spheres.push(light);
    }
}
