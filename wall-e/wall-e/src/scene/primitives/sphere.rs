use nalgebra::Unit;

use crate::{
    prelude::{Intersection, Ray},
    scene::Collidable,
};

#[derive(Clone, Debug)]
pub struct Sphere {
    radius: f32,
}

impl Sphere {
    /// Create a sphere with a given radius.
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }
}

impl Collidable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        // Note: we assume the sphere is centered at the origin.
        let a = 1.0; // ray.direction^2
        let b = 2.0 * ray.origin().dot(&ray.direction().into_inner());
        let c = ray.origin().dot(&ray.origin()) - self.radius * self.radius;

        let disc = b * b - 4.0 * a * c;
        if disc < 0.0 {
            return None;
        }

        let t = if disc == 0.0 {
            -b / (2.0 * a)
        } else {
            let t0 = (-b + disc.sqrt()) / (2.0 * a);
            let t1 = (-b - disc.sqrt()) / (2.0 * a);

            if t0 <= 0.0 {
                t1
            } else if t1 <= 0.0 {
                t0
            } else {
                f32::min(t0, t1)
            }
        };

        if t <= 0.0 {
            return None;
        }

        let point = ray.point(t);
        // The normal at this point is simple the normalized point, because the
        // sphere is centered at the origin.
        let normal = Unit::new_normalize(point);

        Some(Intersection::new(ray.clone(), None, t, normal))
    }
}
