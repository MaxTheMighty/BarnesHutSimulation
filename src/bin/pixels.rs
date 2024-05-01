#![deny(clippy::all)]
#![forbid(unsafe_code)]

use std::ops::IndexMut;
use cgmath::Vector2;
use error_iter::ErrorIter as _;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;


/// Representation of the application state. In this example, a box will bounce around the screen.

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let mut my_buffer: Vec<(u8,u8,u8,u8)> = Vec::new();
    let mut buffer_update: bool = false;
    my_buffer.resize((WIDTH * HEIGHT) as usize, (0,0,0,255));
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
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
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };


    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            if buffer_update {
                draw(pixels.frame_mut(), &my_buffer);
                buffer_update = false;
            }

            if let Err(err) = pixels.render() {
                log_error("pixels.render", err);
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

            if input.mouse_pressed(0){
                match Some(input.mouse()){
                    Some(x) => {
                        let x_unwrap = x.unwrap();
                        let pos = x_unwrap.0 as usize + (x_unwrap.1 as usize * WIDTH as usize);
                        my_buffer[pos] = (255,255,255,255);
                        buffer_update = true;
                    }
                    None => {;}
                }
            }

            // Update internal state and request a redraw
            //world.update();
            window.request_redraw();
        }
    });
}

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}


fn draw(buffer: &mut [u8], my_buffer: &Vec<(u8,u8,u8,u8)>){
    for (i,mut pixel) in buffer.chunks_exact_mut(4).enumerate(){
        pixel[0] = my_buffer[i].0;
        pixel[1] = my_buffer[i].1;
        pixel[2] = my_buffer[i].2;
        pixel[3] = my_buffer[i].3;
    }
}