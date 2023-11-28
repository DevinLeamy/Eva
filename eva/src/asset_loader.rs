use obj::{Obj, ObjData};
use std::path::PathBuf;

use image::{io::Reader, DynamicImage};

pub struct AssetLoader;

impl AssetLoader {
    pub fn load_texture_image(extension: &PathBuf) -> DynamicImage {
        let mut path =
            PathBuf::from("/Users/Devin/Desktop/Github/DevinLeamy/eva/eva/assets/textures/");
        path.push(extension);

        Self::load_image(path)
    }

    pub fn load_skybox_image(extension: &PathBuf) -> DynamicImage {
        let mut path =
            PathBuf::from("/Users/Devin/Desktop/Github/DevinLeamy/eva/eva/assets/skybox/");
        path.push(extension);

        Self::load_image(path)
    }

    pub fn load_obj(extension: &PathBuf) -> ObjData {
    let mut path =
            PathBuf::from("/Users/Devin/Desktop/Github/DevinLeamy/eva/eva-py/assets/meshes/");
        path.push(extension);

        Obj::load(path).unwrap().data
    }

    fn load_image(path: PathBuf) -> DynamicImage {
        Reader::open(path).unwrap().decode().unwrap()
    }
}
