mod py_camera;
mod py_geometry;
mod py_light;
mod py_material;
mod py_scene;
mod py_transform;

mod prelude {
    pub use crate::py_camera::PyCamera;
    pub use crate::py_geometry::PyGeometry;
    pub use crate::py_light::PyLight;
    pub use crate::py_material::PyMaterial;
    pub use crate::py_scene::PyScene;
    pub use crate::py_transform::PyTransform;
    pub use eva::prelude::*;
    pub use nalgebra::Vector3;
    pub use pyo3::prelude::*;

    pub use eva_py_macros::PyNode;
}

use crate::prelude::*;

#[pyfunction]
#[pyo3(name = "ray_trace")]
fn eva_py_ray_trace(scene: &PyScene, camera: &PyCamera) -> PyResult<()> {
    pollster::block_on(eva::prelude::ray_trace(
        camera.inner.clone(),
        Scene {
            ambient: scene.ambient,
            root: scene.root.clone(),
            skybox: scene.skybox.clone(),
            textures: scene.texture_loader.clone().textures(),
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
    m.add_function(wrap_pyfunction!(eva_py_ray_trace, m)?)?;

    Ok(())
}
