use encase::ShaderType;
use eva_macros::ShaderStructMacro;
use nalgebra::Vector3;

#[derive(ShaderType, Debug, ShaderStructMacro)]
pub struct ShaderPointLight {
    pub position: Vector3<f32>,
    pub colour: Vector3<f32>,
}
