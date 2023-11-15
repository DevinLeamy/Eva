use encase::ShaderType;

use crate::prelude::Sphere;

use super::ShaderTransform;

#[derive(ShaderType)]
pub struct ShaderSphereModel {
    sphere: Sphere,
    transform: ShaderTransform,
}
