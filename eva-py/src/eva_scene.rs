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
}
