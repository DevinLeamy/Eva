use encase::ShaderType;
use eva_macros::ShaderStructMacro;
use nalgebra::Vector3;

#[derive(ShaderType, ShaderStructMacro)]
pub struct ShaderGlobalConfig {
    pub ambient: Vector3<f32>,
}
