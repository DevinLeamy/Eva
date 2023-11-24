#[macro_use]
extern crate lazy_static;

use std::sync::{Arc, Mutex};
use std::time::Instant;

use prelude::{
    Camera, DynamicRenderContext, Scene, ShaderSkybox, ShaderTextures, StaticRenderContext,
};
use pyo3::types::PyFunction;
use pyo3::{PyObject, PyResult, Python};
use renderer::RendererBuilder;
use winit::dpi::LogicalSize;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod config;
mod obj_loader;
mod obj_mesh;
mod renderer;
mod scene;
mod shader;
mod texture_loader;
mod utils;

pub mod prelude {
    pub use std::path::PathBuf;
    pub use wgpu::*;

    pub use crate::config::*;
    pub use crate::obj_loader::*;
    pub use crate::obj_mesh::*;
    pub use crate::renderer::*;
    pub use crate::scene::*;
    pub use crate::shader::*;
    pub use crate::texture_loader::*;
    pub use crate::utils::*;

    pub use crate::main;
    pub use eva_macros::*;
}

pub struct EvaRunDescriptor {
    camera: Camera,
    scene: Scene,
    textures: ShaderTextures,
    skybox: ShaderSkybox,
    update: PyObject,
}

pub fn main(run: EvaRunDescriptor) {
    env_logger::init();
    let py_update_arc = Arc::new(Mutex::new(run.update));

    let context = StaticRenderContext {
        textures: run.textures.clone(),
        skybox: run.skybox.clone(),
    };

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Eva")
        .with_inner_size(LogicalSize::new(850, 850))
        .build(&event_loop)
        .unwrap();

    let mut renderer = RendererBuilder::new(window, context).build();
    let mut last_frame_time: Instant = Instant::now();

    let py_update_clone = Arc::clone(&py_update_arc);
    let context_arc = Arc::new(Mutex::new(DynamicRenderContext {
        scene: run.scene,
        camera: run.camera,
    }));
    let context_arc_clone = Arc::clone(&context_arc);

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
        Python::with_gil(|py| -> PyResult<()> {
            let py_update_inner = py_update_clone.lock();
            let py_func_ref: &PyFunction = py_update_inner.as_ref().unwrap().downcast(py)?;
            py_func_ref.call1(())?;

            Ok(())
        })
        .unwrap();
        if now.duration_since(last_frame_time).as_millis() > 32 {
            renderer
                .render(&DynamicRenderContext {
                    camera: context_arc_clone.lock().unwrap().camera.clone(),
                    scene: context_arc_clone.lock().unwrap().scene.clone(),
                })
                .unwrap();
            last_frame_time = Instant::now();
        }
    });
}
