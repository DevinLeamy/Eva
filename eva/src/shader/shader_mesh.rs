use encase::ShaderType;
use nalgebra::Vector3;

use crate::prelude::Cube;

use super::{IntoShaderBool, ShaderBool, ShaderBuffer, ShaderTransform};

#[derive(Debug, Default)]
pub struct ShaderMeshModels {
    pub headers: ShaderBuffer<ShaderMeshModelHeader>,
    pub triangles: ShaderBuffer<Vector3<u32>>,
    pub vertices: ShaderBuffer<ShaderMeshVertex>,
    pub positions: ShaderBuffer<Vector3<f32>>,
    pub normals: ShaderBuffer<Vector3<f32>>,
}

pub struct ShaderMeshModel {
    pub transform: ShaderTransform,
    pub material_id: u32,
    pub bounding_box: Cube,
    pub positions: Vec<Vector3<f32>>,
    pub normals: Vec<Vector3<f32>>,
    pub triangles: Vec<Vector3<u32>>,
    pub vertices: Vec<ShaderMeshVertex>,
}

impl ShaderMeshModels {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, mesh: ShaderMeshModel) {
        let triangle_count = mesh.triangles.len() as u32;
        let has_normals = (mesh.normals.len() > 0).shader_bool();
        let vertex_offset = self.vertices.extend(mesh.vertices);
        let normal_offset = self.normals.extend(mesh.normals);
        let position_offset = self.positions.extend(mesh.positions);
        let triangle_offset = self.triangles.extend(mesh.triangles);

        self.headers.push(ShaderMeshModelHeader {
            material_id: mesh.material_id,
            transform: mesh.transform,
            bounding_box: mesh.bounding_box,

            vertex_offset,

            triangle_offset,
            triangle_count,

            position_offset,
            normal_offset,

            has_normals,
        });
    }
}

#[derive(ShaderType, Debug, Default, Clone)]
pub struct ShaderMeshVertex {
    pub position: u32,
    pub normal: u32,
}

#[derive(ShaderType, Debug, Default)]
pub struct ShaderMeshModelHeader {
    material_id: u32,
    transform: ShaderTransform,
    bounding_box: Cube,

    vertex_offset: u32,
    triangle_offset: u32,
    triangle_count: u32,

    position_offset: u32,
    normal_offset: u32,

    has_normals: ShaderBool,
}
