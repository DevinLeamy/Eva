use nalgebra::Vector3;

use crate::{
    prelude::{PbrMaterial, ShaderBuffer},
    shader::{ShaderSkybox, ShaderTextures},
};

pub struct StaticRenderContext {
    pub textures: ShaderTextures,
    pub skybox: ShaderSkybox,
    pub ambient: Vector3<f32>,
    pub materials: ShaderBuffer<PbrMaterial>,
}
