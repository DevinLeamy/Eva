use encase::ShaderType;
use nalgebra::Vector3;

use super::ShaderStruct;

#[derive(ShaderType, Debug)]
pub struct ShaderPointLight {
    pub position: Vector3<f32>,
    pub colour: Vector3<f32>,
}

impl ShaderStruct for ShaderPointLight {
    fn as_bytes(&self) -> Option<Vec<u8>> {
        todo!()
    }
}
