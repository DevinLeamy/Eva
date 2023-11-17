use std::path::PathBuf;

use nalgebra::Vector3;

use crate::obj_loader::ObjLoader;

#[derive(Clone, Debug)]
pub struct Mesh {
    pub points: Vec<Vector3<f32>>,
    pub triangles: Vec<Vector3<u32>>,
}

impl Mesh {
    pub fn from_path<P: Into<PathBuf>>(path: P) -> Self {
        let obj_mesh = ObjLoader::load(path).unwrap();
        let mut triangles = Vec::new();

        for triangle in obj_mesh.triangles {
            let position_indices = &triangle.vertices;
            triangles.push(Vector3::<u32>::new(
                position_indices[0] as u32,
                position_indices[1] as u32,
                position_indices[2] as u32,
            ));
        }

        Self {
            points: obj_mesh.positions,
            triangles,
        }
    }
}
