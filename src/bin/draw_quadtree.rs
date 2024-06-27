#![allow(warnings)]

use std::env;
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

const DEBUG: bool = false;

fn main() -> Result<(), Error> {
    env::set_var("RUST_BACKTRACE", "FULL");
    //pre update
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let mut my_buffer: Vec<(u8,u8,u8,u8)> = Vec::new();
    let mut draw_boxes: bool = false;
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

    let rec: Rectangle = Rectangle::new(Vector2::new(0.0f64,0.0f64),Vector2::new(WIDTH_F - 100.0,HEIGHT_F - 100.0));
    let mut qt: Quadtree = Quadtree::new(rec,1);
    let mut bodies: Vec<Body> = Vec::new();
    let mut runner: BarnesHutRunner = BarnesHutRunner::from_theta(0.5f64);
    runner.generate_square(&mut bodies,50,200.0);
    runner.generate_square(&mut bodies,50,600.0);
    // bodies.push(Body::with_mass_and_pos(10.0,Vector2::new(150.0,150.0)));
    // bodies.push(Body::with_mass_and_pos(1000.0,Vector2::new(500.0,50.0)));
    // bodies.push(Body::with_mass_and_pos(1.0,Vector2::new(450.0,100.0)));
    // bodies[1].velocity.y = -1.0;
    // bodies[1].velocity.x = 1.0;
    // bodies.push(Body::with_mass_and_pos(1.0,Vector2::new(1.0,50.0)));
    // bodies.push(Body::with_mass_and_pos(1.0,Vector2::new(1.1,50.0)));

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            runner.iterate(&mut qt, &mut bodies);
            if(DEBUG) {runner.print_bodies(&qt);}
            match(draw_boxes){
                true => {recursively_draw_tree(&mut my_buffer, &qt);},
                false => {recursively_draw_tree_no_box(&mut my_buffer, &qt);}
            }
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

            if input.key_pressed(VirtualKeyCode::Space){
                draw_boxes = !draw_boxes;
                println!("draw boxes set to {draw_boxes}");
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

fn recursively_draw_tree_no_box(buffer: &mut Vec<(u8,u8,u8,u8)>, qt: &Quadtree){
    // draw_square(buffer, &qt.boundaries);

    for tree in &qt.subtrees{
        recursively_draw_tree_no_box(buffer,&tree);
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

