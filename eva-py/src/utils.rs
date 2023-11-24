use nalgebra::Vector3;
use pyo3::IntoPy;

pub struct PyVector3<T>(pub Vector3<T>);

impl<T> Into<PyVector3<T>> for Vector3<T> {
    fn into(self) -> PyVector3<T> {
        PyVector3::<T>(self)
    }
}

impl<T: IntoPy<f32> + Copy> IntoPy<[f32; 3]> for PyVector3<T> {
    fn into_py(self, py: pyo3::prelude::Python<'_>) -> [f32; 3] {
        [
            self.0[0].into_py(py),
            self.0[1].into_py(py),
            self.0[2].into_py(py),
        ]
    }
}
