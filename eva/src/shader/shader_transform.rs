use encase::ShaderType;
use nalgebra::{Matrix3, Matrix4};

#[derive(ShaderType)]
pub struct ShaderTransform {
    m: Matrix4<f32>,
    m_inverse: Matrix4<f32>,
    m_normal_inverse: Matrix3<f32>,
}

