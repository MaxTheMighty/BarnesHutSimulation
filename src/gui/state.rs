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

    pub fn resize(&mut self, _width: u32, _height: u32) {
        // We'll do stuff here in the next tutorial
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

        let output = self.gpu.surface.get_current_texture()?;

        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.gpu.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });


        {
            let color_r = self.rgb_color.0[0] as f64;
            let color_g = self.rgb_color.0[1] as f64;
            let color_b = self.rgb_color.0[2] as f64;
            let color_a = 1.0 ;
            println!("color: {:#?}",self.rgb_color);

            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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
        }

        // submit will accept anything that implements IntoIter
        self.gpu.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
