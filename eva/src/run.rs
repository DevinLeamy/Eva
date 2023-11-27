pub use crate::prelude::*;

pub enum RenderMode {
    Static { scene: DynamicRenderContext },
    Dynamic { scene: Box<dyn DynamicScene> },
}

pub trait DynamicScene {
    fn update(&mut self);
    fn handle_input(&mut self, key: String, state: String);
    fn dynamic_context(&self) -> DynamicRenderContext;
}

pub trait GlobalConfig {
    async fn static_context(&self) -> StaticRenderContext;
}

pub struct RunDescriptor<G: GlobalConfig> {
    pub global: G,
    pub render: RenderMode,
}
