use std::path::PathBuf;

use crate::{
    obj_loader::ObjLoader,
    prelude::*,
    utils::{vector_max_mut, vector_min_mut},
};

#[derive(Clone, Debug)]
pub struct Mesh {
    triangles: Vec<Triangle>,
    bounding_cube: Cube,
}

impl Mesh {
    pub fn from_path<P: Into<PathBuf>>(path: P) -> Self {
        let obj_mesh = ObjLoader::load(path).unwrap();
        let mut triangles = Vec::new();
        let mut min = obj_mesh.positions()[0];
        let mut max = obj_mesh.positions()[0];

        for triangle in obj_mesh.triangles() {
            let position_indices = &triangle.vertices;
            let p1 = obj_mesh.positions()[position_indices[0]];
            let p2 = obj_mesh.positions()[position_indices[1]];
            let p3 = obj_mesh.positions()[position_indices[2]];
            triangles.push(Triangle::new(p1, p2, p3));

            vector_min_mut(&mut min, &p1);
            vector_min_mut(&mut min, &p2);
            vector_min_mut(&mut min, &p3);

            vector_max_mut(&mut max, &p1);
            vector_max_mut(&mut max, &p2);
            vector_max_mut(&mut max, &p3);
        }

        Self {
            triangles,
            bounding_cube: Cube::from_points(min, max),
        }
    }
}
