use std::path::PathBuf;

pub use crate::prelude::*;

const MESH_PATH: &str = "./wall-e-py/assets/meshes/";

#[pyclass]
#[pyo3(name = "Mesh")]
#[derive(PyNode)]
pub struct PyMesh {
    pub inner: Geometry,
}

#[pymethods]
impl PyMesh {
    #[new]
    fn new(mesh_name: String) -> Self {
        let mut path = PathBuf::from(MESH_PATH);
        path.push(mesh_name);
        let mesh = Mesh::from_path(path);

        Self {
            inner: Geometry::from_collidable(Box::new(mesh)),
        }
    }

    fn set_material(&mut self, material: PyRef<PyMaterial>) {
        self.inner.set_material(material.inner.clone());
    }
}
