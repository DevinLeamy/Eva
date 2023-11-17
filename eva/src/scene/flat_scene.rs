use nalgebra::Vector3;

use crate::shader::{
    ShaderCubeModel, ShaderCubeModels, ShaderPointLight, ShaderPointLights, ShaderSphereModel,
    ShaderSphereModels,
};

use super::{Geometry, Light, Node, Primitive, Scene, Sphere, Transform};

#[derive(Debug)]
pub struct FlatScene {
    pub lights: ShaderPointLights,
    pub spheres: ShaderSphereModels,
    pub cubes: ShaderCubeModels,
    pub ambient: Vector3<f32>,
}

impl std::fmt::Display for FlatScene {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for light in &self.lights.lights {
            writeln!(f, "Light: {:?}", light)?;
        }
        for geometry in &self.spheres.spheres {
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
    lights: ShaderPointLights,
    spheres: ShaderSphereModels,
    cubes: ShaderCubeModels,
}

impl SceneFlattener {
    fn new() -> Self {
        Self {
            transforms: vec![Transform::default()],
            lights: ShaderPointLights::new(),
            spheres: ShaderSphereModels::new(),
            cubes: ShaderCubeModels::new(),
        }
    }

    pub fn flatten(scene: Scene) -> FlatScene {
        let mut flattener = Self::new();
        flattener.traverse_scene(&scene);

        FlatScene {
            lights: flattener.lights,
            spheres: flattener.spheres,
            cubes: flattener.cubes,
            ambient: scene.ambient(),
        }
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
        self.lights.add(ShaderPointLight {
            position: self.top_transform().translation(),
            colour: light.colour().clone(),
        });
    }

    fn handle_geometry(&mut self, geometry: &Geometry) {
        match geometry.primitive().clone() {
            Primitive::Cube(cube) => {
                self.cubes.add(ShaderCubeModel {
                    cube,
                    transform: self.top_transform().into(),
                    material: geometry.material().clone(),
                });
            }
            Primitive::Sphere(sphere) => {
                self.spheres.add(ShaderSphereModel {
                    sphere,
                    transform: self.top_transform().into(),
                    material: geometry.material().clone(),
                });
            }
            Primitive::Mesh(_) => {
                // TODO: Add mesh geometries.
                self.spheres.add(ShaderSphereModel {
                    sphere: Sphere { radius: 1.0 },
                    transform: self.top_transform().into(),
                    material: geometry.material().clone(),
                });
            }
        }
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
