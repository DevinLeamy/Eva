use encase::ShaderType;
use nalgebra::{Matrix4, Vector3, Vector4};
use wall_e::prelude::Camera;

#[derive(Debug, ShaderType)]
pub struct ShaderCamera {
    camera_to_world: Matrix4<f32>,
    position: Vector3<f32>,
    fov: f32,
}

impl ShaderCamera {
    pub fn as_wgsl_bytes(&self) -> encase::internal::Result<Vec<u8>> {
        let mut buffer = encase::UniformBuffer::new(Vec::new());
        buffer.write(self)?;
        Ok(buffer.into_inner())
    }
}

impl Into<ShaderCamera> for Camera {
    fn into(self) -> ShaderCamera {
        ShaderCamera {
            camera_to_world: self.camera_to_world_mat(),
            // position: self.origin().into(),
            position: [1.0, 0.0, 0.0].into(),
            fov: self.fov(),
        }
    }
}
