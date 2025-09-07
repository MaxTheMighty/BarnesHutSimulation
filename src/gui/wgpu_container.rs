use std::sync::Arc;
use log::info;
use wgpu::{Surface, SurfaceCapabilities, SurfaceConfiguration, Device, Queue};
use winit::dpi::PhysicalSize;
use winit::window::Window;

// Struct containing all the WGPU internals
#[derive(Debug)]
pub struct WGPUContainer {
    pub surface: Surface<'static>,
    pub device: Device,
    pub queue: Queue,
    pub config: SurfaceConfiguration,
    pub pipeline: wgpu::RenderPipeline,
    pub color_pipeline: wgpu::RenderPipeline,
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

        let surface_config: SurfaceConfiguration = SurfaceConfiguration {
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

        // Create a second pipeline that uses the triangle's position data to create a color that it then sends to the fragment shader.
        // Have the app swap between these when you press the spacebar.
        // Hint: you'll need to modify VertexOutput

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
                buffers: &[],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState{
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState{
                    format: surface_config.format,
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

        let color_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{
            label: Some("Color Render pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &color_shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState{
                module: &color_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState{
                    format: surface_config.format,
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


        Ok(Self {
            surface,
            device,
            queue,
            config: surface_config,
            pipeline: render_pipeline,
            color_pipeline,
            is_surface_configured: false,

        })
    }
}