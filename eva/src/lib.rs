#[macro_use]
extern crate lazy_static;

use std::time::Instant;

use renderer::{Renderer, RendererBuilder};
use winit::dpi::LogicalSize;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod obj_loader;
mod obj_mesh;
mod ray_tracer;
mod renderer;
mod scene;
mod shader;
mod utils;

pub mod prelude {
    pub use crate::obj_loader::*;
    pub use crate::obj_mesh::*;
    pub use crate::ray_tracer::*;
    pub use crate::renderer::*;
    pub use crate::scene::*;
    pub use crate::shader::*;
    pub use crate::utils::*;
}

pub async fn run() {
    env_logger::init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Unnamed")
        .with_inner_size(LogicalSize::new(850, 850))
        .build(&event_loop)
        .unwrap();

    let window_id = window.id();
    let mut renderer = RendererBuilder::new(window).build();

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
                            renderer.update(key, state);
                        }

                        _ => {}
                    }
                }
                _ => {}
            },
            _ => {}
        }
        let now = Instant::now();
        if now.duration_since(last_frame_time).as_millis() > 32 {
            renderer.render().unwrap();
            last_frame_time = Instant::now();
        }
    });
}
