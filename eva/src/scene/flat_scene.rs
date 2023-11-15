use nalgebra::Vector3;

use super::{Geometry, Light, Node, Scene, Transform};

#[derive(Debug)]
pub struct FlatScene {
    lights: Vec<Light>,
    geometry: Vec<Geometry>,
    ambient: Vector3<f32>,
}

impl FlatScene {
    pub fn new(lights: Vec<Light>, geometry: Vec<Geometry>, ambient: Vector3<f32>) -> Self {
        Self {
            lights,
            geometry,
            ambient,
        }
    }
}

impl FlatScene {
    pub fn geometry(&self) -> &Vec<Geometry> {
        &self.geometry
    }

    pub fn lights(&self) -> &Vec<Light> {
        &self.lights
    }

    pub fn ambient(&self) -> &Vector3<f32> {
        &self.ambient
    }
}

impl std::fmt::Display for FlatScene {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for light in &self.lights {
            writeln!(f, "Light: {:?}", light)?;
        }
        for geometry in &self.geometry {
            writeln!(f, "Object: {:?}", geometry)?;
        }

        Ok(())
    }
}

impl From<Scene> for FlatScene {
    fn from(scene: Scene) -> Self {
        SceneFlattener::flatten(scene)
    }
}

struct SceneFlattener {
    transforms: Vec<Transform>,
    lights: Vec<Light>,
    geometry: Vec<Geometry>,
}

impl SceneFlattener {
    fn new() -> Self {
        Self {
            transforms: vec![Transform::default()],
            lights: Vec::new(),
            geometry: Vec::new(),
        }
    }

    pub fn flatten(scene: Scene) -> FlatScene {
        let mut flattener = Self::new();
        flattener.traverse_scene(&scene);

        FlatScene::new(flattener.lights, flattener.geometry, scene.ambient())
    }
}

impl SceneFlattener {
    fn traverse_scene(&mut self, scene: &Scene) {
        self.traverse_node(scene.root());
    }

    fn traverse_node(&mut self, node: &Node) {
        self.push_transform(node.transform());
        match node {
            Node::Light(light) => self.handle_light(light),
            Node::Geometry(geometry) => self.handle_geometry(geometry),
            Node::Transformation(_) => (),
        }
        for child in node.children() {
            self.traverse_node(child);
        }

        self.pop_transform();
    }

    fn handle_light(&mut self, light: &Light) {
        let new_light = Light::new(
            self.top_transform(),
            light.colour().clone(),
            light.attenuation().clone(),
            Vec::new(),
        );
        self.lights.push(new_light);
    }

    fn handle_geometry(&mut self, geometry: &Geometry) {
        let new_geometry = Geometry::new(
            self.top_transform(),
            geometry.material().clone(),
            geometry.primitive().clone(),
            Vec::new(),
        );
        self.geometry.push(new_geometry);
    }

    fn push_transform(&mut self, transform: &Transform) {
        self.transforms
            .push(self.top_transform().transform(transform));
    }

    fn pop_transform(&mut self) {
        self.transforms.pop();
    }

    fn top_transform(&self) -> Transform {
        self.transforms
            .last()
            .expect("transform stack was empty")
            .clone()
    }
}
