pub use crate::prelude::*;

const MESH_PATH: &str = "./eva-py/assets/meshes/";

enum GeometryType {
    Sphere,
    Cube,
    Mesh(String),
}

impl From<&str> for GeometryType {
    fn from(val: &str) -> Self {
        match val {
            "sphere" => GeometryType::Sphere,
            "cube" => GeometryType::Cube,
            name => GeometryType::Mesh(name.to_string()),
        }
    }
}

#[pyclass]
#[pyo3(name = "Geometry")]
#[derive(PyNode)]
pub struct PyGeometry {
    pub inner: Geometry,
}

#[pymethods]
impl PyGeometry {
    #[new]
    fn new(geometry_type: &str) -> PyResult<Self> {
        let geometry_type: GeometryType = geometry_type.into();
        let primitive: Primitive = match geometry_type {
            GeometryType::Sphere => Primitive::Sphere(Sphere::new(1.0)),
            GeometryType::Cube => Primitive::Cube(Cube::new(1.0)),
            GeometryType::Mesh(name) => {
                let mut path = PathBuf::from(MESH_PATH);
                path.push(name);
                Primitive::Mesh(Mesh::from_path(path))
            }
        };

        Ok(Self {
            inner: Geometry::from_primitive(primitive),
        })
    }

    fn set_material(&mut self, material: PyRef<PyMaterial>) {
        self.inner.set_material(material.inner.clone());
    }

    fn set_texture(&mut self, texture_id: u32) {
        self.inner.material_mut().set_texture(texture_id);
    }
}
