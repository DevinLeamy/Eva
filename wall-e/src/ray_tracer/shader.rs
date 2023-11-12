use nalgebra::{Unit, Vector3};

use crate::{prelude::PhongMaterial, utils::vector_mul};

pub fn phong_illumination(
    // Camera location.
    camera: Vector3<f32>,
    // Intersection point.
    intersection: Vector3<f32>,
    // Light position.
    light: Vector3<f32>,
    // Surface normal at the point of intersection.
    normal: Unit<Vector3<f32>>,
    // Surface properties.
    material: &PhongMaterial,
    // Ambient intensity.
    ambient: &Vector3<f32>,
    light_colour: &Vector3<f32>,
    light_attenuation: &Vector3<f32>,
    in_shadow: bool,
) -> Vector3<f32> {
    let ambient = vector_mul(&vector_mul(light_colour, ambient), material.diffuse());
    if in_shadow {
        return ambient;
    }

    let to_light = (light - intersection).normalize();
    let to_view = (camera - intersection).normalize();

    let distance = (light - intersection).magnitude();
    let attenuation = 1.0
        / (light_attenuation.x
            + distance * light_attenuation.y
            + distance * distance * light_attenuation.z);


    let diffuse_strength = normal.dot(&to_light).max(0.0);
    let diffuse = attenuation * vector_mul(&(diffuse_strength * light_colour), material.diffuse());

    let half_vector = (to_view + to_light).normalize();
    let specular_strength = normal.dot(&half_vector).max(0.0).powf(material.shininess());
    let specular =
        attenuation * vector_mul(&(light_colour * specular_strength), material.specular());

    ambient + diffuse + specular
}
