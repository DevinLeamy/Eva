use std::{
    sync::{Arc, Mutex},
    time::Instant,
};

use crate::prelude::*;
use winit::{event::*, event_loop::*, window::Window, *};

pub struct Runner {
    window: Window,
    event_loop: EventLoop<()>,
}

impl Runner {
    pub fn new(window: Window, event_loop: EventLoop<()>) -> Self {
        Self { window, event_loop }
    }

    pub async fn run<G: GlobalConfig>(self, run: RunDescriptor<G>) {
        let mut renderer =
            RendererBuilder::new(self.window, run.global.static_context().await).build();

        match run.render {
            RenderMode::Static { scene } => {
                renderer.render(&scene).unwrap();

                // Close on 'x'.
                self.event_loop
                    .run(move |event, _, control_flow| match event {
                        Event::WindowEvent {
                            ref event,
                            window_id: _,
                        } => match event {
                            WindowEvent::CloseRequested
                            | WindowEvent::KeyboardInput {
                                input:
                                    KeyboardInput {
                                        state: ElementState::Pressed,
                                        virtual_keycode: Some(VirtualKeyCode::Escape),
                                        ..
                                    },
                                ..
                            } => *control_flow = ControlFlow::Exit,
                            _ => {}
                        },
                        _ => {}
                    });
            }
            RenderMode::Dynamic { scene } => {
                let sync_arc = Arc::new(Mutex::new(scene));
                let sync_arc_clone = Arc::clone(&sync_arc);
                let mut last_frame_time: Instant = Instant::now();
                self.event_loop.run(move |event, _, control_flow| {
                    let now = Instant::now();
                    let mut sync = sync_arc_clone.lock().unwrap();
                    match event {
                        Event::WindowEvent {
                            event: window_event,
                            window_id: _,
                        } => match window_event {
                            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                            WindowEvent::KeyboardInput { input, .. } => {
                                match (input.virtual_keycode, input.state) {
                                    (Some(key), state) => {
                                        let key = format!("{:?}", key);
                                        let state = format!("{:?}", state);
                                        sync.handle_input(key, state);
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        },
                        _ => {}
                    }

                    if now.duration_since(last_frame_time).as_millis() > 32 {
                        renderer.render(&sync.dynamic_context()).unwrap();
                        last_frame_time = Instant::now();
                    }
                });
            }
        }
    }
}
