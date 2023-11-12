#[macro_use]
extern crate lazy_static;

mod config;
mod obj_loader;
mod obj_mesh;
mod output;
mod ray_tracer;
mod scene;
mod utils;

pub mod prelude {
    pub use crate::output::*;
    pub use crate::ray_tracer::*;
    pub use crate::scene::*;
    pub use nalgebra::Vector3;
    pub use crate::config::*;
}
