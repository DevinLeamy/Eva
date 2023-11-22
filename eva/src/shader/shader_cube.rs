use encase::ShaderType;

use crate::prelude::{Cube, PhongMaterial};

use super::{ShaderTransform, ShaderStruct};

#[derive(ShaderType, Debug)]
pub struct ShaderCubeModel {
    pub cube: Cube,
    pub transform: ShaderTransform,
    pub material: PhongMaterial,
}

impl ShaderStruct for ShaderCubeModel {
    fn as_bytes(&self) -> Option<Vec<u8>> {
        todo!()
    }
}
