use std::sync::Arc;
use image::Rgb;
use winit::window::Window;
use crate::gui::wgpu_container::WGPUContainer;
// The current State of our program, contains various GPU structs.
#[derive(Debug)]
pub struct State {
    pub(crate) window: Arc<Window>,
    pub(crate) gpu: WGPUContainer,
    pub(crate) rgb_color: Rgb<f32>
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

        // We can't render unless the surface is configured
        if !self.gpu.is_surface_configured {
            return Ok(());
        }

        // The current texture to draw to
        let output = self.gpu.surface.get_current_texture()?;

        // A view of that texture. A view allows us to control how the render code interacts with the texture
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        // The encoder stores the commands we are going to send to the GPU, like a command buffer
        let mut encoder = self.gpu.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });


        // The code exists in an enclosure for some ownership workaround (I think)
        {
            let color_r = self.rgb_color.0[0] as f64;
            let color_g = self.rgb_color.0[1] as f64;
            let color_b = self.rgb_color.0[2] as f64;
            let color_a = 1.0 ;

            // A render pass is born from the encoder and has all the methods for actually rendering
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: color_r,
                            g: color_g,
                            b: color_b,
                            a: color_a,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.gpu.color_pipeline);
            render_pass.draw(0..3,0..1);
        }

        // submit will accept anything that implements IntoIter
        // Here we turn the encoder into an iter and send it
        self.gpu.queue.submit(std::iter::once(encoder.finish()));

        // Show the output
        output.present();

        Ok(())
    }
}
