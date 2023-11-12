use nalgebra::{Unit, Vector3};

use crate::prelude::Transform;

#[derive(Clone, Debug)]
pub struct Ray {
    origin: Vector3<f32>,
    direction: Unit<Vector3<f32>>,
}

impl Ray {
    pub fn new(origin: Vector3<f32>, direction: Unit<Vector3<f32>>) -> Self {
        Self { origin, direction }
    }

    pub fn from_points(src: Vector3<f32>, dest: Vector3<f32>) -> Self {
        Self::new(src, Unit::new_normalize(dest - src))
    }

    pub fn origin(&self) -> Vector3<f32> {
        self.origin
    }

    pub fn direction(&self) -> Unit<Vector3<f32>> {
        self.direction
    }

    pub fn point(&self, t: f32) -> Vector3<f32> {
        self.origin + self.direction.into_inner() * t
    }

    pub fn t(&self, point: &Vector3<f32>) -> f32 {
        let dist = point - self.origin;
        dist.magnitude()
    }

    pub fn into_inverse_transform_ray(&self, transform: &Transform) -> Self {
        let p1 = self.origin.push(1.0);
        let p2 = (self.origin + self.direction.into_inner()).push(1.0);
        let m = transform.as_mat4().try_inverse().unwrap();
        let new_p1 = m * p1;
        let new_p2 = m * p2;

        Self::from_points(new_p1.xyz(), new_p2.xyz())
    }

    pub fn into_transformed_ray(&self, transform: &Transform) -> Self {
        let p1 = self.origin.push(1.0);
        let p2 = (self.origin + self.direction.into_inner()).push(1.0);
        let m = transform.as_mat4();
        let new_p1 = m * p1;
        let new_p2 = m * p2;

        Self::from_points(new_p1.xyz(), new_p2.xyz())
    }
}
