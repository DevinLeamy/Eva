use encase::ShaderType;

#[derive(Clone, Debug, ShaderType)]
pub struct Sphere {
    pub radius: f32,
}

impl Sphere {
    /// Create a sphere with a given radius.
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }
}
