use std::path::PathBuf;

use nalgebra::Vector3;

use crate::{obj_loader::ObjLoader, shader::ShaderMeshVertex};

#[derive(Clone, Debug)]
pub struct Mesh {
    pub triangles: Vec<Vector3<u32>>,
    pub vertices: Vec<ShaderMeshVertex>,
    pub positions: Vec<Vector3<f32>>,
    pub normals: Vec<Vector3<f32>>,
}

impl Mesh {
    pub fn from_path<P: Into<PathBuf>>(path: P) -> Self {
        let obj_mesh = ObjLoader::load(path).unwrap();

        Self {
            triangles: obj_mesh.triangles,
            positions: obj_mesh.positions,
            normals: obj_mesh.normals,
            vertices: obj_mesh.vertices,
        }
    }
}
