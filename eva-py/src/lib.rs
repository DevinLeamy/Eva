mod eva_camera;
mod eva_global;
mod eva_main;
mod eva_material;
mod eva_scene;
mod py_geometry;
mod py_transform;
mod utils;

mod prelude {
    pub use crate::eva_camera::EvaCamera;
    pub use crate::eva_global::EvaGlobal;
    pub use crate::eva_material::EvaMaterial;
    pub use crate::eva_scene::EvaScene;
    pub use crate::py_geometry::PyGeometry;
    pub use crate::py_transform::PyTransform;
    pub use crate::utils::*;
    pub use eva::prelude::*;
    pub use nalgebra::Vector3;
    pub use pyo3::prelude::*;

    pub use eva_py_macros::PyNode;
}

use eva_main::EvaRender;

use crate::eva_main::EvaRunDescriptor;

use crate::prelude::*;

#[pyfunction]
#[pyo3(name = "eva_main_dynamic")]
fn eva_py_main_dynamic(
    global: &EvaGlobal,
    scene: PyObject,
    camera: PyObject,
    render: PyObject,
) -> PyResult<()> {
    eva_main::main(EvaRunDescriptor {
        global,
        render: EvaRender::Dynamic {
            camera,
            scene,
            render,
        },
    });

    Ok(())
}

#[pyfunction]
#[pyo3(name = "eva_main_static")]
fn eva_py_main_static(
    global: &EvaGlobal,
    scene: PyRef<EvaScene>,
    camera: PyRef<EvaCamera>,
) -> PyResult<()> {
    eva_main::main(EvaRunDescriptor {
        global,
        render: EvaRender::Static {
            camera: camera.inner.clone(),
            scene: scene.inner.clone(),
        },
    });

    Ok(())
}
#[pymodule]
fn eva_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyGeometry>()?;
    m.add_class::<EvaScene>()?;
    m.add_class::<PyTransform>()?;
    m.add_class::<EvaCamera>()?;
    m.add_class::<EvaMaterial>()?;
    m.add_class::<EvaGlobal>()?;
    m.add_function(wrap_pyfunction!(eva_py_main_dynamic, m)?)?;
    m.add_function(wrap_pyfunction!(eva_py_main_static, m)?)?;

    Ok(())
}
