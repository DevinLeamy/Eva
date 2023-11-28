#[macro_use]
extern crate lazy_static;

mod config;
mod obj_loader;
mod obj_mesh;
mod renderer;
mod scene;
mod shader;
mod texture_loader;
mod utils;
mod asset_loader;

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

    pub use eva_macros::*;
}

