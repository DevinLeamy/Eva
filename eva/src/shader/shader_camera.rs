use crate::{prelude::Camera, shader::ShaderStruct};
use encase::ShaderType;
use nalgebra::{Matrix4, Vector3};

#[derive(Debug, ShaderType)]
pub struct ShaderCamera {
    camera_to_world: Matrix4<f32>,
    position: Vector3<f32>,
    fov: f32,
}

impl ShaderStruct for ShaderCamera {
    fn as_bytes(&self) -> Option<Vec<u8>> {
        let mut buffer = encase::UniformBuffer::new(Vec::new());
        buffer.write(self).ok()?;
        Some(buffer.into_inner())
    }
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
