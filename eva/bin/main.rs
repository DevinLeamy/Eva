use eva::{
    prelude::{Camera, Scene},
    ray_trace,
};
use nalgebra::Vector3;

fn main() {
    let camera = Camera::new(
        Vector3::zeros(),
        50.0,
        Vector3::new(0.0, 0.0, -1.0),
        Vector3::new(0.0, 1.0, 0.0),
    );
}
