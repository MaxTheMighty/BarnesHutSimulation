#![allow(warnings)]

use cgmath::Vector3;
use cgmath::Vector2;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use barnes_hut::bh_runner::BarnesHutRunner;
use barnes_hut::body::Body;
use barnes_hut::gravity::*;
use barnes_hut::quadtree::{Quadtree, Rectangle};
use barnes_hut::simulation::Simulation;


const WIDTH: u32 = 1000;
const WIDTH_F: f64 = WIDTH as f64;
const HEIGHT: u32 = 1000;

const HEIGHT_F: f64 = HEIGHT as f64;

const LIMIT: usize = (WIDTH * HEIGHT) as usize;

fn main() -> Result<(), Error> {

    let rec: Rectangle = Rectangle::new(Vector2::new(0.0f64,0.0f64),Vector2::new(400.0f64,400.0f64));
    let mut qt: Quadtree = Quadtree::new(rec,1);
    let mut bodies: Vec<Body> = Vec::new();
    let mut runner: BarnesHutRunner = BarnesHutRunner::from_theta(0.5f64);
    runner.create_tree(&mut qt, &bodies);
    runner.generate_square(&mut bodies, 200, 100.0f64);



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

    let rec: Rectangle = Rectangle::new(Vector2::new(0.0f64,0.0f64),Vector2::new(WIDTH_F,HEIGHT_F));
    let mut qt: Quadtree = Quadtree::new(rec,1);
    let mut bodies: Vec<Body> = Vec::new();
    let mut runner: BarnesHutRunner = BarnesHutRunner::from_theta(0.5f64);
    // runner.create_tree(&mut qt, &bodies);
    // runner.generate_square(&mut bodies, 200, 100.0f64);
    bodies.push(Body::with_pos(Vector2::new(0.0,0.0)));
    bodies.push(Body::with_pos(Vector2::new(1.0,1.0)));
    bodies.push(Body::with_pos(Vector2::new(2.0,2.0)));
    bodies.push(Body::with_pos(Vector2::new(150.0,150.0)));

    event_loop.run(move |event, _, control_flow| {
        // let test_rec: Rectangle = Rectangle::new(Vector2::new(100.0,100.0), Vector2::new(200.0,200.0));
        // draw_square(&mut my_buffer, &test_rec);

        // runner.create_tree(&mut qt,&bodies);
        // runner.iterate(&mut qt, &mut bodies);
        // recursively_draw_tree(&mut my_buffer, &qt);

        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            runner.iterate(&mut qt, &mut bodies);
            runner.print_bodies(&qt);
            recursively_draw_tree(&mut my_buffer, &qt);
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

            if input.mouse_pressed(0){
                match(input.mouse()){
                    None => {},
                    Some(pos) => {
                        println!("{:?}",pos);
                        bodies.push(Body::with_mass_and_pos(100.0,Vector2::new(pos.0 as f64, pos.1 as f64)));
                    }
                }
            }
            window.request_redraw();
        }


    });

}

fn recursively_draw_tree(buffer: &mut Vec<(u8,u8,u8,u8)>, qt: &Quadtree){
    draw_square(buffer, &qt.boundaries);
    for tree in &qt.subtrees{
        recursively_draw_tree(buffer,&tree);
    }

    match(qt.bodies.is_empty()){
        true => {}
        false => {
            for body in &qt.bodies{
                let point_pos = calculate_buffer_pos(&body.pos);
                buffer[point_pos] = (255,0,0,255);

            }
        }
    }

}

fn draw_square(buffer: &mut Vec<(u8,u8,u8,u8)>, rec: &Rectangle){
    let tl: usize = calculate_buffer_pos(&rec.tl);
    let br: usize = calculate_buffer_pos(&rec.br);
    let tr: usize = tl + rec.width() as usize;
    let bl: usize = br - rec.width() as usize;

    //horizontal lines
    for i in tl..tr{
        buffer[i] = (255,255,255,255);
    }

    for i in bl..br{
        buffer[i] = (255,255,255,255);
    }

    //vertical lines
    for i in (tl..bl).step_by(WIDTH as usize){
        buffer[i] = (255,255,255,255);
    }

    for i in (tr..br).step_by(WIDTH as usize){
        buffer[i] = (255,255,255,255);
    }

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

