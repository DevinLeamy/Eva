#[macro_use]
extern crate lazy_static;

mod asset_loader;
mod config;
mod obj_loader;
mod obj_mesh;
mod renderer;
mod scene;
mod screenshot;
mod shader;
mod texture_loader;
mod utils;

pub mod prelude {
    pub use std::path::PathBuf;
    pub use wgpu::*;

    pub use crate::config::*;
    pub use crate::obj_loader::*;
    pub use crate::obj_mesh::*;
    pub use crate::renderer::*;
    pub use crate::scene::*;
    pub use crate::screenshot::*;
    pub use crate::shader::*;
    pub use crate::texture_loader::*;
    pub use crate::utils::*;

    pub use eva_macros::*;
}
