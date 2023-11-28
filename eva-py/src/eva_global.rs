use crate::prelude::*;
use std::path::PathBuf;

const DEFAULT_AMBIENT: Vector3<f32> = Vector3::new(0.3, 0.3, 0.3);

#[pyclass]
#[pyo3(name = "EvaGlobal")]
#[derive(Clone)]
pub struct EvaGlobal {
    pub texture_loader: TextureLoader,
    pub skybox: ShaderSkybox,
    pub ambient: Vector3<f32>,
    pub materials: ShaderBuffer<PbrMaterial>,
    pub sample_count: u32,
    pub max_reflections: u32,
}

#[pymethods]
impl EvaGlobal {
    #[new]
    fn new() -> Self {
        let mut texture_loader = TextureLoader::new();
        texture_loader.load("missing.png".into());

        Self {
            texture_loader,
            skybox: ShaderSkybox::create_skybox(vec![
                "filler.png".into(),
                "filler.png".into(),
                "filler.png".into(),
                "filler.png".into(),
                "filler.png".into(),
                "filler.png".into(),
            ])
            .unwrap(),
            ambient: DEFAULT_AMBIENT,
            materials: ShaderBuffer::new(),
            sample_count: 9,
            max_reflections: 10,
        }
    }

    fn add_texture(&mut self, name: String) -> u32 {
        self.texture_loader.load(name.into())
    }

    fn add_material(&mut self, material: PyRef<EvaMaterial>) -> u32 {
        self.materials.push(material.inner.clone())
    }

    fn add_skybox(&mut self, faces: Vec<String>) {
        let paths: Vec<PathBuf> = faces.iter().map(|face| face.into()).collect();
        self.skybox = ShaderSkybox::create_skybox(paths).unwrap();
    }

    fn set_ambient(&mut self, r: f32, g: f32, b: f32) {
        self.ambient = Vector3::new(r, g, b);
    }

    fn set_sample_count(&mut self, count: u32) {
        self.sample_count = count;
    }

    fn set_max_reflections(&mut self, max_reflections: u32) {
        self.max_reflections = max_reflections;
    }
}
