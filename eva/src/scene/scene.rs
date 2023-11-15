use nalgebra::Vector3;

use super::{Node, Transformation};

#[derive(Clone)]
pub struct Scene {
    ambient: Vector3<f32>,
    root: Node,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            ambient: Vector3::new(0.1, 0.1, 0.1),
            root: Transformation::new().into(),
        }
    }
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
