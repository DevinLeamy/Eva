use pollster::FutureExt;
use renderer::Renderer;
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod renderer;

pub async fn run() {
    env_logger::init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Eva")
        .with_inner_size(LogicalSize::new(850, 850))
        .build(&event_loop)
        .unwrap();

    let window_id = window.id();
    let mut renderer = Renderer::new(window).await;

    event_loop.run(move |event, _, control_flow| renderer.render());
}
