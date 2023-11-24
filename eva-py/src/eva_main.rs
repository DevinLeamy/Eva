use std::sync::{Arc, Mutex};
use std::time::Instant;

use eva::prelude::*;
use pyo3::types::PyFunction;
use pyo3::{PyObject, PyResult, Python};
use winit::dpi::LogicalSize;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::prelude::{EvaCamera, EvaScene};

pub struct EvaRunDescriptor {
    pub camera: PyObject,
    pub scene: PyObject,
    pub textures: ShaderTextures,
    pub skybox: ShaderSkybox,
    pub update: PyObject,
    pub input_handler: PyObject,
}

pub struct ThreadSyncContext {
    pub update: PyObject,
    pub camera: PyObject,
    pub scene: PyObject,
    pub input_handler: PyObject,
}

pub fn main(run: EvaRunDescriptor) {
    env_logger::init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Eva")
        .with_inner_size(LogicalSize::new(850, 850))
        .build(&event_loop)
        .unwrap();
    let static_context = StaticRenderContext {
        textures: run.textures.clone(),
        skybox: run.skybox.clone(),
    };
    let mut renderer = RendererBuilder::new(window, static_context).build();

    let sync_arc = Arc::new(Mutex::new(ThreadSyncContext {
        update: run.update,
        camera: run.camera,
        scene: run.scene,
        input_handler: run.input_handler,
    }));
    let sync_arc_clone = Arc::clone(&sync_arc);

    let mut last_frame_time: Instant = Instant::now();
    event_loop.run(move |event, _, control_flow| {
        let now = Instant::now();
        let sync = sync_arc_clone.lock();

        match event {
            Event::WindowEvent {
                window_id: _,
                event: window_event,
            } => match window_event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput { input, .. } => {
                    match (input.virtual_keycode, input.state) {
                        (Some(key), state) => {
                            let key = format!("{:?}", key);
                            let state = format!("{:?}", state);

                            Python::with_gil(|py| -> PyResult<()> {
                                let py_input_handler: &PyFunction =
                                    sync.as_ref().unwrap().input_handler.downcast(py)?;
                                py_input_handler.call1((key, state))?;

                                Ok(())
                            })
                            .unwrap();
                        }

                        _ => {}
                    }
                }
                _ => {}
            },
            _ => {}
        }

        if now.duration_since(last_frame_time).as_millis() > 32 {
            let (camera, scene) = Python::with_gil(|py| -> PyResult<(Camera, Scene)> {
                let py_func_ref: &PyFunction = sync.as_ref().unwrap().update.downcast(py)?;
                py_func_ref.call1(())?;

                let py_camera_ref = &sync.as_ref().unwrap().camera;
                let py_scene_ref = &sync.as_ref().unwrap().scene;
                let eva_camera: EvaCamera = py_camera_ref
                    .getattr(py, "inner")
                    .unwrap()
                    .extract(py)
                    .unwrap();
                let eva_scene: EvaScene = py_scene_ref
                    .call_method1(py, "inner", ())
                    .unwrap()
                    .extract(py)
                    .unwrap();

                Ok((eva_camera.inner, eva_scene.inner))
            })
            .unwrap();

            let context = DynamicRenderContext { scene, camera };
            renderer.render(&context).unwrap();
            last_frame_time = Instant::now();
        }
    });
}
