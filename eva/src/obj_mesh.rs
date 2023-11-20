use nalgebra::{Vector2, Vector3};

use crate::shader::ShaderMeshVertex;

#[derive(Clone, Debug)]
pub struct ObjMesh {
    pub positions: Vec<Vector3<f32>>,
    pub normals: Vec<Vector3<f32>>,
    pub uvs: Vec<Vector2<f32>>,
    pub triangles: Vec<Vector3<u32>>,
    pub vertices: Vec<ShaderMeshVertex>,
}
