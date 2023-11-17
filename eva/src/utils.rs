use nalgebra::Vector3;

pub fn vector_mul(a: &Vector3<f32>, b: &Vector3<f32>) -> Vector3<f32> {
    Vector3::new(a.x * b.x, a.y * b.y, a.z * b.z)
}

pub fn extents(points: &Vec<Vector3<f32>>) -> (Vector3<f32>, Vector3<f32>) {
    let mut min = points[0];
    let mut max = points[0];

    for point in points {
        min.x = f32::min(point.x, min.x);
        min.y = f32::min(point.y, min.y);
        min.z = f32::min(point.z, min.z);

        max.x = f32::max(point.x, max.x);
        max.y = f32::max(point.y, max.y);
        max.z = f32::max(point.z, max.z);
    }

    (min.clone(), max.clone())
}
