use super::Collidable;

#[derive(Clone, Debug)]
pub struct Sphere {
    radius: f32,
}

impl Sphere {
    /// Create a sphere with a given radius.
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }
}

impl Collidable for Sphere {
    fn foo(&self) {}
}
