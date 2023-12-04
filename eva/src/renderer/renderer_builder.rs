use std::num::NonZeroU32;

use pollster::FutureExt;
use wgpu::*;
use winit::window::Window;

use crate::prelude::*;

use super::StaticRenderContext;

const SPHERE_COUNT: u64 = 100;
const CUBE_COUNT: u64 = 100;
const MATERIAL_COUNT: u64 = 200;
const MESH_POINT_BUFFER_SIZE: u64 = 1_500_000;
const MESH_TRIANGLE_BUFFER_SIZE: u64 = 1_500_000;
const MESH_HEADERS_BUFFER_SIZE: u64 = 10_000;

pub struct RendererBuilder {
    surface: Surface,
    device: Device,
    queue: Queue,
    window: Window,
    adapter: Adapter,
    context: StaticRenderContext,

    ray_tracer_shader: Option<ShaderModule>,
    display_shader: Option<ShaderModule>,

    ray_tracer_pipeline: Option<ComputePipeline>,
    display_pipeline: Option<RenderPipeline>,

    ray_tracer_bind_group_layout: Option<BindGroupLayout>,
    mesh_bind_group_layout: Option<BindGroupLayout>,
    texture_bind_group_layout: Option<BindGroupLayout>,
    skybox_bind_group_layout: Option<BindGroupLayout>,
    display_bind_group_layout: Option<BindGroupLayout>,

    texture_bind_group: Option<BindGroup>,
    skybox_bind_group: Option<BindGroup>,

    mesh_positions_buffer: Option<Buffer>,
    mesh_triangles_buffer: Option<Buffer>,
    mesh_normals_buffer: Option<Buffer>,
    mesh_vertices_buffer: Option<Buffer>,
    mesh_headers_buffer: Option<Buffer>,

    camera_buffer: Option<Buffer>,
    spheres_buffer: Option<Buffer>,
    cubes_buffer: Option<Buffer>,
    config_buffer: Option<Buffer>,
    materials_buffer: Option<Buffer>,    
    screenshot_buffer: Option<Buffer>,
}

impl RendererBuilder {
    pub fn new(window: Window, context: StaticRenderContext) -> Self {
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
            format: TextureFormat::Bgra8Unorm,
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
            skybox_bind_group_layout: None,
            display_bind_group_layout: None,

            texture_bind_group: None,
            skybox_bind_group: None,

            mesh_positions_buffer: None,
            mesh_triangles_buffer: None,
            mesh_headers_buffer: None,
            mesh_normals_buffer: None,
            mesh_vertices_buffer: None,

            camera_buffer: None,
            config_buffer: None,
            spheres_buffer: None,
            cubes_buffer: None,
            materials_buffer: None,
            screenshot_buffer: None
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
            static_context: self.context,

            ray_tracer_pipeline: self.ray_tracer_pipeline.unwrap(),
            display_pipeline: self.display_pipeline.unwrap(),

            ray_tracer_bind_group_layout: self.ray_tracer_bind_group_layout.unwrap(),
            mesh_bind_group_layout: self.mesh_bind_group_layout.unwrap(),
            texture_bind_group_layout: self.texture_bind_group_layout.unwrap(),
            skybox_bind_group_layout: self.skybox_bind_group_layout.unwrap(),
            display_bind_group_layout: self.display_bind_group_layout.unwrap(),

            texture_bind_group: self.texture_bind_group.unwrap(),
            skybox_bind_group: self.skybox_bind_group.unwrap(),

            mesh_positions_buffer: self.mesh_positions_buffer.unwrap(),
            mesh_triangles_buffer: self.mesh_triangles_buffer.unwrap(),
            mesh_headers_buffer: self.mesh_headers_buffer.unwrap(),
            mesh_normals_buffer: self.mesh_normals_buffer.unwrap(),
            mesh_vertices_buffer: self.mesh_vertices_buffer.unwrap(),

            camera_buffer: self.camera_buffer.unwrap(),
            config_buffer: self.config_buffer.unwrap(),
            cubes_buffer: self.cubes_buffer.unwrap(),
            spheres_buffer: self.spheres_buffer.unwrap(),
            materials_buffer: self.materials_buffer.unwrap(),
            screenshot_buffer: self.screenshot_buffer.unwrap()
        }
    }
}

impl RendererBuilder {
    #[rustfmt::skip]
    fn create_assets(&mut self) {
        // Shaders.
        self.ray_tracer_shader = Some(self.device.create_shader_module(include_wgsl!("../../shaders/ray_tracer.wgsl")));
        self.display_shader = Some(self.device.create_shader_module(include_wgsl!("../../shaders/display.wgsl")));

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

        self.mesh_positions_buffer = Some(self.device.create_buffer(&BufferDescriptor { 
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
        
        self.mesh_normals_buffer = Some(self.device.create_buffer(&BufferDescriptor { 
            label: None,
            size: MESH_POINT_BUFFER_SIZE, 
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST, 
            mapped_at_creation: false
        }));

        self.mesh_vertices_buffer = Some(self.device.create_buffer(&BufferDescriptor { 
            label: None,
            size: MESH_POINT_BUFFER_SIZE, 
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST, 
            mapped_at_creation: false
        }));

        self.materials_buffer = Some(self.device.create_buffer(&BufferDescriptor { 
            label: None, 
            size: PbrMaterial::size() * MATERIAL_COUNT, 
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST, 
            mapped_at_creation: false 
        }));

        let size = self.window.inner_size();
        self.screenshot_buffer = Some(self.device.create_buffer(&BufferDescriptor { 
            label: None, 
            size: (align(size.width, 256) * size.height * 8) as u64, 
            usage: BufferUsages::COPY_DST | BufferUsages::MAP_READ, 
            mapped_at_creation: false,
        }));
    }

    fn create_bind_group_layouts(&mut self) {
        self.skybox_bind_group_layout = Some(self.device.create_bind_group_layout(
            &BindGroupLayoutDescriptor { label: None, entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::Texture {
                        sample_type: TextureSampleType::Float { filterable: false },
                        view_dimension: TextureViewDimension::Cube,
                        multisampled: false,
                    },
                    count: None
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::Sampler(SamplerBindingType::NonFiltering),
                    count: None
                },
            ]}
        ));

        self.texture_bind_group_layout = Some(self.device.create_bind_group_layout(
            &BindGroupLayoutDescriptor {
                label: None,
                entries: &[
                    BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::COMPUTE,
                        ty: BindingType::Texture {
                            sample_type: TextureSampleType::Float { filterable: false },
                            view_dimension: TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: NonZeroU32::new(TEXTURE_2D_COUNT),
                    },
                    BindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStages::COMPUTE,
                        ty: BindingType::Sampler(SamplerBindingType::NonFiltering),
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
                    },
                    BindGroupLayoutEntry {
                        binding: 3,
                        visibility: ShaderStages::COMPUTE,
                        ty: BindingType::Buffer {
                            ty: BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None
                    },
                    BindGroupLayoutEntry {
                        binding: 4,
                        visibility: ShaderStages::COMPUTE,
                        ty: BindingType::Buffer {
                            ty: BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None
                    },
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
                            ty: BufferBindingType::Uniform, 
                            has_dynamic_offset: false, 
                            min_binding_size: None 
                        },
                        count: None
                    },
                    BindGroupLayoutEntry {
                        binding: 4,
                        visibility: ShaderStages::COMPUTE,
                        ty: BindingType::Buffer { 
                            ty: BufferBindingType::Storage { read_only: true }, 
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
                            min_binding_size: None,
                        },
                        count: None
                    },
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

    fn create_bind_groups(&mut self) {
        // Texture bind group.
        let texture_descriptors: Vec<TextureDescriptor> = self.context.textures.textures().iter().map(|texture| texture.clone().into()).collect();
        let texture_extents: Vec<Extent3d> = self.context.textures.textures().iter().map(|texture| Extent3d { 
            width: texture.width(), 
            height: texture.height(), 
            depth_or_array_layers: 1, 
        }).collect();
        let texture_data: Vec<Vec<f32>> = self.context.textures.textures().iter().map(|texture| texture.as_bytes()).collect();

        let textures: Vec<Texture> = texture_descriptors.into_iter().map(|descriptor| self.device.create_texture(&descriptor)).collect();
        let texture_views: Vec<TextureView> = textures.iter().map(|texture| texture.create_view(&TextureViewDescriptor {
            ..Default::default()
        })).collect();

        let texture_2d_sampler = self.device.create_sampler(&SamplerDescriptor {
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            mipmap_filter: FilterMode::Linear,
            address_mode_u: AddressMode::ClampToEdge, 
            address_mode_v: AddressMode::ClampToEdge, 
            address_mode_w: AddressMode::ClampToEdge,
            lod_min_clamp: 0.0, 
            lod_max_clamp: std::f32::MAX,
            ..Default::default()
        });
        let mut samplers = Vec::new();

        for i in 0..texture_data.len() {
            self.queue.write_texture(
                textures[i].as_image_copy(),
                &bytemuck::cast_slice(&texture_data[i]),
                ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(4 * 4 * texture_extents[i].width),
                    rows_per_image: None,
                },
                texture_extents[i]
            );
            samplers.push(&texture_2d_sampler);
        }

        self.texture_bind_group = Some(self.device.create_bind_group(&BindGroupDescriptor { 
            label: None, 
            layout: &self.texture_bind_group_layout.as_ref().unwrap(), 
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureViewArray(
                        &texture_views.iter().map(|c| c).collect::<Vec<&TextureView>>()
                    ) 
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::SamplerArray(&samplers)
                }
            ] 
        }));

        // Skybox bind group.
        let skybox_texture_view = self.device.create_skybox_view(&self.queue, &self.context.skybox);
        let skybox_sampler = self.device.create_sampler(&SamplerDescriptor::default());
        self.skybox_bind_group = Some(self.device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &self.skybox_bind_group_layout.as_ref().unwrap(),
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&skybox_texture_view) 
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(&skybox_sampler)
                }
            ]
        }))
    }

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
                    self.texture_bind_group_layout.as_ref().unwrap(),
                    self.skybox_bind_group_layout.as_ref().unwrap(),
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
