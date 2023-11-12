use std::path::PathBuf;

use super::Collidable;
use crate::{
    config,
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

impl Collidable for Mesh {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        if *config::RENDER_BOUNDING_VOLUMES.lock().unwrap() {
            return self.bounding_cube.intersect(ray);
        }

        if self.bounding_cube.intersect(ray).is_none() {
            return None;
        }

        let mut intersection: Option<Intersection> = None;
        for triangle in &self.triangles {
            if let Some(hit) = triangle.intersect(ray) {
                if intersection.is_none() || intersection.as_ref().unwrap().t() > hit.t() {
                    intersection = Some(hit);
                }
            }
        }

        intersection
    }
}
