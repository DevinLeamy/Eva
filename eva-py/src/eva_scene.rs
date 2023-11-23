use crate::prelude::*;

#[rustfmt::skip]
const TEXTURE_PATH: &'static str = "/Users/Devin/Desktop/Github/DevinLeamy/eva/eva/assets/textures";
const SKYBOX_PATH: &'static str = "/Users/Devin/Desktop/Github/DevinLeamy/eva/eva/assets/skybox";

#[pyclass]
#[pyo3(name = "EvaScene")]
pub struct EvaScene {
    pub root: Node,
    pub ambient: Vector3<f32>,
    pub texture_loader: TextureLoader,
    pub skybox: ShaderSkybox,
}

fn skybox_image_path(name: &str) -> PathBuf {
    PathBuf::from(format!("{SKYBOX_PATH}/{name}"))
}

#[pymethods]
impl EvaScene {
    #[new]
    fn new() -> Self {
        let mut texture_loader = TextureLoader::new();
        let path = PathBuf::from(format!("{TEXTURE_PATH}/missing.png"));
        texture_loader.load(path);

        Self {
            root: Node::Transformation(Transformation::new()),
            ambient: Vector3::new(0.1, 0.1, 0.1),
            texture_loader,
            skybox: ShaderSkybox::create_skybox(vec![
                skybox_image_path("filler.png"),
                skybox_image_path("filler.png"),
                skybox_image_path("filler.png"),
                skybox_image_path("filler.png"),
                skybox_image_path("filler.png"),
                skybox_image_path("filler.png"),
            ])
            .unwrap(),
        }
    }

    fn set_root(&mut self, py: Python, root: PyObject) {
        if let Ok(child) = root.extract::<PyRef<PyGeometry>>(py) {
            self.root = child.inner.clone().into();
        } else if let Ok(child) = root.extract::<PyRef<PyTransform>>(py) {
            self.root = child.inner.clone().into();
        } else if let Ok(child) = root.extract::<PyRef<EvaLight>>(py) {
            self.root = child.inner.clone().into();
        } else {
            panic!("add_child only accepts PyGeometry, PyTransform, or EvaLight");
        }
    }

    fn set_ambient(&mut self, r: f32, g: f32, b: f32) {
        self.ambient = Vector3::new(r, g, b);
    }

    fn add_texture(&mut self, texture_name: String) -> u32 {
        let mut path = PathBuf::from(TEXTURE_PATH);
        path.push(texture_name);
        self.texture_loader.load(path)
    }

    fn set_skybox(&mut self, faces: Vec<String>) {
        let paths: Vec<PathBuf> = faces.iter().map(|face| skybox_image_path(face)).collect();
        self.skybox = ShaderSkybox::create_skybox(paths).unwrap();
    }
}
