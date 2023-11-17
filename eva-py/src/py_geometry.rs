pub use crate::prelude::*;

enum PrimitiveType {
    Sphere,
    Cube,
}

impl From<&str> for PrimitiveType {
    fn from(val: &str) -> Self {
        match val {
            "sphere" => PrimitiveType::Sphere,
            "cube" => PrimitiveType::Cube,
            _ => panic!("invalid primitive type: {val}"),
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
    fn new(primitive_type: &str) -> PyResult<Self> {
        let primitive_type: PrimitiveType = primitive_type.into();
        let primitive: Primitive = match primitive_type {
            PrimitiveType::Sphere => Primitive::Sphere(Sphere::new(1.0)),
            PrimitiveType::Cube => Primitive::Cube(Cube::new(1.0)),
        };

        Ok(Self {
            inner: Geometry::from_primitive(primitive),
        })
    }

    fn set_material(&mut self, material: PyRef<PyMaterial>) {
        self.inner.set_material(material.inner.clone());
    }
}
