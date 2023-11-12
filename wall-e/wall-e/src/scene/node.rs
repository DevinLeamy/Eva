use nalgebra::Vector3;

use crate::prelude::{Intersection, Ray};

use super::{Collidable, PhongMaterial, Sphere, Transform};

#[derive(Clone, Debug)]
pub enum Node {
    Light(Light),
    Geometry(Geometry),
    Transformation(Transformation),
}

impl Node {
    pub fn add_child(&mut self, child: Node) {
        self.children_mut().push(child);
    }

    pub fn rotate(&mut self, _v: Vector3<f32>) {
        todo!()
    }

    pub fn translate(&mut self, v: Vector3<f32>) {
        self.transform_mut().translate(v)
    }

    pub fn scale_nonuniform(&mut self, v: Vector3<f32>) {
        self.transform_mut().scale_nonuniform(v)
    }
}

impl Node {
    pub fn transform(&self) -> &Transform {
        match self {
            Node::Light(light) => light.transform(),
            Node::Geometry(geometry) => geometry.transform(),
            Node::Transformation(transformation) => transformation.transform(),
        }
    }

    pub fn children(&self) -> &Vec<Node> {
        match self {
            Node::Light(light) => &light.children,
            Node::Geometry(geometry) => &geometry.children,
            Node::Transformation(transformation) => &transformation.children,
        }
    }

    fn transform_mut(&mut self) -> &mut Transform {
        match self {
            Node::Light(light) => light.transform_mut(),
            Node::Geometry(geometry) => geometry.transform_mut(),
            Node::Transformation(transformation) => transformation.transform_mut(),
        }
    }

    fn children_mut(&mut self) -> &mut Vec<Node> {
        match self {
            Node::Light(light) => &mut light.children,
            Node::Geometry(geometry) => &mut geometry.children,
            Node::Transformation(transformation) => &mut transformation.children,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Geometry {
    transform: Transform,
    children: Vec<Node>,
    material: PhongMaterial,
    primitive: Box<dyn Collidable>,
}

impl Default for Geometry {
    fn default() -> Self {
        Self {
            transform: Transform::default(),
            children: Vec::new(),
            material: PhongMaterial::default(),
            primitive: Box::new(Sphere::new(1.0)),
        }
    }
}

impl Geometry {
    pub fn new(
        transform: Transform,
        material: PhongMaterial,
        primitive: Box<dyn Collidable>,
        children: Vec<Node>,
    ) -> Self {
        Self {
            transform,
            children,
            material,
            primitive,
        }
    }

    pub fn from_collidable(primitive: Box<dyn Collidable>) -> Self {
        Self {
            transform: Transform::default(),
            children: Vec::new(),
            material: PhongMaterial::default(),
            primitive,
        }
    }

    pub fn transform(&self) -> &Transform {
        &self.transform
    }

    pub fn children(&self) -> &Vec<Node> {
        &self.children
    }

    pub fn transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }

    pub fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }

    pub fn set_material(&mut self, material: PhongMaterial) {
        self.material = material;
    }

    pub fn primitive(&self) -> &Box<dyn Collidable> {
        &self.primitive
    }

    pub fn material(&self) -> &PhongMaterial {
        &self.material
    }
}

impl Collidable for Geometry {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let incoming = ray.into_inverse_transform_ray(&self.transform);
        let Some(mut intersection) = self.primitive.intersect(&incoming) else {
            return None;
        };

        intersection.set_material(self.material.clone());
        Some(intersection.apply_transforms(&self.transform))
    }
}

impl From<Geometry> for Node {
    fn from(val: Geometry) -> Self {
        Node::Geometry(val)
    }
}

#[derive(Clone, Debug, Default)]
pub struct Light {
    transform: Transform,
    colour: Vector3<f32>,
    /// Parameters for quadratic attenuation.
    attenuation: Vector3<f32>,
    children: Vec<Node>,
}

impl Light {
    pub fn new(transform: Transform, colour: Vector3<f32>, attenuation: Vector3<f32>, children: Vec<Node>) -> Self {
        Self {
            transform,
            colour,
            attenuation,
            children,
        }
    }

    pub fn transform(&self) -> &Transform {
        &self.transform
    }

    pub fn children(&self) -> &Vec<Node> {
        &self.children
    }

    pub fn transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }

    pub fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }

    pub fn colour(&self) -> &Vector3<f32> {
        &self.colour
    }

    pub fn attenuation(&self) -> &Vector3<f32> {
        &self.attenuation
    }
}

impl From<Light> for Node {
    fn from(val: Light) -> Self {
        Node::Light(val)
    }
}

#[derive(Clone, Debug)]
pub struct Transformation {
    transform: Transform,
    children: Vec<Node>,
}

impl Default for Transformation {
    fn default() -> Self {
        Self::new()
    }
}

impl Transformation {
    pub fn new() -> Self {
        Self {
            transform: Transform::default(),
            children: Vec::new(),
        }
    }

    pub fn transform(&self) -> &Transform {
        &self.transform
    }

    pub fn children(&self) -> &Vec<Node> {
        &self.children
    }

    pub fn transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }

    pub fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }
}

impl From<Transformation> for Node {
    fn from(val: Transformation) -> Self {
        Node::Transformation(val)
    }
}
