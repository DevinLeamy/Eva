use nalgebra::Vector3;

use crate::{
    prelude::{PbrMaterial, ShaderBuffer},
    shader::{ShaderSkybox, ShaderTextures},
};

pub struct StaticRenderContext {
    pub textures: ShaderTextures,
    pub skybox: ShaderSkybox,
    pub materials: ShaderBuffer<PbrMaterial>,
    pub sample_count: u32,
    pub max_reflections: u32,
}
