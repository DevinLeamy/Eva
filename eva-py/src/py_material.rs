use crate::prelude::*;

#[pyclass]
#[pyo3(name = "Material")]
pub struct PyMaterial {
    pub inner: PhongMaterial,
}

#[pymethods]
impl PyMaterial {
    #[new]
    fn new(kd: (f32, f32, f32), ks: (f32, f32, f32), shininess: f32) -> Self {
        Self {
            inner: PhongMaterial::new(
                Vector3::new(kd.0, kd.1, kd.2),
                Vector3::new(ks.0, ks.1, ks.2),
                shininess,
            ),
        }
    }
}
