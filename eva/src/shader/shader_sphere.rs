use encase::ShaderType;

use crate::prelude::Sphere;

use super::{ShaderStruct, ShaderTransform};

#[derive(ShaderType)]
pub struct ShaderSphereModel {
    pub sphere: Sphere,
    pub transform: ShaderTransform,
}

impl ShaderStruct for ShaderSphereModel {
    fn as_bytes(&self) -> Option<Vec<u8>> {
        let mut buffer = encase::UniformBuffer::new(Vec::new());
        buffer.write(self).ok()?;

        Some(buffer.into_inner())
    }

    
}
