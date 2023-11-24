pub use crate::prelude::*;
use eva::prelude::Transform;

#[pyclass]
#[pyo3(name = "EvaLight")]
#[derive(PyNode)]
pub struct EvaLight {
    pub inner: Light,
}

#[pymethods]
impl EvaLight {
    #[new]
    fn new(colour: (f32, f32, f32), attenuation: (f32, f32, f32)) -> Self {
        let (r, g, b) = colour;
        let (c0, c1, c2) = attenuation;
        Self {
            inner: Light::new(
                Transform::default(),
                Vector3::new(r, g, b),
                Vector3::new(c0, c1, c2),
                Vec::new(),
            ),
        }
    }
}
