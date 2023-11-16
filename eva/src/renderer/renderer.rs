use crate::{
    prelude::FlatScene,
    shader::{ShaderCamera, ShaderGlobalConfig, ShaderStruct},
};

use nalgebra::Vector3;
use wgpu::{util::DeviceExt, *};
use winit::{
    event::{ElementState, VirtualKeyCode},
    window::Window,
};

use super::RenderContext;

pub struct Renderer {
    pub surface: Surface,
    pub device: Device,
    pub queue: Queue,
    pub window: Window,
    pub display_pipeline: RenderPipeline,
    pub display_bind_group_layout: BindGroupLayout,
    pub ray_tracer_bind_group_layout: BindGroupLayout,
    pub ray_tracer_pipeline: ComputePipeline,
    pub context: RenderContext,

    pub camera_buffer: Buffer,
    pub config_buffer: Buffer,
    pub spheres_buffer: Buffer,
    pub lights_buffer: Buffer,
}

impl Renderer {
    pub fn render(&mut self) -> Result<(), SurfaceError> {
        let surface_texture = self.surface.get_current_texture().unwrap();

        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor::default());
        self.encode_pass(&mut encoder, &surface_texture);
        self.queue.submit([encoder.finish()]);

        surface_texture.present();

        Ok(())
    }

    // Temporary: just for testing.
    pub fn update(&mut self, key: VirtualKeyCode, state: ElementState) {
        let speed = 10.0;

        match (key, state) {
            (VirtualKeyCode::A, ElementState::Pressed) => {
                self.context.camera.translate(Vector3::new(speed, 0.0, 0.0))
            }
            (VirtualKeyCode::D, ElementState::Pressed) => self
                .context
                .camera
                .translate(Vector3::new(-speed, 0.0, 0.0)),
            (VirtualKeyCode::W, ElementState::Pressed) => {
                self.context.camera.translate(Vector3::new(0.0, speed, 0.0))
            }
            (VirtualKeyCode::S, ElementState::Pressed) => self
                .context
                .camera
                .translate(Vector3::new(0.0, -speed, 0.0)),
            _ => {}
        };
    }
}

impl Renderer {
    fn encode_pass(&self, encoder: &mut CommandEncoder, surface_texture: &SurfaceTexture) {
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
            usage: TextureUsages::COPY_DST
                | TextureUsages::STORAGE_BINDING
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

        self.ray_tracer_pass(encoder, &texture_view);
        self.display_pass(encoder, &surface_texture, &texture_view);
    }

    fn display_pass(
        &self,
        encoder: &mut CommandEncoder,
        surface_texture: &SurfaceTexture,
        texture_view: &TextureView,
    ) {
        // Create a view into the current texture that we can write to.
        let view = surface_texture
            .texture
            .create_view(&TextureViewDescriptor::default());

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
                        r: 1.0,
                        g: 0.0,
                        b: 1.0,
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
    fn ray_tracer_pass(&self, encoder: &mut CommandEncoder, texture_view: &TextureView) {
        let window_size = self.window.inner_size();
        let shader_camera: ShaderCamera = self.context.camera.clone().into();
        let flat_scene: FlatScene = self.context.scene.clone().into();

        let config = ShaderGlobalConfig {
            ambient: flat_scene.ambient,
        };

        self.queue.write_buffer(&self.config_buffer, 0, &config.as_bytes().unwrap());
        self.queue.write_buffer(&self.camera_buffer, 0, &shader_camera.as_bytes().unwrap());
        self.queue.write_buffer(&self.spheres_buffer, 0, &flat_scene.spheres.as_bytes().unwrap());
        self.queue.write_buffer(&self.lights_buffer, 0, &flat_scene.lights.as_bytes().unwrap());

        let ray_tracer_bind_group = self.device.create_bind_group(&BindGroupDescriptor {
            label: Some("ray tracer bind group"),
            layout: &self.ray_tracer_bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&texture_view),
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
                    resource: self.lights_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 4,
                    resource: self.config_buffer.as_entire_binding(),
                },
            ],
        });

        // Invoke the compute shader.
        let mut ray_tracer_pass = encoder.begin_compute_pass(&ComputePassDescriptor {
            label: Some("ray tracer pass"),
        });

        ray_tracer_pass.set_pipeline(&self.ray_tracer_pipeline);
        ray_tracer_pass.set_bind_group(0, &ray_tracer_bind_group, &[]);
        ray_tracer_pass.dispatch_workgroups(window_size.width / 3, window_size.height / 3, 1);

        drop(ray_tracer_pass);
    }
}
