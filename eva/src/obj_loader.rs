use nalgebra::{Vector2, Vector3};
use obj::IndexTuple;
use std::{collections::HashMap, path::PathBuf, sync::Mutex};

use crate::{asset_loader::AssetLoader, obj_mesh::ObjMesh, shader::ShaderMeshVertex};

lazy_static! {
    static ref LOADED_MESHES: Mutex<HashMap<PathBuf, ObjMesh>> = Mutex::new(HashMap::new());
}

/// Loads wavefront .obj files.
pub struct ObjLoader;

impl ObjLoader {
    pub fn load<P: Into<PathBuf>>(path: P) -> Result<ObjMesh, String> {
        let path = path.into();
        let mut map = LOADED_MESHES.lock().unwrap();
        if map.contains_key(&path) {
            let value = map.get(&path).unwrap();
            return Ok(value.clone());
        }

        let mesh = Self::load_mesh(&path)?;
        map.insert(path, mesh.clone());

        Ok(mesh)
    }

    fn load_mesh(path: &PathBuf) -> Result<ObjMesh, String> {
        let obj_data = AssetLoader::load_obj(path);
        let mut vertices = Vec::<ShaderMeshVertex>::new();

        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();
        let mut triangles = Vec::new();

        for position in obj_data.position {
            positions.push(Vector3::new(position[0], position[1], position[2]));
        }
        for normal in obj_data.normal {
            normals.push(Vector3::new(normal[0], normal[1], normal[2]));
        }
        for uv in obj_data.texture {
            uvs.push(Vector2::new(uv[0], uv[1]));
        }

        for shape in obj_data.objects.iter().flat_map(|o| &o.groups[0].polys) {
            let base_vertex_index = vertices.len() as u32;

            for IndexTuple(v_position, _v_uv, v_normal) in &shape.0 {
                let mut normal_index = 0 as u32;

                if let Some(normal) = v_normal {
                    normal_index = *normal as u32;
                }

                vertices.push(ShaderMeshVertex {
                    position: *v_position as u32,
                    normal: normal_index,
                })
            }

            let vertices = shape.0.len();

            if vertices == 3 {
                triangles.push(Vector3::new(
                    base_vertex_index,
                    base_vertex_index + 1,
                    base_vertex_index + 2,
                ))
            } else if vertices == 4 {
                triangles.push(Vector3::new(
                    base_vertex_index,
                    base_vertex_index + 1,
                    base_vertex_index + 2,
                ));
                triangles.push(Vector3::new(
                    base_vertex_index + 1,
                    base_vertex_index + 2,
                    base_vertex_index + 3,
                ));
            } else {
                panic!("Invalid number of vertices {:?}", vertices);
            }
        }

        Ok(ObjMesh {
            triangles,
            vertices,
            positions,
            normals,
            uvs,
        })
    }
}
