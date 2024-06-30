#![allow(warnings)]

use std::env;
use cgmath::Vector2;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use barnes_hut::bh_runner::BarnesHutRunner;
use barnes_hut::body::Body;
use barnes_hut::quadtree::{Quadtree, Rectangle};
const DEBUG: bool = false;

const WIDTH: u32 = 1000;
const WIDTH_F: f64 = WIDTH as f64;
const HEIGHT: u32 = 1000;

const HEIGHT_F: f64 = HEIGHT as f64;

const LIMIT: usize = ((WIDTH + 1) * (HEIGHT + 1)) as usize;



fn main() -> Result<(), Error> {
    env::set_var("RUST_BACKTRACE", "FULL");
    //pre update
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let mut my_buffer: Vec<(u8,u8,u8,u8)> = Vec::new();
    let mut draw_boxes: bool = false;
    my_buffer.resize(LIMIT, (0,0,0,255));
    my_buffer.resize(LIMIT, (0,0,0,255));
    let window = {
        let size = LogicalSize::new(WIDTH as f64 * 2.0, HEIGHT as f64);
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

    let rec: Rectangle = Rectangle::new(Vector2::new(0.0f64,0.0f64),Vector2::new(WIDTH_F ,HEIGHT_F));
    let mut qt: Quadtree = Quadtree::new(rec,1);
    let mut bodies: Vec<Body> = Vec::new();
    let mut runner: BarnesHutRunner = BarnesHutRunner::from_theta(0.5f64);
    runner.generate_circle(&mut bodies,350.0,15.0);
    runner.generate_circle(&mut bodies,550.0,15.0);
    runner.create_tree(&mut qt,&mut bodies);
    runner.toggle_pause();
    println!("{:?}",bodies.len());
    // return Ok(());
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

            if input.key_pressed(VirtualKeyCode::P){
                runner.toggle_pause();
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
    for tree in &qt.subtrees{
        recursively_draw_tree_no_box(buffer,&tree);
    }
    draw_bodies(buffer,&qt.bodies);
}
fn recursively_draw_tree(buffer: &mut Vec<(u8,u8,u8,u8)>, qt: &Quadtree){

    draw_square(buffer, &qt.boundaries);
    for tree in &qt.subtrees{
        recursively_draw_tree(buffer,&tree);
    }
    draw_bodies(buffer,&qt.bodies);
}

fn draw_bodies(buffer: &mut Vec<(u8,u8,u8,u8)>, bodies: &Vec<Body>, right: bool){
    match(bodies.is_empty()){
        true => {}
        false => {
            for body in bodies{
                if(!within_buffer(&body.pos)){
                    continue;
                }
                let point_pos = calculate_buffer_pos(&body.pos);
                if right{
                    point_pos
                }
                update_pixel_heat(buffer, point_pos);

            }
        }
    }
}

fn update_pixel_heat(buffer: &mut Vec<(u8,u8,u8,u8)>, pos: usize){
    if(buffer[pos].3 == 0){
        buffer[pos] = (0,0,255,255);
    } else {
        buffer[pos].1+=10;
        // buffer[pos].2-=10;
        if(buffer[pos].1 > 255){
            buffer[pos].1 = 255;
        }
        // if(buffer[pos].2 < 0){
        //     buffer[pos].2 = 0;
        // }
    }
}

fn draw_square(buffer: &mut Vec<(u8,u8,u8,u8)>, rec: &Rectangle){
    let tl: usize = calculate_buffer_pos(&rec.tl);
    let br: usize = (calculate_buffer_pos(&rec.br));
    let tr: usize = tl + rec.width() as usize;
    let bl: usize = (br - rec.width() as usize);

    if(DEBUG) {println!("\nrec: {:?}\ntl: {tl}\nbr: {br}\ntr: {tr}\nbl: {bl}", rec) };
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


fn within_buffer(pos: &Vector2<f64>) -> bool {
    return pos.x < WIDTH_F && pos.x > 0.0 && pos.y < HEIGHT_F && pos.y > 0.0;
}

fn calculate_buffer_pos(pos: &Vector2<f64>) -> usize{
    return (pos.x + (pos.y.floor() * WIDTH_F)) as usize;
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

