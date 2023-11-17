use encase::{ArrayLength, ShaderType};

use crate::prelude::{Cube, PhongMaterial};

use super::{ShaderStruct, ShaderTransform};

#[derive(ShaderType, Debug)]
pub struct ShaderCubeModel {
    pub cube: Cube,
    pub transform: ShaderTransform,
    pub material: PhongMaterial,
}

impl ShaderStruct for ShaderCubeModel {
    fn as_bytes(&self) -> Option<Vec<u8>> {
        let mut buffer = encase::UniformBuffer::new(Vec::new());
        buffer.write(self).ok()?;

        Some(buffer.into_inner())
    }
}

#[derive(ShaderType, Debug)]
pub struct ShaderCubeModels {
    pub length: ArrayLength,
    #[size(runtime)]
    pub cubes: Vec<ShaderCubeModel>,
}

impl ShaderStruct for ShaderCubeModels {
    fn as_bytes(&self) -> Option<Vec<u8>> {
        let mut buffer = encase::StorageBuffer::new(Vec::new());
        buffer.write(self).ok()?;
        Some(buffer.into_inner())
    }
}

impl ShaderCubeModels {
    pub fn new() -> Self {
        Self {
            length: ArrayLength,
            cubes: Vec::new(),
        }
    }

    pub fn add(&mut self, light: ShaderCubeModel) {
        self.cubes.push(light);
    }
}
