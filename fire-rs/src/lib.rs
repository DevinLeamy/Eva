use wasm_bindgen::prelude::*;

use winit::dpi::{LogicalSize, PhysicalSize};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    keyboard::{Key, NamedKey},
    window::WindowBuilder,
};

#[wasm_bindgen(start)]
pub fn run() {
    env_logger::init();
    // std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    // console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");

    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("Eva")
        .with_inner_size(LogicalSize::new(850, 850))
        .build(&event_loop)
        .unwrap();
    let id = window.id();

    event_loop
        .run(move |event, control_flow| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == id => match event {
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            state: ElementState::Pressed,
                            logical_key: Key::Named(NamedKey::Escape),
                            ..
                        },
                    ..
                } => control_flow.exit(),
                _ => {}
            },
            _ => {}
        })
        .unwrap();

    // web_sys::window()
    //     .and_then(|win| win.document())
    //     .and_then(|doc| {
    //         let dst = doc.get_element_by_id("fire-canvas")?;
    //         let canvas = web_sys::Element::from(window.canvas().unwrap());
    //         dst.append_child(&canvas).ok()?;
    //         Some(())
    //     })
    //     .expect("Couldn't append canvas to document body.");
}
