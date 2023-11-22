use encase::ShaderType;
use nalgebra::Vector3;

#[derive(ShaderType, Debug)]
pub struct ShaderPointLight {
    pub position: Vector3<f32>,
    pub colour: Vector3<f32>,
}
