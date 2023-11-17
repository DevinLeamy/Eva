use encase::{ArrayLength, ShaderType};
use nalgebra::Vector3;

use crate::prelude::{Cube, PhongMaterial};

use super::{ShaderStruct, ShaderTransform};

#[derive(Debug, Default)]
pub struct ShaderMeshModels {
    pub points: ShaderMeshPoints,
    pub triangles: ShaderMeshTriangles,
    pub headers: ShaderMeshModelHeaders,
}

pub struct ShaderMeshModel {
    pub points: Vec<Vector3<f32>>,
    pub triangles: Vec<Vector3<u32>>,
    pub transform: ShaderTransform,
    pub material: PhongMaterial,
    pub bounding_box: Cube,
}

impl ShaderMeshModels {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, mesh: ShaderMeshModel) {
        let vertex_offset = self.points.add_points(mesh.points.iter());
        let triangle_offset = self.triangles.add_triangles(mesh.triangles.iter());
        self.headers.headers.push(ShaderMeshModelHeader {
            vertex_offset,
            vertex_count: mesh.points.len() as u32,
            triangle_offset,
            triangle_count: mesh.triangles.len() as u32,
            material: mesh.material,
            transform: mesh.transform,
            bounding_box: mesh.bounding_box,
        })
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
    vertex_offset: u32,
    vertex_count: u32,
    triangle_offset: u32,
    triangle_count: u32,
    material: PhongMaterial,
    transform: ShaderTransform,
    bounding_box: Cube,
}
