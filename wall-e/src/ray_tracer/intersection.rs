use nalgebra::{Unit, Vector3};

use crate::{
    prelude::{PhongMaterial, Transform},
    utils::transform_normal,
};

use super::Ray;

#[derive(Clone, Debug)]
pub struct Intersection {
    t: f32,
    ray: Ray,
    material: PhongMaterial,
    /// The surface normal at the point of intersection.
    normal: Unit<Vector3<f32>>,
}

impl Intersection {
    pub fn new(
        ray: Ray,
        material: Option<PhongMaterial>,
        t: f32,
        normal: Unit<Vector3<f32>>,
    ) -> Self {
        Self {
            t,
            material: material.unwrap_or_default(),
            ray,
            normal,
        }
    }

    pub fn t(&self) -> f32 {
        self.t
    }

    pub fn material(&self) -> &PhongMaterial {
        &self.material
    }

    pub fn set_material(&mut self, material: PhongMaterial) {
        self.material = material;
    }

    pub fn point(&self) -> Vector3<f32> {
        self.ray.point(self.t)
    }

    pub fn normal(&self) -> Unit<Vector3<f32>> {
        self.normal
    }

    pub fn ray(&self) -> Ray {
        self.ray.clone()
    }

    pub fn apply_transforms(self, transform: &Transform) -> Self {
        let ray = self.ray.into_transformed_ray(transform);
        let point = (transform.as_mat4() * self.ray.point(self.t).push(1.0)).xyz();

        Self {
            t: ray.t(&point),
            material: self.material,
            ray,
            normal: transform_normal(self.normal, transform),
        }
    }
}
