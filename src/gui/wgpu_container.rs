use std::sync::Arc;
use log::info;
use wgpu::{Surface, SurfaceCapabilities, SurfaceConfiguration, Device, Queue, IndexFormat};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use winit::dpi::PhysicalSize;
use winit::window::Window;
use crate::gpu::vertex::{Vertex, VERTICES};

// Struct containing all the WGPU internals
#[derive(Debug)]
pub struct WGPUContainer {
    pub surface: Surface<'static>,
    pub device: Device,
    pub queue: Queue,
    pub config: SurfaceConfiguration,
    pub pipeline: wgpu::RenderPipeline,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_vertices: u32,
    pub num_indices: u32,
    pub current_pipeline: wgpu::RenderPipeline,
    pub is_surface_configured: bool,
}

impl WGPUContainer {
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        let size: PhysicalSize<u32> = window.inner_size();

        let instance: wgpu::Instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            #[cfg(not(target_arch = "wasm32"))]
            backends: wgpu::Backends::PRIMARY,
            #[cfg(target_arch = "wasm32")]
            backends: wgpu::Backends::GL,
            ..Default::default()
        });

        let surface: Surface = instance.create_surface(window.clone()).expect("Couldn't create a surface");

        info!("Surface selected: {:#?}", surface);

        let adapter: wgpu::Adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }).await?;


        info!("Adapter selected: {:#?}",adapter);

        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            memory_hints: Default::default(),
            trace: wgpu::Trace::Off
        }).await?;

        let surface_capabilities: SurfaceCapabilities = surface.get_capabilities(&adapter);

        let surface_format = surface_capabilities.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_capabilities.formats[0]);

        let config: SurfaceConfiguration = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_capabilities.present_modes[0],
            desired_maximum_frame_latency: 2,
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
        };

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor{
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("/Users/maxwell/RustroverProjects/BarnesHutSimulation/shaders/shader.wgsl").into())
        });

        let color_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor{
            label: Some("Color shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("/Users/maxwell/RustroverProjects/BarnesHutSimulation/shaders/color_shader.wgsl").into())
        });
        

        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{
            label: Some("Render pipeline layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[]
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{
            label: Some("Render pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Vertex::desc()],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState{
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState{
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default()
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false
            },
            multiview: None,
            cache: None
        }
        );

        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor{
            label: Some("Vertex buffer"),
            contents: bytemuck::cast_slice(crate::gpu::vertex::VERTICES),
            usage: wgpu::BufferUsages::VERTEX
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(crate::gpu::vertex::INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        Ok(Self {
            surface,
            device,
            queue,
            config,
            current_pipeline: render_pipeline.clone(),
            pipeline: render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices: crate::gpu::vertex::INDICES.len() as u32,
            is_surface_configured: false,
            num_vertices: VERTICES.len() as u32 // tell the pipeline to draw all the vertices in the const

        })
    }

    pub fn render(&mut self) -> Result<(),wgpu::SurfaceError>{


        // We can't render unless the surface is configured
        if !self.is_surface_configured {
            return Ok(());
        }

        // The current texture to draw to
        let output = self.surface.get_current_texture()?;

        // A view of that texture. A view allows us to control how the render code interacts with the texture
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        // The encoder stores the commands we are going to send to the GPU, like a command buffer
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });


        // The code exists in an enclosure for some ownership workaround (I think)
        {

            // A render pass is born from the encoder and has all the methods for actually rendering
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 0.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.current_pipeline);
            // assign the entire vertex buffer (hence the .. in .slice()) at slot 0
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), IndexFormat::Uint16);
            // render_pass.draw(0..self.num_vertices,0..1);
            render_pass.draw_indexed(0..self.num_indices,0,0..1);
        }

        // submit will accept anything that implements IntoIter
        // Here we turn the encoder into an iter and send it
        self.queue.submit(std::iter::once(encoder.finish()));

        // Show the output
        output.present();

        Ok(())
    }

}