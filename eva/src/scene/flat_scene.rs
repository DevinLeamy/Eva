use nalgebra::Vector3;

use crate::{
    prelude::extents,
    shader::{
        ShaderBuffer, ShaderCubeModel, ShaderMeshModel, ShaderMeshModels, ShaderPointLight,
        ShaderSphereModel
    },
};

use super::{Cube, Geometry, Light, Node, Primitive, Scene, Transform};

#[derive(Debug)]
pub struct FlatScene {
    pub lights: ShaderBuffer<ShaderPointLight>,
    pub spheres: ShaderBuffer<ShaderSphereModel>,
    pub cubes: ShaderBuffer<ShaderCubeModel>,
    pub meshes: ShaderMeshModels,
}

impl std::fmt::Display for FlatScene {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for light in self.lights.iter() {
            writeln!(f, "Light: {:?}", light)?;
        }
        for geometry in self.spheres.iter() {
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
    lights: ShaderBuffer<ShaderPointLight>,
    spheres: ShaderBuffer<ShaderSphereModel>,
    cubes: ShaderBuffer<ShaderCubeModel>,
    meshes: ShaderMeshModels,
}

impl SceneFlattener {
    fn new() -> Self {
        Self {
            transforms: vec![Transform::default()],
            lights: ShaderBuffer::new(),
            spheres: ShaderBuffer::new(),
            cubes: ShaderBuffer::new(),
            meshes: ShaderMeshModels::new(),
        }
    }

    pub fn flatten(scene: Scene) -> FlatScene {
        let mut flattener = Self::new();
        flattener.traverse_scene(&scene);

        FlatScene {
            lights: flattener.lights,
            spheres: flattener.spheres,
            cubes: flattener.cubes,
            meshes: flattener.meshes,
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
        self.lights.push(ShaderPointLight {
            position: self.top_transform().translation(),
            colour: light.colour().clone(),
        });
    }

    fn handle_geometry(&mut self, geometry: &Geometry) {
        match geometry.primitive().clone() {
            Primitive::Cube(cube) => {
                self.cubes.push(ShaderCubeModel {
                    cube,
                    transform: self.top_transform().into(),
                    material: geometry.material().clone(),
                });
            }
            Primitive::Sphere(sphere) => {
                self.spheres.push(ShaderSphereModel {
                    sphere,
                    transform: self.top_transform().into(),
                    material: geometry.material().clone(),
                });
            }
            Primitive::Mesh(mesh) => {
                let (min, max) = extents(&mesh.positions);

                self.meshes.add(ShaderMeshModel {
                    material: geometry.material().clone(),
                    bounding_box: Cube::from_points(min, max),
                    transform: self.top_transform().into(),
                    triangles: mesh.triangles,
                    vertices: mesh.vertices,

                    positions: mesh.positions,
                    normals: mesh.normals,
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
