use bytes::Bytes;
use obj::{IndexTuple, Obj, ObjData};
use std::{
    io::{BufReader, Cursor, Read},
    path::PathBuf,
};

use image::{io::Reader, DynamicImage};

pub struct AssetLoader;

#[cfg(target_arch = "wasm32")]
impl AssetLoader {
    pub async fn load_texture_image(path: String) -> DynamicImage {
        Self::load_image(format!("static/textures/{path}"))
    }

    pub async fn load_skybox_image(path: String) -> DynamicImage {
        Self::load_image(format!("static/skybox/{path}"))
    }

    pub async fn load_obj(path: String) -> ObjData {
        ObjData::load_buf(Self::load_buffer(format!("/static/meshes/{path}"))).unwrap()
    }

    fn load_buffer(url: String) -> Cursor<Bytes> {
        let response = pollster::block_on(reqwest::get(url)).unwrap();
        let bytes = pollster::block_on(response.bytes()).unwrap();
        Cursor::new(bytes)
    }

    fn load_image(relative_url: String) -> DynamicImage {
        let response = pollster::block_on(reqwest::get(format!(
            "http://localhost:8000/{relative_url}"
        )))
        .unwrap();
        let bytes = pollster::block_on(response.bytes()).unwrap();
        image::load_from_memory(&bytes).unwrap()
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl AssetLoader {
    pub async fn load_texture_image(path: String) -> DynamicImage {
        Self::load_image(format!(
            "/Users/Devin/Desktop/Github/DevinLeamy/eva/eva/assets/textures/{path}"
        )).await
    }

    pub async fn load_skybox_image(path: String) -> DynamicImage {
        Self::load_image(format!(
            "/Users/Devin/Desktop/Github/DevinLeamy/eva/eva/assets/skybox/{path}"
        )).await
    }

    pub async fn load_obj(path: String) -> ObjData {
        Obj::load(format!(
            "/Users/Devin/Desktop/Github/DevinLeamy/eva/eva-py/assets/meshes/{path}"
        ))
        .unwrap()
        .data
    }

    async fn load_image(path: String) -> DynamicImage {
        Reader::open(path).unwrap().decode().unwrap()
    }
}
