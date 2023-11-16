use nalgebra::{SimdBool, Unit, Vector3};

use super::Collidable;

#[derive(Clone, Debug)]
pub struct Cube {
    min: Vector3<f32>,
    max: Vector3<f32>,
}

impl Cube {
    /// Create a cube with a given side length.
    pub fn new(size: f32) -> Self {
        Self {
            min: Vector3::zeros(),
            max: Vector3::new(size, size, size),
        }
    }

    pub fn from_points(min: Vector3<f32>, max: Vector3<f32>) -> Self {
        Self { min, max }
    }
}

impl Cube {
    fn is_inside(&self, point: &Vector3<f32>) -> bool {
        point.ge(&self.min).all() && point.le(&self.max).all()
    }
}

impl Collidable for Cube {
    fn foo(&self) {}
}
