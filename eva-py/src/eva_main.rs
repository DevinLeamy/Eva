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

use crate::prelude::EvaCamera;

pub struct EvaRunDescriptor {
    pub camera: PyObject,
    pub scene: Scene,
    pub textures: ShaderTextures,
    pub skybox: ShaderSkybox,
    pub update: PyObject,
}

pub struct ThreadSyncContext {
    pub update: PyObject,
    pub context: DynamicRenderContext,
    pub camera: PyObject,
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

    let context = DynamicRenderContext {
        scene: run.scene.clone(),
        camera: Camera::default(),
    };
    let sync_arc = Arc::new(Mutex::new(ThreadSyncContext {
        update: run.update,
        context,
        camera: run.camera,
    }));
    let sync_arc_clone = Arc::clone(&sync_arc);

    let mut last_frame_time: Instant = Instant::now();
    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                window_id: _,
                event: window_event,
            } => match window_event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput { input, .. } => {
                    match (input.virtual_keycode, input.state) {
                        (Some(key), state) => {
                            // renderer.update(key, state);
                        }

                        _ => {}
                    }
                }
                _ => {}
            },
            _ => {}
        }
        let now = Instant::now();
        let sync = sync_arc_clone.lock();
        let mut camera = Camera::default();
        Python::with_gil(|py| -> PyResult<()> {
            let py_func_ref: &PyFunction = sync.as_ref().unwrap().update.downcast(py)?;
            py_func_ref.call1(())?;

            let py_camera_ref = &sync.as_ref().unwrap().camera;
            let eva_camera: EvaCamera = py_camera_ref
                .getattr(py, "inner")
                .unwrap()
                .extract(py)
                .unwrap();
            camera = eva_camera.inner;

            Ok(())
        })
        .unwrap();

        println!("{:?}", camera);

        if now.duration_since(last_frame_time).as_millis() > 32 {
            let context = DynamicRenderContext {
                scene: sync.as_ref().unwrap().context.scene.clone(),
                camera
            };
            renderer.render(&context).unwrap();
            last_frame_time = Instant::now();
        }
    });
}
