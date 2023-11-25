use encase::ShaderType;
use eva_macros::ShaderStructMacro;

use crate::prelude::Sphere;

use super::{ShaderTransform};

#[derive(ShaderType, Debug, ShaderStructMacro)]
pub struct ShaderSphereModel {
    pub sphere: Sphere,
    pub transform: ShaderTransform,
    pub material_id: u32,
}
