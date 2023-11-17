mod py_camera;
mod py_geometry;
mod py_light;
mod py_material;
mod py_mesh;
mod py_transform;

mod prelude {
    pub use crate::py_camera::PyCamera;
    pub use crate::py_geometry::PyGeometry;
    pub use crate::py_light::PyLight;
    pub use crate::py_material::PyMaterial;
    pub use crate::py_mesh::PyMesh;
    pub use crate::py_transform::PyTransform;
    pub use eva::prelude::{
        Camera, Cube, Geometry, Light, Mesh, PhongMaterial, Primitive, Scene, Sphere,
        Transformation,
    };
    pub use nalgebra::Vector3;
    pub use pyo3::prelude::*;

    pub use eva_py_macros::PyNode;
}

use crate::prelude::*;

#[pyclass]
#[pyo3(name = "Scene")]
struct PyScene {
    inner: Scene,
}

#[pymethods]
impl PyScene {
    #[new]
    fn new() -> Self {
        Self {
            inner: Scene::new(),
        }
    }

    fn set_root(&mut self, py: Python, root: PyObject) {
        if let Ok(child) = root.extract::<PyRef<PyGeometry>>(py) {
            *self.inner.root_mut() = child.inner.clone().into();
        } else if let Ok(child) = root.extract::<PyRef<PyTransform>>(py) {
            *self.inner.root_mut() = child.inner.clone().into();
        } else if let Ok(child) = root.extract::<PyRef<PyLight>>(py) {
            *self.inner.root_mut() = child.inner.clone().into();
        } else if let Ok(child) = root.extract::<PyRef<PyMesh>>(py) {
            *self.inner.root_mut() = child.inner.clone().into();
        } else {
            panic!("add_child only accepts PyGeometry, PyTransform, PyLight, or PyMesh");
        }
    }

    fn set_ambient(&mut self, r: f32, g: f32, b: f32) {
        self.inner.set_ambient(Vector3::new(r, g, b))
    }
}

#[pyfunction]
fn ray_trace(
    scene: &PyScene,
    camera: &PyCamera,
    width: u32,
    height: u32,
    path: String,
) -> PyResult<()> {
    pollster::block_on(eva::prelude::ray_trace(
        camera.inner.clone(),
        scene.inner.clone(),
    ));

    Ok(())
}

#[pymodule]
fn eva_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyGeometry>()?;
    m.add_class::<PyScene>()?;
    m.add_class::<PyLight>()?;
    m.add_class::<PyTransform>()?;
    m.add_class::<PyCamera>()?;
    m.add_class::<PyMaterial>()?;
    m.add_class::<PyMesh>()?;
    m.add_function(wrap_pyfunction!(ray_trace, m)?)?;

    Ok(())
}
