use nalgebra::Vector3;

pub fn vector_mul(a: &Vector3<f32>, b: &Vector3<f32>) -> Vector3<f32> {
    Vector3::new(a.x * b.x, a.y * b.y, a.z * b.z)
}
