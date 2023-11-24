use crate::prelude::*;

#[pyclass]
#[pyo3(name = "EvaScene")]
#[derive(Clone)]
pub struct EvaScene {
    pub inner: Scene,
}

#[pymethods]
impl EvaScene {
    #[new]
    fn new() -> Self {
        Self {
            inner: Scene {
                root: Node::Transformation(Transformation::new()),
                ambient: Vector3::new(0.1, 0.1, 0.1),
            },
        }
    }

    fn set_root(&mut self, py: Python, root: PyObject) {
        if let Ok(child) = root.extract::<PyRef<PyGeometry>>(py) {
            self.inner.root = child.inner.clone().into();
        } else if let Ok(child) = root.extract::<PyRef<PyTransform>>(py) {
            self.inner.root = child.inner.clone().into();
        } else if let Ok(child) = root.extract::<PyRef<EvaLight>>(py) {
            self.inner.root = child.inner.clone().into();
        } else {
            panic!("add_child only accepts PyGeometry, PyTransform, or EvaLight");
        }
    }

    fn set_ambient(&mut self, r: f32, g: f32, b: f32) {
        self.inner.ambient = Vector3::new(r, g, b);
    }
}
