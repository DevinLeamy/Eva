use crate::prelude::{Scene, Camera};

// We'll assume for now that this is _before_ the scene is read for the shader.
// I.e. we store Camera and Scene and not ShaderCamera and FlatScene
pub struct RenderContext {
    pub camera: Camera,
    pub scene: Scene,
}
