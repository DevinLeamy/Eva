use std::sync::{Arc, Mutex};
use std::time::Instant;

use eva::prelude::*;
use pyo3::{PyObject, PyResult, Python};
use winit::dpi::LogicalSize;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::prelude::{EvaCamera, EvaGlobal, EvaScene};

pub enum EvaRender {
    Static {
        camera: Camera,
        scene: Scene,
    },
    Dynamic {
        camera: PyObject,
        scene: PyObject,
        render: PyObject,
    },
}

pub struct EvaRunDescriptor<'a> {
    pub global: &'a EvaGlobal,
    pub render: EvaRender,
}

pub struct DynamicThreadSyncContext {
    pub render: PyObject,
    pub camera: PyObject,
    pub scene: PyObject,
}

pub struct StaticThreadSyncContext {
    pub rendered: bool,
    pub camera: Camera,
    pub scene: Scene,
    pub screenshot: Option<PathBuf>,
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
        skybox: run.global.skybox.clone(),
        ambient: run.global.ambient,
        textures: run.global.texture_loader.clone().textures(),
        materials: run.global.materials.clone(),
        sample_count: run.global.sample_count,
        max_reflections: run.global.max_reflections,
    };
    let mut renderer = RendererBuilder::new(window, static_context).build();

    match run.render {
        EvaRender::Static { camera, scene } => {
            let sync_arc = Arc::new(Mutex::new(StaticThreadSyncContext {
                camera,
                scene,
                rendered: false,
                screenshot: run.global.screenshot_path.clone(),
            }));
            let sync_arc_clone = Arc::clone(&sync_arc);

            event_loop.run(move |event, _, control_flow| {
                let mut sync = sync_arc_clone.lock().unwrap();
                match event {
                    Event::WindowEvent {
                        window_id: _,
                        event: window_event,
                    } => match window_event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        _ => {}
                    },
                    _ => {}
                }

                if !sync.rendered {
                    sync.rendered = true;
                    let context = DynamicRenderContext {
                        scene: sync.scene.clone(),
                        camera: sync.camera.clone(),
                        screenshot: sync.screenshot.clone(),
                    };
                    renderer.render(&context).unwrap();
                }
            });
        }
        EvaRender::Dynamic {
            camera,
            scene,
            render,
        } => {
            let sync_arc = Arc::new(Mutex::new(DynamicThreadSyncContext {
                render,
                camera,
                scene,
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
                                        let render_ref: &PyObject = &sync.as_ref().unwrap().render;
                                        render_ref.call_method1(
                                            py,
                                            "handle_input",
                                            (key, state),
                                        )?;

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
                        let render_ref: &PyObject = &sync.as_ref().unwrap().render;
                        render_ref.call_method0(py, "update")?;

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

                    let context = DynamicRenderContext {
                        scene,
                        camera,
                        screenshot: None,
                    };
                    renderer.render(&context).unwrap();
                    last_frame_time = Instant::now();
                }
            });
        }
    }
}
