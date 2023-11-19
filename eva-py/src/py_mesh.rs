pub use crate::prelude::*;
use std::path::PathBuf;

const MESH_PATH: &str = "./eva-py/assets/meshes/";

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
            inner: Geometry::from_primitive(Primitive::Mesh(mesh)),
        }
    }

    fn set_material(&mut self, material: PyRef<PyMaterial>) {
        self.inner.set_material(material.inner.clone());
    }

    fn set_texture(&mut self, texture_id: u32) {
        self.inner.material_mut().set_texture(texture_id);
    }
}
