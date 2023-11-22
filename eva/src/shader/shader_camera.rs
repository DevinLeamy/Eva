use crate::{prelude::Camera, shader::ShaderStruct};
use encase::ShaderType;
use eva_macros::ShaderStructMacro;
use nalgebra::{Matrix4, Vector3};

#[derive(Debug, ShaderType, ShaderStructMacro)]
pub struct ShaderCamera {
    camera_to_world: Matrix4<f32>,
    position: Vector3<f32>,
    fov: f32,
}

impl Into<ShaderCamera> for Camera {
    fn into(self) -> ShaderCamera {
        ShaderCamera {
            camera_to_world: self.camera_to_world_mat(),
            position: self.origin().into(),
            fov: self.fov(),
        }
    }
}
