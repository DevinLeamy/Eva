use encase::ShaderType;
use eva_macros::ShaderStructMacro;

use crate::prelude::{Cube, PhongMaterial};

use super::{ShaderStruct, ShaderTransform};

#[derive(ShaderType, Debug, ShaderStructMacro)]
pub struct ShaderCubeModel {
    pub cube: Cube,
    pub transform: ShaderTransform,
    pub material: PhongMaterial,
}
