use crate::prelude::*;

#[pyclass]
#[pyo3(name = "Transform")]
#[derive(PyNode)]
pub struct PyTransform {
    pub inner: Transformation,
}

#[pymethods]
impl PyTransform {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self {
            inner: Transformation::new(),
        })
    }
}
