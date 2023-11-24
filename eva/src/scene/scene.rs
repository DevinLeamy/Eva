use crate::prelude::*;
use nalgebra::Vector3;

#[derive(Clone)]
pub struct Scene {
    pub ambient: Vector3<f32>,
    pub root: Node,
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
}

impl Scene {}
