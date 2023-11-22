use encase::ShaderType;
use eva_macros::ShaderStructMacro;

use crate::prelude::{PhongMaterial, Sphere};

use super::{ShaderStruct, ShaderTransform};

#[derive(ShaderType, Debug, ShaderStructMacro)]
pub struct ShaderSphereModel {
    pub sphere: Sphere,
    pub transform: ShaderTransform,
    pub material: PhongMaterial,
}
