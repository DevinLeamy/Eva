use encase::ShaderType;

use crate::prelude::{Cube, PhongMaterial};

use super::ShaderTransform;

#[derive(ShaderType, Debug)]
pub struct ShaderCubeModel {
    pub cube: Cube,
    pub transform: ShaderTransform,
    pub material: PhongMaterial,
}
