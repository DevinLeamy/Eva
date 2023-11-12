use nalgebra::{Vector2, Vector3};
use obj::{IndexTuple, Obj};
use std::{collections::HashMap, path::PathBuf, sync::Mutex};

use crate::obj_mesh::{ObjMesh, ObjTriangle};

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
        let obj: Obj = Obj::load(path).map_err(|e| e.to_string())?;
        let obj_data = obj.data;
        let mut triangles = Vec::new();

        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();

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
            assert!(shape.0.len() == 3);
            let mut vertices = Vec::new();
            let mut normals = Vec::new();
            let mut uvs = Vec::new();

            for IndexTuple(v_position, v_normal, v_uv) in &shape.0 {
                vertices.push(*v_position);
                if let Some(normal) = v_normal {
                    normals.push(*normal);
                }
                if let Some(uv) = v_uv {
                    uvs.push(*uv);
                }
            }

            triangles.push(ObjTriangle {
                vertices,
                normals,
                uvs,
            });
        }

        Ok(ObjMesh::new(positions, normals, uvs, triangles))
    }
}
