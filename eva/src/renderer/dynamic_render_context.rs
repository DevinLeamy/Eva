use std::path::PathBuf;

use crate::prelude::{Camera, Scene};

pub struct DynamicRenderContext {
    pub camera: Camera,
    pub scene: Scene,
    /// Path to save a screenshot. A screenshot is only taken is a path is provided.
    pub screenshot: Option<PathBuf>,
}
