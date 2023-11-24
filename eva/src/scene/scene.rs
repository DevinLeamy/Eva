use crate::prelude::*;
use nalgebra::Vector3;

#[derive(Clone)]
pub struct Scene {
    pub root: Node,
}

impl Scene {
    pub fn root(&self) -> &Node {
        &self.root
    }

    pub fn root_mut(&mut self) -> &mut Node {
        &mut self.root
    }
}

impl Scene {}
