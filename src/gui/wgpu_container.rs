use std::sync::Arc;
use log::info;
use wgpu::{BindGroupDescriptor, Device, IndexFormat, Queue, ShaderStages, Surface, SurfaceCapabilities, SurfaceConfiguration};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use winit::dpi::PhysicalSize;
use winit::window::Window;
use crate::gpu::camera::{Camera, CameraController, CameraUniform};
use crate::gpu::texture::{self};
use crate::gpu::vertex::{Vertex, VERTICES};

// Struct containing all the WGPU internals
#[derive(Debug)]
pub struct WGPUContainer {
    pub surface: Surface<'static>,
    pub device: Device,
    pub queue: Queue,
    pub config: SurfaceConfiguration,
    pub diffuse_bind_groups: Vec<wgpu::BindGroup>,
    pub diffuse_textures: Vec<texture::Texture>,
    pub texture_index: usize,
    pub pipeline: wgpu::RenderPipeline,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_vertices: u32,
    pub num_indices: u32,
    pub current_pipeline: wgpu::RenderPipeline,
    pub is_surface_configured: bool,
    pub camera: Camera,
    pub camera_uniform: CameraUniform,
    pub camera_controller: CameraController,
    pub camera_buffer: wgpu::Buffer,
    pub camera_bind_group: wgpu::BindGroup
}

impl WGPUContainer {
    pub async fn new(window: Arc<Window>, camera: Camera) -> anyhow::Result<Self> {
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

        let diffuse_bytes = include_bytes!("/Users/maxwell/RustroverProjects/BarnesHutSimulation/textures/happy-tree.png");
        let diffuse_bytes_2 = include_bytes!("/Users/maxwell/RustroverProjects/BarnesHutSimulation/textures/brick-texture.jpeg");
        let diffuse_texture = texture::Texture::from_bytes(&device, &queue, diffuse_bytes, "happy-tree texture").unwrap();
        let diffuse_texture_2 = texture::Texture::from_bytes(&device, &queue, diffuse_bytes_2, "brick-wall texture").unwrap(); 
        let textures = vec![diffuse_texture,diffuse_texture_2];
        // Define a bind group layout for our textures so they can be accessed by a shader
        let texture_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
            label: Some("texture_bind_group_layout"),
            entries: &[
                // Create a layout entry for the actual texture
                wgpu::BindGroupLayoutEntry{
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture { 
                        sample_type: wgpu::TextureSampleType::Float { filterable: true }, 
                        view_dimension: wgpu::TextureViewDimension::D2, 
                        multisampled: false },
                    count: None,
                },
                
                // Create a layout entry for the sampler
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                }

            ]
        });


        let texture_index = 0;
        let mut diffuse_bind_groups: Vec<wgpu::BindGroup> = Vec::new();
        // Now we create the bind group
        // This creates the bindings for the resources
        for texture in &textures{
            // The reason its separate is so we can replace the resources but use the same bind group layout
            // If we change the resources however, we have to adhere to the bind group layout
            let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor{
                label: Some("diffuse_bind_group"),
                layout: &texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry{
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&texture.view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&texture.sampler),
                    }
                ]
            });
            diffuse_bind_groups.push(diffuse_bind_group);
        }
       

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor{
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("/Users/maxwell/RustroverProjects/BarnesHutSimulation/shaders/shader.wgsl").into())
        });


        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_proj(&camera);

        let camera_buffer = device.create_buffer_init(&BufferInitDescriptor { 
            label: Some("camera buffer"), 
            contents: bytemuck::cast_slice(&[camera_uniform]), 
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST, 
        });

        let camera_buffer_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("camera buffer bind group layout"),
            entries: &[wgpu::BindGroupLayoutEntry{
                binding: 0,
                // Vertex transformations so send it to the VERTEX
                visibility: ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer { 
                    // It's uniform so its the same for all buffers
                    ty: wgpu::BufferBindingType::Uniform, 
                    // No dynamic offset since we wont change it
                    has_dynamic_offset: false, 
                    // No need to define min binding size
                    min_binding_size: None 
                },
                count: None
            }],
        });

        let camera_buffer_bind_group = device.create_bind_group(&BindGroupDescriptor{
            label: Some("camera buffer bind group"),
            layout: &camera_buffer_bind_group_layout,
            entries: &[wgpu::BindGroupEntry{
                binding: 0,
                resource: camera_buffer.as_entire_binding()
            }]
        });


        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{
            label: Some("Render pipeline layout"),
            bind_group_layouts: &[&texture_bind_group_layout, &camera_buffer_bind_group_layout],
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
            diffuse_bind_groups: diffuse_bind_groups,
            diffuse_textures: textures,
            texture_index,
            current_pipeline: render_pipeline.clone(),
            pipeline: render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices: crate::gpu::vertex::INDICES.len() as u32,
            is_surface_configured: false,
            num_vertices: VERTICES.len() as u32, // tell the pipeline to draw all the vertices in the const
            camera,
            camera_uniform,
            camera_buffer,
            camera_bind_group: camera_buffer_bind_group,
            camera_controller: CameraController { speed: 1.0f32 }
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

            let texture_bind_group = self.diffuse_bind_groups.get(self.texture_index).unwrap();
            render_pass.set_pipeline(&self.current_pipeline);
            render_pass.set_bind_group(0, texture_bind_group, &[]);
            render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
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
    
    pub fn increment_texture(&mut self){
        let mut new_index = self.texture_index.saturating_add(1);
        if new_index >= self.diffuse_textures.len() {
            new_index = 0;
        }
        self.texture_index = new_index;
    }
}