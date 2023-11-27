use eva::prelude::*;
use log::info;
use nalgebra::Vector3;

pub struct BallDemo {
    config: BallDemoConfig,
}

impl BallDemo {}

impl BallDemo {
    pub fn new() -> Self {
        Self {
            config: BallDemoConfig::new(),
        }
    }
}

impl DynamicScene for BallDemo {
    fn update(&mut self) {
        println!("UPDATE")
    }

    fn handle_input(&mut self, key: String, state: String) {
        info!("{key}");
        println!("INPUT RECEIVED")
    }

    fn dynamic_context(&self) -> DynamicRenderContext {
        DynamicRenderContext {
            camera: Camera::new(
                Vector3::new(0.0, 0.0, 800.0),
                50.0,
                Vector3::new(0.0, 0.0, -1.0),
                Vector3::new(0.0, 1.0, 0.0),
            ),
            scene: Scene {
                root: Node::Transformation(Transformation::default()),
            },
        }
    }
}

impl BallDemo {}

impl Into<RunDescriptor> for BallDemo {
    fn into(self) -> RunDescriptor {
        RunDescriptor {
            global: Box::new(self.config.clone()),
            render: RenderMode::Dynamic {
                scene: Box::new(self),
            },
        }
    }
}

#[derive(Clone)]
struct BallDemoConfig {
    pub texture_loader: TextureLoader,
    pub skybox: ShaderSkybox,
    pub ambient: Vector3<f32>,
    pub materials: ShaderBuffer<PbrMaterial>,
    pub sample_count: u32,
    pub max_reflections: u32,
}

impl BallDemoConfig {
    fn new() -> Self {
        let mut texture_loader = TextureLoader::new();
        texture_loader.load("missing.png".to_string());

        Self {
            texture_loader,
            skybox: ShaderSkybox::create_skybox(vec![
                "filler.png".to_string(),
                "filler.png".to_string(),
                "filler.png".to_string(),
                "filler.png".to_string(),
                "filler.png".to_string(),
                "filler.png".to_string(),
            ])
            .unwrap(),
            ambient: Vector3::zeros(),
            materials: ShaderBuffer::new(),
            sample_count: 9,
            max_reflections: 10,
        }
    }
}

impl GlobalConfig for BallDemoConfig {
    fn static_context(&self) -> StaticRenderContext {
        StaticRenderContext {
            skybox: self.skybox.clone(),
            ambient: self.ambient,
            textures: self.texture_loader.clone().textures(),
            materials: self.materials.clone(),
            sample_count: self.sample_count,
            max_reflections: self.max_reflections,
        }
    }
}
