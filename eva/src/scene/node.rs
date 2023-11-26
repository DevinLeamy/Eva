use nalgebra::Vector3;

use super::{Primitive, Sphere, Transform};

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

    pub fn transform_mut(&mut self) -> &mut Transform {
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
    pub material_id: u32,
    primitive: Primitive,
}

impl Default for Geometry {
    fn default() -> Self {
        Self {
            transform: Transform::default(),
            children: Vec::new(),
            material_id: 0,
            primitive: Primitive::Sphere(Sphere::new(1.0)),
        }
    }
}

impl Geometry {
    pub fn new(
        transform: Transform,
        material_id: u32,
        primitive: Primitive,
        children: Vec<Node>,
    ) -> Self {
        Self {
            transform,
            children,
            material_id,
            primitive,
        }
    }

    pub fn from_primitive(primitive: Primitive) -> Self {
        Self {
            transform: Transform::default(),
            children: Vec::new(),
            material_id: 0,
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

    pub fn set_material(&mut self, material_id: u32) {
        self.material_id = material_id;
    }

    pub fn primitive(&self) -> &Primitive {
        &self.primitive
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
    pub fn new(
        transform: Transform,
        colour: Vector3<f32>,
        attenuation: Vector3<f32>,
        children: Vec<Node>,
    ) -> Self {
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
