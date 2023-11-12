use nalgebra::{Matrix3, Matrix4, Rotation3, Vector3};

use crate::utils::vector_mul;

#[derive(Clone, Debug)]
pub struct Transform {
    rotation: Vector3<f32>,
    translation: Vector3<f32>,
    scale: Vector3<f32>,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            rotation: Vector3::zeros(),
            translation: Vector3::zeros(),
            scale: Vector3::new(1.0, 1.0, 1.0),
        }
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::new()
    }
}

impl Transform {
    pub fn rotation(&self) -> Vector3<f32> {
        self.rotation
    }

    pub fn set_rotation(&mut self, v: Vector3<f32>) {
        self.rotation = v;
    }

    pub fn rotate_x(&mut self, rad: f32) {
        self.rotation.x += rad;
    }

    pub fn rotate_y(&mut self, rad: f32) {
        self.rotation.y += rad;
    }

    pub fn rotate_z(&mut self, rad: f32) {
        self.rotation.z += rad;
    }

    pub fn scale(&self) -> Vector3<f32> {
        self.scale
    }

    pub fn set_scale(&mut self, v: Vector3<f32>) {
        self.scale = v;
    }

    pub fn scale_nonuniform(&mut self, v: Vector3<f32>) {
        let mut scale = self.scale();
        scale.x *= v.x;
        scale.y *= v.y;
        scale.z *= v.z;
        self.set_scale(scale);
    }

    pub fn translation(&self) -> Vector3<f32> {
        self.translation
    }

    pub fn set_translation(&mut self, v: Vector3<f32>) {
        self.translation = v;
    }

    pub fn translate(&mut self, v: Vector3<f32>) {
        self.set_translation(self.translation() + v);
    }

    /// Apply the given transform to self and return the new transform.
    pub fn transform(&self, transform: &Transform) -> Transform {
        let new_scale = vector_mul(&self.scale, &transform.scale);
        let current_rotation = rotation_to_rot3(self.rotation);
        let other_rotation = rotation_to_rot3(transform.rotation);
        let new_rot3 = current_rotation * other_rotation;
        let rotated_other_translation = current_rotation * transform.translation;
        let new_translation = self.translation + rotated_other_translation;
        let new_rotation = rot3_to_rotation(new_rot3);

        Transform {
            rotation: new_rotation,
            translation: new_translation,
            scale: new_scale,
        }
    }

    pub fn as_mat4(&self) -> Matrix4<f32> {
        // Order: T - R - S
        let scale_matrix = Matrix4::new_nonuniform_scaling(&self.scale);
        let rotation_matrix = rotation_to_rot3(self.rotation).matrix().to_homogeneous();
        let translation_matrix = Matrix4::new_translation(&self.translation);

        translation_matrix * rotation_matrix * scale_matrix
    }

    pub fn as_mat3_inverse(&self) -> Matrix3<f32> {
        let reciprocal_scale =
            Vector3::new(1.0 / self.scale.x, 1.0 / self.scale.y, 1.0 / self.scale.z);
        let scale_matrix = Matrix4::new_nonuniform_scaling(&reciprocal_scale);
        let rotation_matrix = rotation_to_rot3(self.rotation).matrix().to_homogeneous();

        // Order: S - R
        let m4 = scale_matrix * rotation_matrix;
        let m3_view = m4.fixed_slice::<3, 3>(0, 0);
        let m = Matrix3::from_iterator(m3_view.iter().cloned());

        m
    }
}

fn rotation_to_rot3(rotation: Vector3<f32>) -> Rotation3<f32> {
    Rotation3::from_euler_angles(rotation.x, rotation.y, rotation.z)
}

fn rot3_to_rotation(r: Rotation3<f32>) -> Vector3<f32> {
    let (x, y, z) = r.euler_angles();
    Vector3::new(x, y, z)
}
