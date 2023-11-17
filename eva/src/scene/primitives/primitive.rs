use super::{Cube, Mesh, Sphere};

#[derive(Clone, Debug)]
pub enum Primitive {
    Cube(Cube),
    Sphere(Sphere),
    Mesh(Mesh),
}
