use nalgebra::{Unit, Vector3};

use crate::{
    prelude::{Intersection, Ray},
    scene::Collidable,
};

#[derive(Clone, Debug)]
pub struct Triangle {
    p1: Vector3<f32>,
    p2: Vector3<f32>,
    p3: Vector3<f32>,
}

impl Triangle {
    /// Create a new triangle with three vertices.
    pub fn new(p1: Vector3<f32>, p2: Vector3<f32>, p3: Vector3<f32>) -> Self {
        Self { p1, p2, p3 }
    }
}

impl Collidable for Triangle {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let edge1 = self.p2 - self.p1;
        let edge2 = self.p3 - self.p1;

        let cross_dir_edge2 = ray.direction().cross(&edge2);
        let det = edge1.dot(&cross_dir_edge2);

        const EPSILON: f32 = 1e-10;
        if det.abs() < EPSILON {
            return None; // Parallel or lies in triangle plane
        }

        let inv_det = 1.0 / det;
        let to_origin = ray.origin() - self.p1;
        let u = inv_det * to_origin.dot(&cross_dir_edge2);

        if !(0.0..=1.0).contains(&u) {
            return None;
        }

        let cross_origin_edge1 = to_origin.cross(&edge1);
        let v = inv_det * ray.direction().dot(&cross_origin_edge1);

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = inv_det * edge2.dot(&cross_origin_edge1);
        if t > EPSILON {
            return Some(Intersection::new(
                ray.clone(),
                None,
                t,
                Unit::new_normalize(edge1.cross(&edge2)),
            ));
        }

        None
    }
}
