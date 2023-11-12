use std::mem::swap;

use nalgebra::{SimdBool, Unit, Vector3};

use crate::{
    prelude::{Intersection, Ray},
    scene::Collidable,
    utils::opposite_sign,
};

#[derive(Clone, Debug)]
pub struct Cube {
    min: Vector3<f32>,
    max: Vector3<f32>,
}

impl Cube {
    /// Create a cube with a given side length.
    pub fn new(size: f32) -> Self {
        Self {
            min: Vector3::zeros(),
            max: Vector3::new(size, size, size),
        }
    }

    pub fn from_points(min: Vector3<f32>, max: Vector3<f32>) -> Self {
        Self { min, max }
    }
}

impl Cube {
    fn is_inside(&self, point: &Vector3<f32>) -> bool {
        point.ge(&self.min).all() && point.le(&self.max).all()
    }
}

impl Collidable for Cube {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let (origin, direction) = (ray.origin(), ray.direction());
        if self.is_inside(&origin) {
            return None;
        }

        let mut tmin = (self.min.x - origin.x) / direction.x;
        let mut tmax = (self.max.x - origin.x) / direction.x;
        if tmin > tmax {
            swap(&mut tmin, &mut tmax);
        }

        let mut tmin_y = (self.min.y - origin.y) / direction.y;
        let mut tmax_y = (self.max.y - origin.y) / direction.y;
        if tmin_y > tmax_y {
            swap(&mut tmin_y, &mut tmax_y);
        }

        if tmin > tmax_y || tmax < tmin_y {
            return None;
        }

        tmin = f32::max(tmin, tmin_y);
        tmax = f32::min(tmax, tmax_y);

        let mut tmin_z = (self.min.z - origin.z) / direction.z;
        let mut tmax_z = (self.max.z - origin.z) / direction.z;
        if tmin_z > tmax_z {
            swap(&mut tmin_z, &mut tmax_z);
        }

        if tmin > tmax_z || tmax < tmin_z {
            return None;
        }

        tmin = f32::max(tmin, tmin_z);
        tmax = f32::min(tmax, tmax_z);

        if tmin <= 0.0 {
            return None;
        }

        let mut normal = Vector3::<f32>::zeros();
        // Flip the sign based on the direction of the incoming ray.
        if tmin > tmin_y && tmin > tmin_z {
            normal.x = opposite_sign(direction.x);
        } else if tmin_y > tmin_z {
            normal.y = opposite_sign(direction.y);
        } else {
            normal.z = opposite_sign(direction.z);
        }

        Some(Intersection::new(
            ray.clone(),
            None,
            tmin,
            Unit::new_normalize(normal),
        ))
    }
}
