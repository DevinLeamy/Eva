pub use crate::prelude::*;

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
            GeometryType::Mesh(name) => Primitive::Mesh(Mesh::from_path(name)),
        };

        Ok(Self {
            inner: Geometry::from_primitive(primitive),
        })
    }

    fn set_material(&mut self, material_id: u32) {
        self.inner.set_material(material_id);
    }
}
