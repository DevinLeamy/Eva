use crate::prelude::*;

#[pyclass]
#[derive(Clone)]
pub struct EvaMaterial {
    pub inner: PbrMaterial,
}

#[pymethods]
impl EvaMaterial {
    #[new]
    fn new(roughness: f32, metallic: f32, albedo: (f32, f32, f32)) -> Self {
        Self {
            inner: PbrMaterial {
                roughness,
                albedo: Vector3::new(albedo.0, albedo.1, albedo.2),
                metallic,
            },
        }
    }

    fn set_texture(&mut self, texture_id: u32) {
        self.inner.set_texture(texture_id)
    }
}
