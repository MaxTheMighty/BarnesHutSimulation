#![allow(warnings)]

use cgmath::Vector3;
use cgmath::Vector2;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use barnes_hut::body::Body;
use barnes_hut::gravity::*;
use barnes_hut::simulation::Simulation;


const WIDTH: u32 = 1000;
const WIDTH_F: f64 = WIDTH as f64;
const HEIGHT: u32 = 1000;

const HEIGHT_F: f64 = HEIGHT as f64;

const LIMIT: usize = (WIDTH * HEIGHT) as usize;

fn main() -> Result<(), Error> {

    let mut simulation = Simulation::new();
    // simulation.generate(2000, 500.0f64,200.0f64);
    // simulation.cube(600.0, 600.0, 60.0, 100.0, 2000);
    println!("{:?}",simulation.bodies.len());
    // simulation.square(50u16,300.0f64);
    // // simulation.bodies.push(Body::with_mass_and_pos(10.0f64,Vector3::new(510.0f64,500.0f64,0.0f64)));
    // simulation.bodies.push(Body::with_mass_and_pos(50000.0f64,Vector2::new(350.0f64,350.0f64)));
    // simulation.bodies.push(Body::with_mass_and_pos(20.0f64,Vector3::new(500.0f64,500.0f64,1000.0f64)));
    // simulation.bodies[1].velocity.x = -1.0f64;
    // simulation.bodies.push(Body::with_mass_and_pos(1000.0,Vector2::new(500.0,50.0)));
    // simulation.bodies.push(Body::with_mass_and_pos(1.0,Vector2::new(450.0,100.0)));
    // simulation.bodies[1].velocity.y = -1.0;
    // simulation.bodies[1].velocity.x = 1.0;
    simulation.square(50,200.0);
    simulation.square(50,600.0);
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

            simulation.update();
            let mut index: usize = 0;
            let mut alpha: u8 = 0;
            for body in &simulation.bodies {
                //println!("z: {:?}",body.pos.z);
                if(body.pos.x > WIDTH_F || body.pos.y > HEIGHT_F || body.pos.x < 0.0f64 || body.pos.y < 0.0f64) {
                    continue;
                }
                index = calculate_buffer_pos(&body.pos);
                if(index >= LIMIT ){
                    continue;
                }

                // alpha = (255 - body.pos.z as u8).max(5);
                my_buffer[index] = (255,255,255,255);
            }
            window.request_redraw();
        }
    });

}


fn calculate_buffer_pos(pos: &Vector2<f64>) -> usize{
    return (pos.x + (pos.y.round() * WIDTH_F)) as usize;
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

