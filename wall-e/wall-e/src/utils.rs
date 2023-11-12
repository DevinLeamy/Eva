use nalgebra::{Unit, Vector3};

use crate::prelude::Transform;

pub fn opposite_sign(n: f32) -> f32 {
    if n.is_sign_negative() {
        1.0
    } else {
        -1.0
    }
}

pub fn transform_normal(normal: Unit<Vector3<f32>>, transform: &Transform) -> Unit<Vector3<f32>> {
    Unit::new_normalize(transform.as_mat3_inverse() * normal.into_inner())
}

pub fn vector_mul(a: &Vector3<f32>, b: &Vector3<f32>) -> Vector3<f32> {
    Vector3::new(a.x * b.x, a.y * b.y, a.z * b.z)
}

pub fn vector_min_mut(a: &mut Vector3<f32>, b: &Vector3<f32>) {
    a.x = a.x.min(b.x);
    a.y = a.y.min(b.y);
    a.z = a.z.min(b.z);
}

pub fn vector_max_mut(a: &mut Vector3<f32>, b: &Vector3<f32>) {
    a.x = a.x.max(b.x);
    a.y = a.y.max(b.y);
    a.z = a.z.max(b.z);
}
