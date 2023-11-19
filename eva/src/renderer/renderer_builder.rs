use std::num::NonZeroU32;

use pollster::FutureExt;
use wgpu::*;
use winit::window::Window;

use crate::{Renderer, shader::{ShaderSphereModel, ShaderStruct, ShaderCamera, ShaderPointLight, ShaderGlobalConfig, ShaderCubeModel}};

use super::RenderContext;

const SPHERE_COUNT: u64 = 100;
const CUBE_COUNT: u64 = 25;
const LIGHT_COUNT: u64 = 5;
const MESH_POINT_BUFFER_SIZE: u64 = 400_000;
const MESH_TRIANGLE_BUFFER_SIZE: u64 = 400_000;
const MESH_HEADERS_BUFFER_SIZE: u64 = 5_000;

const TEXTURE_2D_COUNT: u32 = 1;

pub struct RendererBuilder {
    surface: Surface,
    device: Device,
    queue: Queue,
    window: Window,
    adapter: Adapter,
    context: RenderContext,

    ray_tracer_shader: Option<ShaderModule>,
    display_shader: Option<ShaderModule>,

    ray_tracer_pipeline: Option<ComputePipeline>,
    ray_tracer_bind_group_layout: Option<BindGroupLayout>,
    mesh_bind_group_layout: Option<BindGroupLayout>,
    texture_bind_group_layout: Option<BindGroupLayout>,

    display_pipeline: Option<RenderPipeline>,
    display_bind_group_layout: Option<BindGroupLayout>,

    mesh_points_buffer: Option<Buffer>,
    mesh_triangles_buffer: Option<Buffer>,
    mesh_headers_buffer: Option<Buffer>,

    camera_buffer: Option<Buffer>,
    spheres_buffer: Option<Buffer>,
    cubes_buffer: Option<Buffer>,
    lights_buffer: Option<Buffer>,
    config_buffer: Option<Buffer>,
}

impl RendererBuilder {
    pub fn new(window: Window, context: RenderContext) -> Self {
        let size = window.inner_size();
        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::all(),
            ..Default::default()
        });

        // Must live as long as the window that created it.
        let surface = unsafe { instance.create_surface(&window) }.unwrap();
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::None,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .block_on()
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    features: Features::SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING | Features::TEXTURE_BINDING_ARRAY,
                    limits: Limits::default(),
                    label: None,
                },
                None,
            )
            .block_on()
            .unwrap();

        let surface_capabilities = surface.get_capabilities(&adapter);
        println!("Present modes: {:?}", surface_capabilities);
        let surface_config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: TextureFormat::Rgba16Float,
            width: size.width,
            height: size.height,
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &surface_config);

        Self {
            device,
            queue,
            window,
            surface,
            adapter,
            context,

            ray_tracer_shader: None,
            display_shader: None,

            ray_tracer_pipeline: None,
            display_pipeline: None,

            ray_tracer_bind_group_layout: None,
            mesh_bind_group_layout: None,
            texture_bind_group_layout: None,
            display_bind_group_layout: None,

            mesh_points_buffer: None,
            mesh_triangles_buffer: None,
            mesh_headers_buffer: None,

            camera_buffer: None,
            config_buffer: None,
            spheres_buffer: None,
            cubes_buffer: None,
            lights_buffer: None,
        }
    }

    pub fn build(mut self) -> Renderer {
        self.create_assets();
        self.create_bind_group_layouts();
        self.create_bind_groups();
        self.create_pipelines();

        Renderer {
            surface: self.surface,
            device: self.device,
            queue: self.queue,
            window: self.window,
            context: self.context,

            ray_tracer_pipeline: self.ray_tracer_pipeline.unwrap(),
            display_pipeline: self.display_pipeline.unwrap(),

            ray_tracer_bind_group_layout: self.ray_tracer_bind_group_layout.unwrap(),
            mesh_bind_group_layout: self.mesh_bind_group_layout.unwrap(),
            texture_bind_group_layout: self.texture_bind_group_layout.unwrap(),
            display_bind_group_layout: self.display_bind_group_layout.unwrap(),

            mesh_points_buffer: self.mesh_points_buffer.unwrap(),
            mesh_triangles_buffer: self.mesh_triangles_buffer.unwrap(),
            mesh_headers_buffer: self.mesh_headers_buffer.unwrap(),

            camera_buffer: self.camera_buffer.unwrap(),
            config_buffer: self.config_buffer.unwrap(),
            cubes_buffer: self.cubes_buffer.unwrap(),
            spheres_buffer: self.spheres_buffer.unwrap(),
            lights_buffer: self.lights_buffer.unwrap(),
        }
    }
}

impl RendererBuilder {
    #[rustfmt::skip]
    fn create_assets(&mut self) {
        // Shaders.
        self.ray_tracer_shader = Some(self.device.create_shader_module(include_wgsl!("../../assets/shaders/ray_tracer.wgsl")));
        self.display_shader = Some(self.device.create_shader_module(include_wgsl!("../../assets/shaders/display.wgsl")));

        self.camera_buffer = Some(self.device.create_buffer(&BufferDescriptor { 
            label: Some("camera buffer"), 
            size: ShaderCamera::size(),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST, 
            mapped_at_creation: false 
        }));

        self.config_buffer = Some(self.device.create_buffer(&BufferDescriptor { 
            label: None,
            size: ShaderGlobalConfig::size(),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST, 
            mapped_at_creation: false 
        }));

        self.spheres_buffer = Some(self.device.create_buffer(&BufferDescriptor {
            label: Some("spheres buffer"),
            // Must be larger than the size of any data used.
            size: ShaderSphereModel::size() * SPHERE_COUNT, 
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
            mapped_at_creation: false
        }));

        self.cubes_buffer = Some(self.device.create_buffer(&BufferDescriptor {
            label: None,
            // Must be larger than the size of any data used.
            size: ShaderCubeModel::size() * CUBE_COUNT, 
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
            mapped_at_creation: false
        }));

        self.lights_buffer = Some(self.device.create_buffer(&BufferDescriptor { 
            label: Some("lights buffer"), 
            size: ShaderPointLight::size() * LIGHT_COUNT, 
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST, 
            mapped_at_creation: false
        }));

        self.mesh_points_buffer = Some(self.device.create_buffer(&BufferDescriptor { 
            label: None,
            size: MESH_POINT_BUFFER_SIZE, 
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST, 
            mapped_at_creation: false
        }));

        self.mesh_triangles_buffer = Some(self.device.create_buffer(&BufferDescriptor { 
            label: None,
            size: MESH_TRIANGLE_BUFFER_SIZE, 
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST, 
            mapped_at_creation: false
        }));

        self.mesh_headers_buffer = Some(self.device.create_buffer(&BufferDescriptor { 
            label: None,
            size: MESH_HEADERS_BUFFER_SIZE, 
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST, 
            mapped_at_creation: false
        }));
    }

    fn create_bind_group_layouts(&mut self) {
        self.texture_bind_group_layout = Some(self.device.create_bind_group_layout(
            &BindGroupLayoutDescriptor {
                label: None,
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: NonZeroU32::new(TEXTURE_2D_COUNT),
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: NonZeroU32::new(TEXTURE_2D_COUNT),
                    }
                ]
            }
        ));

        self.mesh_bind_group_layout = Some(self.device.create_bind_group_layout(
            &BindGroupLayoutDescriptor { 
                label: None, 
                entries: &[
                    BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::COMPUTE,
                        ty: BindingType::Buffer {
                            ty: BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None
                    },
                    BindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStages::COMPUTE,
                        ty: BindingType::Buffer {
                            ty: BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None
                    },
                    BindGroupLayoutEntry {
                        binding: 2,
                        visibility: ShaderStages::COMPUTE,
                        ty: BindingType::Buffer {
                            ty: BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None
                    }
                ]
            }
        ));

        self.ray_tracer_bind_group_layout = Some(self.device.create_bind_group_layout(
            &BindGroupLayoutDescriptor {
                label: Some("ray tracer bind group layout"),
                entries: &[
                    BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::COMPUTE,
                        ty: BindingType::StorageTexture {
                            access: StorageTextureAccess::WriteOnly,
                            format: TextureFormat::Rgba16Float,
                            view_dimension: TextureViewDimension::D2,
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStages::COMPUTE,
                        ty: BindingType::Buffer {
                            ty: BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 2,
                        visibility: ShaderStages::COMPUTE,
                        ty: BindingType::Buffer { 
                            ty: BufferBindingType::Storage { read_only: true }, 
                            has_dynamic_offset: false, 
                            min_binding_size: None 
                        },
                        count: None
                    },
                    BindGroupLayoutEntry {
                        binding: 3,
                        visibility: ShaderStages::COMPUTE,
                        ty: BindingType::Buffer { 
                            ty: BufferBindingType::Storage { read_only: true }, 
                            has_dynamic_offset: false, 
                            min_binding_size: None 
                        },
                        count: None
                    },
                    BindGroupLayoutEntry {
                        binding: 4,
                        visibility: ShaderStages::COMPUTE,
                        ty: BindingType::Buffer { 
                            ty: BufferBindingType::Uniform, 
                            has_dynamic_offset: false, 
                            min_binding_size: None 
                        },
                        count: None
                    },
                    BindGroupLayoutEntry {
                        binding: 5,
                        visibility: ShaderStages::COMPUTE,
                        ty: BindingType::Buffer { 
                            ty: BufferBindingType::Storage { read_only: true }, 
                            has_dynamic_offset: false, 
                            min_binding_size: None 
                        },
                        count: None
                    }
                ],
            },
        ));

        self.display_bind_group_layout = Some(self.device.create_bind_group_layout(
            &BindGroupLayoutDescriptor {
                label: Some("display bind group layout"),
                entries: &[
                    BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Sampler(SamplerBindingType::NonFiltering),
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Texture {
                            sample_type: TextureSampleType::Float { filterable: false },
                            view_dimension: TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                ],
            },
        ));
    }

    fn create_bind_groups(&mut self) {}

    fn create_pipelines(&mut self) {
        self.create_render_pipeline();
        self.create_display_pipeline();
    }

    fn create_render_pipeline(&mut self) {
        let layout = self
            .device
            .create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("ray tracer pipeline layout"),
                bind_group_layouts: &[
                    self.ray_tracer_bind_group_layout.as_ref().unwrap(), 
                    self.mesh_bind_group_layout.as_ref().unwrap(),
                    self.texture_bind_group_layout.as_ref().unwrap()
                ],
                push_constant_ranges: &[],
            });

        self.ray_tracer_pipeline = Some(self.device.create_compute_pipeline(
            &ComputePipelineDescriptor {
                label: Some("ray tracer pipeline"),
                layout: Some(&layout),
                module: self.ray_tracer_shader.as_ref().unwrap(),
                entry_point: "compute_main",
            },
        ));
    }

    fn create_display_pipeline(&mut self) {
        let size = self.window.inner_size();
        let surface_capabilities = self.surface.get_capabilities(&self.adapter);

        let surface_config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: TextureFormat::Rgba16Float,
            width: size.width,
            height: size.height,
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
        };

        self.surface.configure(&self.device, &surface_config);

        let layout = self
            .device
            .create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("display pipeline layout"),
                bind_group_layouts: &[self.display_bind_group_layout.as_ref().unwrap()],
                push_constant_ranges: &[],
            });

        self.display_pipeline = Some(self.device.create_render_pipeline(
            &RenderPipelineDescriptor {
                label: Some("screen pipeline"),
                layout: Some(&layout),
                vertex: VertexState {
                    module: self.display_shader.as_ref().unwrap(),
                    entry_point: "vs_main",
                    buffers: &[],
                },
                fragment: Some(FragmentState {
                    module: self.display_shader.as_ref().unwrap(),
                    entry_point: "fs_main",
                    targets: &[Some(ColorTargetState {
                        format: surface_config.format,
                        blend: Some(BlendState::REPLACE),
                        write_mask: ColorWrites::ALL,
                    })],
                }),
                primitive: PrimitiveState {
                    topology: PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: FrontFace::Ccw,
                    cull_mode: None, // Some(Face::Cw)
                    polygon_mode: PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: MultisampleState::default(),
                multiview: None,
            },
        ));
    }
}
