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

use std::path::PathBuf;

use eva::prelude::{Node, TextureLoader};

use crate::prelude::*;

const TEXTURE_PATH: &str = "./eva/assets/textures/";

#[pyclass]
#[pyo3(name = "Scene")]
struct PyScene {
    root: Node,
    ambient: Vector3<f32>,
    texture_loader: TextureLoader,
}

#[pymethods]
impl PyScene {
    #[new]
    fn new() -> Self {
        let mut texture_loader = TextureLoader::new();
        let mut path = PathBuf::from(TEXTURE_PATH);
        path.push("missing.png".to_string());
        texture_loader.load(path);

        Self {
            root: Node::Transformation(Transformation::new()),
            ambient: Vector3::new(0.1, 0.1, 0.1),
            texture_loader,
        }
    }

    fn set_root(&mut self, py: Python, root: PyObject) {
        if let Ok(child) = root.extract::<PyRef<PyGeometry>>(py) {
            self.root = child.inner.clone().into();
        } else if let Ok(child) = root.extract::<PyRef<PyTransform>>(py) {
            self.root = child.inner.clone().into();
        } else if let Ok(child) = root.extract::<PyRef<PyLight>>(py) {
            self.root = child.inner.clone().into();
        } else if let Ok(child) = root.extract::<PyRef<PyMesh>>(py) {
            self.root = child.inner.clone().into();
        } else {
            panic!("add_child only accepts PyGeometry, PyTransform, PyLight, or PyMesh");
        }
    }

    fn set_ambient(&mut self, r: f32, g: f32, b: f32) {
        self.ambient = Vector3::new(r, g, b);
    }

    fn add_texture(&mut self, texture_name: String) -> u32 {
        let mut path = PathBuf::from(TEXTURE_PATH);
        path.push(texture_name);
        self.texture_loader.load(path)
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
        Scene {
            ambient: scene.ambient,
            root: scene.root.clone(),
            textures: scene.texture_loader.textures().clone(),
        },
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
