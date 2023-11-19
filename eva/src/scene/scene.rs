use std::path::PathBuf;

use nalgebra::Vector3;

use crate::{shader::ShaderTextures, texture_loader::TextureLoader};

use super::{Node, Transformation};

#[derive(Clone)]
pub struct Scene {
    pub ambient: Vector3<f32>,
    pub root: Node,
    pub textures: ShaderTextures,
}

impl Scene {
    pub fn root(&self) -> &Node {
        &self.root
    }

    pub fn root_mut(&mut self) -> &mut Node {
        &mut self.root
    }

    pub fn ambient(&self) -> Vector3<f32> {
        self.ambient
    }

    pub fn set_ambient(&mut self, ambient: Vector3<f32>) {
        self.ambient = ambient;
    }

    pub fn textures(&self) -> &ShaderTextures {
        &self.textures
    }
}

impl Scene {}
