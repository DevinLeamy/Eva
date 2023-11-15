use wgpu::{util::DeviceExt, *};
use winit::window::Window;

pub struct Renderer {
    surface: Surface,
    device: Device,
    queue: Queue,
    surface_config: SurfaceConfiguration,
    window: Window,

    display_pipeline: RenderPipeline,
    display_bind_group_layout: BindGroupLayout,

    ray_tracer_bind_group_layout: BindGroupLayout,

    ray_tracer_pipeline: ComputePipeline,
}

impl Renderer {
    pub async fn new(window: Window) -> Self {
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
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    features: Features::empty(),
                    limits: Limits::default(),
                    label: None,
                },
                None,
            )
            .await
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

        // Create the compute pipeline.
        let compute_shader =
            device.create_shader_module(include_wgsl!("../assets/shaders/ray_tracer.wgsl"));

        let ray_tracer_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("ray tracer bind group layout"),
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::StorageTexture {
                        access: wgpu::StorageTextureAccess::WriteOnly,
                        format: wgpu::TextureFormat::Rgba8Unorm,
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                    count: None,
                }],
            });

        let ray_tracer_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("ray tracer pipeline layout"),
            bind_group_layouts: &[&ray_tracer_bind_group_layout],
            push_constant_ranges: &[],
        });

        let ray_tracer_pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some("ray tracer pipeline"),
            layout: Some(&ray_tracer_pipeline_layout),
            module: &compute_shader,
            entry_point: "compute_main",
        });

        // Create the pipelines.
        let display_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
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
            });

        let display_shader =
            device.create_shader_module(include_wgsl!("../assets/shaders/display.wgsl"));
        let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("display pipeline layout"),
            bind_group_layouts: &[&display_bind_group_layout],
            push_constant_ranges: &[],
        });

        let display_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("screen pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: VertexState {
                module: &display_shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(FragmentState {
                module: &display_shader,
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
                cull_mode: Some(Face::Back),
                polygon_mode: PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState::default(),
            multiview: None,
        });

        Self {
            surface,
            device,
            queue,
            surface_config,
            window,

            ray_tracer_pipeline,
            display_pipeline,

            display_bind_group_layout,
            ray_tracer_bind_group_layout,
        }
    }

    pub fn render(&mut self) -> Result<(), SurfaceError> {
        // Build a command encoder to encode commands that are send to the GPU.
        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor::default());

        let surface_texture = self.surface.get_current_texture().unwrap();

        self.ray_tracer_pass(&mut encoder);
        self.display_pass(&mut encoder, &surface_texture);

        self.queue.submit([encoder.finish()]);

        surface_texture.present();

        Ok(())
    }
}

impl Renderer {
    fn display_pass(&self, encoder: &mut CommandEncoder, surface_texture: &SurfaceTexture) {
        // Create a view into the current texture that we can write to.
        let view = surface_texture
            .texture
            .create_view(&TextureViewDescriptor::default());

        let window_size = self.window.inner_size();
        let texture = self.device.create_texture(&TextureDescriptor {
            label: Some("display texture"),
            size: Extent3d {
                width: window_size.width,
                height: window_size.height,
                depth_or_array_layers: 1, // not sure why.
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba16Float,
            usage: TextureUsages::COPY_SRC
                | TextureUsages::COPY_DST
                | TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let texture_view = texture.create_view(&TextureViewDescriptor {
            label: Some("texture view"),
            format: Some(TextureFormat::Rgba16Float),
            dimension: Some(TextureViewDimension::D2),
            aspect: TextureAspect::All,
            base_mip_level: 0,
            mip_level_count: Some(1),
            base_array_layer: 0,
            array_layer_count: None,
        });
        let sampler = self.device.create_sampler(&SamplerDescriptor::default());

        let display_bind_group = self.device.create_bind_group(&BindGroupDescriptor {
            label: Some("display bind group"),
            layout: &self.display_bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::Sampler(&sampler),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::TextureView(&texture_view),
                },
            ],
        });

        let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("render pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                // The texture view to write the data to.
                view: &view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0,
                    }),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        render_pass.set_pipeline(&self.display_pipeline);
        render_pass.set_bind_group(0, &display_bind_group, &[]);
        render_pass.draw(0..6 as u32, 0..1);

        drop(render_pass);
    }

    fn ray_tracer_pass(&self, encoder: &mut CommandEncoder) {
        let window_size = self.window.inner_size();

        let storage_texture = self.device.create_texture(&TextureDescriptor {
            label: None,
            size: Extent3d {
                width: window_size.width,
                height: window_size.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8Unorm,
            usage: TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });

        let storage_texture_view = storage_texture.create_view(&TextureViewDescriptor::default());

        let ray_tracer_bind_group = self.device.create_bind_group(&BindGroupDescriptor {
            label: Some("ray tracer bind group"),
            layout: &self.ray_tracer_bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: BindingResource::TextureView(&storage_texture_view),
            }],
        });

        // Invoke the compute shader.
        let mut ray_tracer_pass = encoder.begin_compute_pass(&ComputePassDescriptor {
            label: Some("ray tracer pass"),
        });

        ray_tracer_pass.set_pipeline(&self.ray_tracer_pipeline);
        ray_tracer_pass.set_bind_group(0, &ray_tracer_bind_group, &[]);
        ray_tracer_pass.dispatch_workgroups(8, 8, 1);

        drop(ray_tracer_pass);
    }
}
