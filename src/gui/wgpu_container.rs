use std::sync::Arc;
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
        
        let adapter: wgpu::Adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }).await?;

        println!("Adapter selected: {:#?}",adapter);

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

        Ok(Self {
            surface,
            device,
            queue,
            config: surface_config,
            is_surface_configured: false,
        })
    }
}