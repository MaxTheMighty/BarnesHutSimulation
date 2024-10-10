use std::env;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use barnes_hut::canvas::Canvas;
pub mod frame_renderer;

fn main() -> Result<(), Error> {
    env::set_var("RUST_BACKTRACE", "FULL");
    //pre update
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let mut canvas: Canvas = Canvas::new(1001,1001,(0,0,0,0));
    let window = {
        let size = LogicalSize::new(1001, 1001);
        WindowBuilder::new()
            .with_title("Hello Pixels")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(1001, 1001, surface_texture)?
    };





    // return Ok(());
    event_loop.run(move |event, _, control_flow| {

        // canvas.set_color(0,1000,&(255,0,0,255));
        // for i in 0..360{
        //     canvas.set_hue(i, 0, i as f64, 1.0, 1.0);
        // }
        // canvas.copy_huemap_to_canvas();

        // canvas.draw_square(5,3,3,&(255,255,255,255));
        // canvas.draw_square(0,0,500,&(255,255,255,255));
        // canvas.draw_square(500,0,500,&(255,255,255,255));
        // canvas.set_color_safe(10000,0,&(255,255,255,255));
        // canvas.draw_square(500,500,400,&(255,255,255,255));
        // canvas.draw_square_safe(500,500,400,&(255,255,255,255));
        // canvas.draw_square(0,500,500,&(255,255,255,255));
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {

            draw(pixels.frame_mut(), &mut canvas.buffer);
            if let Err(err) = pixels.render() {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }


            window.request_redraw();
        }


    });
}

fn draw(buffer: &mut [u8], my_buffer: &mut Vec<(u8,u8,u8,u8)>){
    for (i,mut pixel) in buffer.chunks_exact_mut(4).enumerate(){
        pixel[0] = my_buffer[i].0;
        pixel[1] = my_buffer[i].1;
        pixel[2] = my_buffer[i].2;
        pixel[3] = my_buffer[i].3;

        my_buffer[i] = (0,0,0,0);
    }
}

