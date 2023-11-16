use encase::ShaderType;
use nalgebra::{Matrix3, Matrix4};

use crate::prelude::Transform;

#[derive(ShaderType, Debug)]
pub struct ShaderTransform {
    m: Matrix4<f32>,
    m_inverse: Matrix4<f32>,
    m_normal_inverse: Matrix3<f32>,
}

impl From<Transform> for ShaderTransform {
    fn from(transform: Transform) -> Self {
        Self {
            m: transform.as_mat4(),
            m_inverse: transform.as_mat4().try_inverse().unwrap(),
            m_normal_inverse: transform.as_mat3_inverse()
        }
    }
}
