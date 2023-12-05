use crate::{
    prelude::{FlatScene, align, screenshot_rgba16f_buffer},
    shader::{ShaderCamera, ShaderGlobalConfig, ShaderStruct},
};

use wgpu::*;
use winit::window::Window;

use super::{StaticRenderContext, DynamicRenderContext};


pub struct Renderer {
    pub surface: Surface,
    pub device: Device,
    pub queue: Queue,
    pub window: Window,
    pub static_context: StaticRenderContext,

    pub ray_tracer_pipeline: ComputePipeline,
    pub display_pipeline: RenderPipeline,
    pub copy_texture_pipeline: RenderPipeline,

    pub ray_tracer_bind_group_layout: BindGroupLayout,
    pub display_bind_group_layout: BindGroupLayout,
    pub copy_texture_bind_group_layout: BindGroupLayout,
    pub mesh_bind_group_layout: BindGroupLayout,
    pub texture_bind_group_layout: BindGroupLayout,
    pub skybox_bind_group_layout: BindGroupLayout,

    pub texture_bind_group: BindGroup,
    pub skybox_bind_group: BindGroup,

    pub mesh_positions_buffer: Buffer,
    pub mesh_triangles_buffer: Buffer,
    pub mesh_headers_buffer: Buffer,
    pub mesh_normals_buffer: Buffer,
    pub mesh_vertices_buffer: Buffer,

    pub camera_buffer: Buffer,
    pub config_buffer: Buffer,
    pub spheres_buffer: Buffer,
    pub cubes_buffer: Buffer,
    pub materials_buffer: Buffer,
    pub screenshot_buffer: Buffer
}

impl Renderer {
    pub fn render(&mut self, context: &DynamicRenderContext) -> Result<(), SurfaceError> {
        let surface_texture = self.surface.get_current_texture().unwrap();

        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor::default());

        self.encode_pass(&mut encoder, &surface_texture, context);
        self.queue.submit([encoder.finish()]);
        
        if let Some(path) = &context.screenshot {
            // Wait for the render to complete.
            while !self.device.poll(MaintainBase::Poll) {
                continue;
            }
            screenshot_rgba16f_buffer(
                &self.device, 
                &self.screenshot_buffer, 
                path.to_owned(),
                self.width(), 
                self.height()
            );
        }

        surface_texture.present();
        Ok(())
    }
}

impl Renderer {
    fn width(&self) -> u32 {
        self.window.inner_size().width
    }

    fn height(&self) -> u32 {
        self.window.inner_size().height
    }

    #[rustfmt::skip]
    fn encode_pass(&self, encoder: &mut CommandEncoder, surface_texture: &SurfaceTexture, context: &DynamicRenderContext) {
        let raw_texture = self.device.create_texture(&TextureDescriptor {
            label: Some("display texture"),
            size: Extent3d {
                width: self.width(),
                height: self.height(),
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba16Float,
            usage: TextureUsages::COPY_DST
                | TextureUsages::STORAGE_BINDING
                | TextureUsages::TEXTURE_BINDING 
                | TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let raw_texture_view = raw_texture.create_view(&TextureViewDescriptor {
            label: None,
            format: Some(TextureFormat::Rgba16Float),
            dimension: Some(TextureViewDimension::D2),
            aspect: TextureAspect::All,
            base_mip_level: 0,
            mip_level_count: Some(1),
            base_array_layer: 0,
            array_layer_count: None,
        });

        let msaa_texture = self.device.create_texture(&TextureDescriptor {
            label: Some("display texture"),
            size: Extent3d {
                width: self.width(),
                height: self.height(),
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8Unorm,
            usage: TextureUsages::COPY_DST
                | TextureUsages::TEXTURE_BINDING 
                | TextureUsages::RENDER_ATTACHMENT
                | TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let msaa_texture_view = msaa_texture.create_view(&TextureViewDescriptor {
            label: None,
            format: Some(TextureFormat::Bgra8Unorm),
            dimension: Some(TextureViewDimension::D2),
            aspect: TextureAspect::All,
            base_mip_level: 0,
            mip_level_count: Some(1),
            base_array_layer: 0,
            array_layer_count: None,
        });

        self.ray_tracer_pass(encoder, &raw_texture_view, context);
        self.copy_texture_pass(encoder, &raw_texture_view, &msaa_texture_view);
        let output_view = surface_texture.texture.create_view(&TextureViewDescriptor::default());
        self.display_pass(encoder, &output_view, &msaa_texture_view);

        if context.screenshot.is_some() {
            self.screenshot_pass(encoder, &msaa_texture);
        }
    }

    fn copy_texture_pass(
        &self,
        encoder: &mut CommandEncoder,
        single_sample_texture_view: &TextureView,
        output_texture_view: &TextureView,
    ) {
        let msaa_texture = self.device.create_texture(&TextureDescriptor {
            label: Some("display texture"),
            size: Extent3d {
                width: self.width(),
                height: self.height(),
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 4,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8Unorm,
            usage: TextureUsages::COPY_DST
                | TextureUsages::TEXTURE_BINDING 
                | TextureUsages::RENDER_ATTACHMENT 
                | TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let msaa_texture_view = msaa_texture.create_view(&TextureViewDescriptor {
            label: None,
            format: Some(TextureFormat::Bgra8Unorm),
            dimension: Some(TextureViewDimension::D2),
            aspect: TextureAspect::All,
            base_mip_level: 0,
            mip_level_count: Some(1),
            base_array_layer: 0,
            array_layer_count: None,
        });

        let sampler = self.device.create_sampler(&SamplerDescriptor::default());
        let copy_texture_bind_group = self.device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &self.copy_texture_bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::Sampler(&sampler),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::TextureView(&single_sample_texture_view),
                },
            ],
        });

        let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &msaa_texture_view,
                resolve_target: Some(output_texture_view),
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

        render_pass.set_pipeline(&self.copy_texture_pipeline);
        render_pass.set_bind_group(0, &copy_texture_bind_group, &[]);
        render_pass.draw(0..6 as u32, 0..1);

        drop(render_pass);
    }

    fn display_pass(
        &self,
        encoder: &mut CommandEncoder,
        output_view: &TextureView,
        texture_view: &TextureView,
    ) {
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
                view: &output_view,
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


    #[rustfmt::skip]
    fn ray_tracer_pass(&self, encoder: &mut CommandEncoder, raw_texture_view: &TextureView, context: &DynamicRenderContext) {
        let shader_camera: ShaderCamera = context.camera.clone().into();
        let flat_scene: FlatScene = context.scene.clone().into();

        let config = ShaderGlobalConfig {
            ambient: self.static_context.ambient,
            sample_count: self.static_context.sample_count,
            max_reflections: self.static_context.max_reflections
        };

        self.queue.write_buffer(&self.config_buffer, 0, &config.as_bytes().unwrap());
        self.queue.write_buffer(&self.camera_buffer, 0, &shader_camera.as_bytes().unwrap());
        self.queue.write_buffer(&self.spheres_buffer, 0, &flat_scene.spheres.as_bytes().unwrap());
        self.queue.write_buffer(&self.cubes_buffer, 0, &flat_scene.cubes.as_bytes().unwrap());
        self.queue.write_buffer(&self.materials_buffer, 0, &self.static_context.materials.as_bytes().unwrap());

        self.queue.write_buffer(&self.mesh_headers_buffer, 0, &flat_scene.meshes.headers.as_bytes().unwrap());
        self.queue.write_buffer(&self.mesh_triangles_buffer, 0, &flat_scene.meshes.triangles.as_bytes().unwrap());
        self.queue.write_buffer(&self.mesh_vertices_buffer, 0, &flat_scene.meshes.vertices.as_bytes().unwrap());
        self.queue.write_buffer(&self.mesh_positions_buffer, 0, &flat_scene.meshes.positions.as_bytes().unwrap());
        self.queue.write_buffer(&self.mesh_normals_buffer, 0, &flat_scene.meshes.normals.as_bytes().unwrap());

        let mesh_bind_group = self.device.create_bind_group(&BindGroupDescriptor { 
            label: None, 
            layout: &self.mesh_bind_group_layout, 
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: self.mesh_headers_buffer.as_entire_binding()
                },
                BindGroupEntry {
                    binding: 1,
                    resource: self.mesh_triangles_buffer.as_entire_binding()
                },
                BindGroupEntry {
                    binding: 2,
                    resource: self.mesh_vertices_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 3,
                    resource: self.mesh_positions_buffer.as_entire_binding()
                },
                BindGroupEntry {
                    binding: 4,
                    resource: self.mesh_normals_buffer.as_entire_binding()
                } 
            ]
        });

        let ray_tracer_bind_group = self.device.create_bind_group(&BindGroupDescriptor {
            label: Some("ray tracer bind group"),
            layout: &self.ray_tracer_bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&raw_texture_view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: self.camera_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: self.spheres_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 3,
                    resource: self.config_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 4,
                    resource: self.cubes_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 5,
                    resource: self.materials_buffer.as_entire_binding()
                }
            ],
        });

        // Invoke the compute shader.
        let mut ray_tracer_pass = encoder.begin_compute_pass(&ComputePassDescriptor {
            label: Some("ray tracer pass"),
        });

        ray_tracer_pass.set_pipeline(&self.ray_tracer_pipeline);
        ray_tracer_pass.set_bind_group(0, &ray_tracer_bind_group, &[]);
        ray_tracer_pass.set_bind_group(1, &mesh_bind_group, &[]);
        ray_tracer_pass.set_bind_group(2, &self.texture_bind_group, &[]);
        ray_tracer_pass.set_bind_group(3, &self.skybox_bind_group, &[]);

        ray_tracer_pass.dispatch_workgroups(self.width(), self.height(), 1);

        drop(ray_tracer_pass);
    }

    fn screenshot_pass(&self, encoder: &mut CommandEncoder, texture: &Texture) {
        encoder.copy_texture_to_buffer(
            ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
                aspect: TextureAspect::All,
            },
            ImageCopyBuffer {
                buffer: &self.screenshot_buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(align(self.width(), 256) * 8),
                    rows_per_image: Some(self.height()),
                },
            },
            Extent3d {
                width: self.width(),
                height: self.height(),
                depth_or_array_layers: 1,
            },
        );
    }
}
