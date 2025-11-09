use std::sync::Arc;
use image::Rgb;
use winit::window::Window;
use crate::gpu::vertex::VERTICES;
use crate::gui::wgpu_container::WGPUContainer;
// The current State of our program, contains various GPU structs.
#[derive(Debug)]
pub struct State {
    pub(crate) window: Arc<Window>,
    pub(crate) gpu: WGPUContainer,
    pub(crate) rgb_color: Rgb<f32>,
}

impl State {
    // We don't need this to be async right now,
    // but we will in the next tutorial
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {

        let wgpu_container: WGPUContainer = WGPUContainer::new(window.clone()).await?;
        
        Ok(Self {
            window,
            gpu: wgpu_container,
            rgb_color: Rgb::from([0.0,0.0,0.0]),
        })

    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.gpu.config.width = width;
            self.gpu.config.height = height;
            self.gpu.surface.configure(&self.gpu.device, &self.gpu.config);
            self.gpu.is_surface_configured = true;
        }
    }

    pub fn update(&mut self) {
        // todo!()

    }

    // impl State
    pub(crate) fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.window.request_redraw();

        self.gpu.render()
    }
    
}
