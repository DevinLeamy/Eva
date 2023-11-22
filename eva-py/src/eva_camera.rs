use crate::prelude::*;

#[pyclass]
#[pyo3(name = "EvaCamera")]
pub struct EvaCamera {
    pub inner: Camera,
}

#[pymethods]
impl EvaCamera {
    #[new]
    fn new(
        position: (f32, f32, f32),
        view: (f32, f32, f32),
        up: (f32, f32, f32),
        fov: f32,
    ) -> Self {
        Self {
            inner: Camera::new(
                Vector3::new(position.0, position.1, position.2),
                fov,
                Vector3::new(view.0, view.1, view.2),
                Vector3::new(up.0, up.1, up.2),
            ),
        }
    }

    fn set_fov(&mut self, fov: f32) {
        self.inner.set_fov(fov);
    }

    fn set_up(&mut self, x: f32, y: f32, z: f32) {
        self.inner.set_up(Vector3::new(x, y, z));
    }

    fn set_view(&mut self, x: f32, y: f32, z: f32) {
        self.inner.set_target(Vector3::new(x, y, z));
    }

    fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.inner.set_position(Vector3::new(x, y, z));
    }

    fn look_at(&mut self, x: f32, y: f32, z: f32) {
        self.inner.look_at(Vector3::new(x, y, z));
    }
}
