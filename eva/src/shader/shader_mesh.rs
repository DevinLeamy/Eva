use encase::{ArrayLength, ShaderType};
use nalgebra::Vector3;

use crate::prelude::{Cube, PhongMaterial};

use super::{ShaderStruct, ShaderTransform};

#[derive(Debug, Default)]
pub struct ShaderMeshModels {
    pub headers: ShaderMeshModelHeaders,
    pub triangles: ShaderMeshTriangles,
    pub vertices: ShaderMeshVertices,
    pub positions: ShaderMeshPoints,
    pub normals: ShaderMeshNormals,
}

pub struct ShaderMeshModel {
    pub transform: ShaderTransform,
    pub material: PhongMaterial,
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
        let vertex_offset = self.vertices.extend(mesh.vertices.into_iter());
        let normal_offset = self.normals.extend(mesh.normals.iter());
        let position_offset = self.positions.add_points(mesh.positions.iter());
        let triangle_offset = self.triangles.add_triangles(mesh.triangles.iter());
        self.headers.headers.push(ShaderMeshModelHeader {
            material: mesh.material,
            transform: mesh.transform,
            bounding_box: mesh.bounding_box,

            vertex_offset,

            triangle_offset,
            triangle_count: mesh.triangles.len() as u32,

            position_offset,
            normal_offset,
        })
    }
}

#[derive(ShaderType, Debug, Default)]
pub struct ShaderMeshVertices {
    length: ArrayLength,
    #[size(runtime)]
    vertices: Vec<ShaderMeshVertex>,
}

impl ShaderMeshVertices {
    pub fn extend<'a>(&mut self, vertices: impl IntoIterator<Item = ShaderMeshVertex>) -> u32 {
        let offset = self.vertices.len();
        self.vertices.extend(vertices);

        offset as u32
    }
}

impl ShaderStruct for ShaderMeshVertices {
    fn as_bytes(&self) -> Option<Vec<u8>> {
        let mut buffer = encase::StorageBuffer::new(Vec::new());
        buffer.write(self).ok()?;
        Some(buffer.into_inner())
    }
}

#[derive(ShaderType, Debug, Default, Clone)]
pub struct ShaderMeshVertex {
    pub position: u32,
    pub normal: u32,
}

#[derive(ShaderType, Debug, Default)]
pub struct ShaderMeshNormals {
    length: ArrayLength,
    #[size(runtime)]
    normals: Vec<Vector3<f32>>,
}

impl ShaderMeshNormals {
    pub fn extend<'a>(&mut self, normals: impl IntoIterator<Item = &'a Vector3<f32>>) -> u32 {
        let offset = self.normals.len();
        self.normals.extend(normals);

        offset as u32
    }
}

impl ShaderStruct for ShaderMeshNormals {
    fn as_bytes(&self) -> Option<Vec<u8>> {
        let mut buffer = encase::StorageBuffer::new(Vec::new());
        buffer.write(self).ok()?;
        Some(buffer.into_inner())
    }
}

#[derive(ShaderType, Debug, Default)]
pub struct ShaderMeshPoints {
    length: ArrayLength,
    #[size(runtime)]
    points: Vec<Vector3<f32>>,
}

impl ShaderMeshPoints {
    pub fn add_points<'a>(&mut self, points: impl IntoIterator<Item = &'a Vector3<f32>>) -> u32 {
        let offset = self.points.len();
        self.points.extend(points);

        offset as u32
    }
}

impl ShaderStruct for ShaderMeshPoints {
    fn as_bytes(&self) -> Option<Vec<u8>> {
        let mut buffer = encase::StorageBuffer::new(Vec::new());
        buffer.write(self).ok()?;
        Some(buffer.into_inner())
    }
}

#[derive(ShaderType, Debug, Default)]
pub struct ShaderMeshTriangles {
    length: ArrayLength,
    #[size(runtime)]
    triangles: Vec<Vector3<u32>>,
}

impl ShaderMeshTriangles {
    pub fn add_triangles<'a>(
        &mut self,
        triangles: impl IntoIterator<Item = &'a Vector3<u32>>,
    ) -> u32 {
        let offset = self.triangles.len();
        self.triangles.extend(triangles);

        offset as u32
    }
}

impl ShaderStruct for ShaderMeshTriangles {
    fn as_bytes(&self) -> Option<Vec<u8>> {
        let mut buffer = encase::StorageBuffer::new(Vec::new());
        buffer.write(self).ok()?;
        Some(buffer.into_inner())
    }
}

#[derive(ShaderType, Debug, Default)]
pub struct ShaderMeshModelHeaders {
    length: ArrayLength,
    #[size(runtime)]
    headers: Vec<ShaderMeshModelHeader>,
}

impl ShaderStruct for ShaderMeshModelHeaders {
    fn as_bytes(&self) -> Option<Vec<u8>> {
        let mut buffer = encase::StorageBuffer::new(Vec::new());
        buffer.write(self).ok()?;
        Some(buffer.into_inner())
    }
}

#[derive(ShaderType, Debug, Default)]
pub struct ShaderMeshModelHeader {
    material: PhongMaterial,
    transform: ShaderTransform,
    bounding_box: Cube,

    vertex_offset: u32,
    triangle_offset: u32,
    triangle_count: u32,

    position_offset: u32,
    normal_offset: u32,
}
