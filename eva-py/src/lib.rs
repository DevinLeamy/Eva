mod eva_camera;
mod eva_global;
mod eva_light;
mod eva_main;
mod eva_scene;
mod py_geometry;
mod py_material;
mod py_transform;

mod prelude {
    pub use crate::eva_camera::EvaCamera;
    pub use crate::eva_global::EvaGlobal;
    pub use crate::eva_light::EvaLight;
    pub use crate::eva_scene::EvaScene;
    pub use crate::py_geometry::PyGeometry;
    pub use crate::py_material::PyMaterial;
    pub use crate::py_transform::PyTransform;
    pub use eva::prelude::*;
    pub use nalgebra::Vector3;
    pub use pyo3::prelude::*;

    pub use eva_py_macros::PyNode;
}

use crate::eva_main::EvaRunDescriptor;

use crate::prelude::*;

#[pyfunction]
#[pyo3(name = "eva_main")]
fn eva_py_main(
    global: &EvaGlobal,
    scene: PyObject,
    camera: PyObject,
    update: PyObject,
    input_handler: PyObject,
) -> PyResult<()> {
    eva_main::main(EvaRunDescriptor {
        global,
        camera,
        scene,
        update,
        input_handler,
    });

    Ok(())
}

#[pymodule]
fn eva_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyGeometry>()?;
    m.add_class::<EvaScene>()?;
    m.add_class::<EvaLight>()?;
    m.add_class::<PyTransform>()?;
    m.add_class::<EvaCamera>()?;
    m.add_class::<PyMaterial>()?;
    m.add_class::<EvaGlobal>()?;
    m.add_function(wrap_pyfunction!(eva_py_main, m)?)?;

    Ok(())
}
