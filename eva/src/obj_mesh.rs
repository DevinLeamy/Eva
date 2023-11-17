use nalgebra::{Vector2, Vector3};

#[derive(Clone, Debug)]
pub struct ObjTriangle {
    pub vertices: Vec<usize>,
    pub normals: Vec<usize>,
    pub uvs: Vec<usize>,
}

#[derive(Clone, Debug)]
pub struct ObjMesh {
    pub positions: Vec<Vector3<f32>>,
    pub normals: Vec<Vector3<f32>>,
    pub uvs: Vec<Vector2<f32>>,
    pub triangles: Vec<ObjTriangle>,
}

impl ObjMesh {
    pub fn new(
        positions: Vec<Vector3<f32>>,
        normals: Vec<Vector3<f32>>,
        uvs: Vec<Vector2<f32>>,
        triangles: Vec<ObjTriangle>,
    ) -> Self {
        Self {
            positions,
            normals,
            uvs,
            triangles,
        }
    }

    pub fn normals(&self) -> &Vec<Vector3<f32>> {
        &self.normals
    }

    pub fn uvs(&self) -> &Vec<Vector2<f32>> {
        &self.uvs
    }
}
