use encase::ShaderType;
use eva_macros::ShaderStructMacro;
use nalgebra::Vector3;

#[derive(ShaderType, ShaderStructMacro)]
pub struct ShaderGlobalConfig {
    pub sample_count: u32,
    pub max_reflections: u32,
}
