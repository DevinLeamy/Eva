#[macro_use]
extern crate lazy_static;

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
mod utils;

pub mod prelude {
    pub use crate::obj_loader::*;
    pub use crate::obj_mesh::*;
    pub use crate::ray_tracer::*;
    pub use crate::renderer::*;
    pub use crate::scene::*;
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

    event_loop.run(move |event, _, control_flow| {
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
        renderer.render().unwrap()
    });
}
