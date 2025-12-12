use std::sync::Arc;
use image::Rgb;
use winit::window::Window;
use crate::gpu::camera::Camera;
use crate::gpu::vertex::VERTICES;
use crate::gui::wgpu_container::WGPUContainer;
// The current State of our program, contains various GPU structs.
#[derive(Debug)]
pub struct State {
    pub(crate) window: Arc<Window>,
    pub(crate) gpu: WGPUContainer,

}

impl State {
    // We don't need this to be async right now,
    // but we will in the next tutorial
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        let window_size = window.inner_size();
        let camera = Camera {
            // Position of the eye of the camera
            eye: (0.0,1.0,2.0).into(),
            // Look at the origin
            target: (0.0,0.0,0.0).into(),
            // Which way is up
            up: cgmath::Vector3::unit_y(),
            // Aspect ratio
            aspect: window_size.width as f32/ window_size.height as f32,
            // FOV
            fovy: 45.0,
            // Near clipping plane
            znear: 0.1,
            // Far clipping plane
            zfar: 100.0
        };
        let wgpu_container: WGPUContainer = WGPUContainer::new(window.clone(), camera).await?;
 

        Ok(Self {
            window,
            gpu: wgpu_container,
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
        // Update the camera uniform struct using the camera struct
        self.gpu.camera_uniform.update_view_proj(&self.gpu.camera);

        // Write this data to the uniform buffer
        let camera_slice = &[self.gpu.camera_uniform.clone()];
        let camera_uniform_bytes = bytemuck::cast_slice(camera_slice);
        self.gpu.queue.write_buffer(&self.gpu.camera_buffer, 0,camera_uniform_bytes) ;

    }

    // impl State
    pub(crate) fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.window.request_redraw();

        self.gpu.render()
    }
    
}
