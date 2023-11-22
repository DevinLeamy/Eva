use encase::ShaderType;
use nalgebra::Vector3;

#[derive(Clone, Debug, ShaderType)]
pub struct Cube {
    min: Vector3<f32>,
    max: Vector3<f32>,
}

impl Default for Cube {
    fn default() -> Self {
        Self::new(1.0)
    }
}

impl Cube {
    /// Create a cube with a given side length.
    pub fn new(size: f32) -> Self {
        Self {
            min: Vector3::new(-size / 2.0, -size / 2.0, -size / 2.0),
            max: Vector3::new(size / 2.0, size / 2.0, size / 2.0),
        }
    }

    pub fn from_points(min: Vector3<f32>, max: Vector3<f32>) -> Self {
        Self { min, max }
    }
}
