use std::ops::Index;

use cgmath::Vector2;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use barnes_hut::body::Body;
use barnes_hut::gravity::*;



const WIDTH: u32 = 800;
const WIDTH_F: f32 = WIDTH as f32;
const HEIGHT: u32 = 800;

const LIMIT: usize = (WIDTH * HEIGHT) as usize;

fn main() -> Result<(), Error> {
    let mut bodies: Vec<Body> = Vec::new();
    bodies.push(Body::with_mass_and_pos(1.0,Vector2::new(400.0,600.0)));
    bodies.push(Body::with_mass_and_pos(500.0,Vector2::new(400.0,200.0)));
    bodies[0].velocity = Vector2::new(0.5f32,0.0f32);


    // //expected to be 80100
    // println!("{:}", calculate_buffer_pos(&bodies[0].pos));
    //
    // //index should have 0 as the last digit
    // for _ in 1..1000{
    //     apply_force(&mut bodies[0]);
    //     println!("{:}", calculate_buffer_pos(&bodies[0].pos));
    // }
    //
    //
    //
    //
    // return Ok(());





    // bodies.push(Body::with_mass_and_pos(50.0,Vector2::new(200.0,400.0)));
    // bodies.push(Body::with_mass_and_pos(50.0,Vector2::new(400.0,400.0)));
    //pre update
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let mut my_buffer: Vec<(u8,u8,u8,u8)> = Vec::new();
    my_buffer.resize(LIMIT, (0,0,0,255));
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
            draw(pixels.frame_mut(), &mut my_buffer);
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

            for i in 0..bodies.len() {
                for j in i..bodies.len() {
                    if i == j {
                        continue;
                    }
                    // println!("{} {}", i, j);
                    calculate_force(&mut bodies[i..j + 1]);
                }
            }


            let mut index: usize = 0;
            for body in bodies.iter_mut() {
                apply_force(body);
                println!("{:?}",body.pos);

                index = calculate_buffer_pos(&body.pos);
                // println!("y={:?}",y_temp);

                if(index > LIMIT){
                    continue;
                }
                my_buffer[index] = (255,255,255,255);
                // println!("{} = {:?}",index, my_buffer[index])
            }

            window.request_redraw();




        }





    });

}

fn calculate_buffer_pos(pos: &Vector2<f32>) -> usize{
    (pos.x + (pos.y.round() * WIDTH_F)) as usize
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

