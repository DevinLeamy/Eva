use encase::ShaderType;

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
        todo!()
    }
}
