#![allow(warnings)]

use std::env;
use std::ops::Add;
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
// const DEBUG: bool = true;

const SIM_WIDTH: u32 = 1000;

const SIM_HEIGHT: u32 = 1000;
const SIM_WIDTH_F: f64 = SIM_WIDTH as f64;

const SIM_HEIGHT_F: f64 = SIM_HEIGHT as f64;

const BUFFER_WIDTH: u32 = SIM_WIDTH * 2;
const BUFFER_HEIGHT: u32 = SIM_HEIGHT;

const BUFFER_WIDTH_F: f64 = BUFFER_WIDTH as f64;
const BUFFER_HEIGHT_F: f64 = BUFFER_HEIGHT as f64;

const LIMIT: usize = ((BUFFER_WIDTH + 1) * (BUFFER_HEIGHT + 1 + 1)) as usize;

const OFFSET_VEC: Vector2<f64> = Vector2::new(BUFFER_WIDTH_F/2.0, 0.0);

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
        let size = LogicalSize::new(BUFFER_WIDTH_F, BUFFER_HEIGHT_F);
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
        Pixels::new(BUFFER_WIDTH, BUFFER_HEIGHT, surface_texture)?
    };



    let rec: Rectangle = Rectangle::new(Vector2::new(0.0f64,0.0f64),Vector2::new(SIM_WIDTH_F, SIM_HEIGHT_F));
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
            recursively_draw_tree(&mut my_buffer, &qt);
            recursively_draw_tree_no_box(&mut my_buffer, &qt);
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
                println!("pause set to {:?}",runner.paused);
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
    draw_bodies(buffer,&qt,false);
}
fn recursively_draw_tree(buffer: &mut Vec<(u8,u8,u8,u8)>, qt: &Quadtree){

    draw_square(buffer, &qt.boundaries,true);
    for tree in &qt.subtrees{
        recursively_draw_tree(buffer,&tree);
    }
    // draw_bodies(buffer,&qt.bodies,true);
}

fn draw_bodies(buffer: &mut Vec<(u8,u8,u8,u8)>, qt: &Quadtree, right: bool){
    match(qt.bodies.is_empty()){
        true => {}
        false => {

            for body in &qt.bodies{
                if(!within_buffer(&body.pos)){
                    continue;
                }

                let mut point_pos = 0;
                if(right){
                    point_pos = calculate_buffer_pos(&body.pos.add(OFFSET_VEC));
                } else {
                    point_pos = calculate_buffer_pos(&body.pos);
                }
                // if right{
                //     point_pos
                // }
                update_pixel_heat(buffer, point_pos);

            }
        }
    }
}

fn update_pixel_heat(buffer: &mut Vec<(u8,u8,u8,u8)>, pos: usize){
    if(buffer[pos].3 == 0){
        buffer[pos] = (0,0,255,255);
    } else {

        // buffer[pos].2-=10;
        if(DEBUG) {println!("pos: {pos}, buffer[pos]: {:?}",buffer[pos])}
        buffer[pos].1 = buffer[pos].1.saturating_add(10);

        //only
        // if(buffer[pos].1 > 255){
        //     buffer[pos].1 = 255;
        // } else {
        // }
        // if(buffer[pos].2 < 0){
        //     buffer[pos].2 = 0;
        // }
    }
}

fn draw_square(buffer: &mut Vec<(u8,u8,u8,u8)>, rec: &Rectangle, right: bool){
    let mut tl: usize = 0;
    let mut br: usize = 0;
    let mut tr: usize = 0;
    let mut bl: usize = 0;


    //if the right boolean is true, then add the offset vector defined as (1000,0) to the positions
    //otherwise, calculate the top left and bottom right indexes in the buffer
    //then fill in between with lines
    if(right){
        tl = calculate_buffer_pos(&rec.tl.add(OFFSET_VEC));
        br = calculate_buffer_pos(&rec.br.add(OFFSET_VEC));
        tr = tl + rec.width() as usize;
        bl = br - rec.width() as usize;
    } else {
        tl = calculate_buffer_pos(&rec.tl);
        br = calculate_buffer_pos(&rec.br);
        tr = tl + rec.width() as usize;
        bl = br - rec.width() as usize;
    }



    if(DEBUG) {println!("\nrec: {:?}\ntl: {tl}\nbr: {br}\ntr: {tr}\nbl: {bl}", rec) };
    //horizontal lines

    //top horizontal
    for i in tl..=tr{
        buffer[i] = (255,255,255,255);
    }

    //bottom horizontal
    for i in bl..=br{
        buffer[i] = (255,255,255,255);
    }

    //vertical lines

    //left vertical
    for i in (tl..=bl).step_by(BUFFER_WIDTH as usize){
        buffer[i] = (255,255,255,255);
    }

    //right vertical
    for i in (tr-1..=br-1).step_by(BUFFER_WIDTH as usize){
        buffer[i] = (255,255,255,255);
    }

}


fn within_buffer(pos: &Vector2<f64>) -> bool {
    return pos.x < BUFFER_WIDTH_F && pos.x > 0.0 && pos.y < BUFFER_HEIGHT_F && pos.y > 0.0;
}

fn calculate_buffer_pos(pos: &Vector2<f64>) -> usize{
    return (pos.x + (pos.y.floor() * BUFFER_WIDTH_F)) as usize;
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

