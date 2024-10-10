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
use barnes_hut::canvas::Canvas;
const DEBUG: bool = false;

const WIDTH: u32 = 800;
const WIDTH_F: f64 = WIDTH as f64;
const HEIGHT: u32 = 800;

const HEIGHT_F: f64 = HEIGHT as f64;

const LIMIT: u32 = ((WIDTH + 1)) * ((HEIGHT + 1) );



fn main() -> Result<(), Error> {
    env::set_var("RUST_BACKTRACE", "FULL");
    //pre update
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let mut draw_boxes: bool = false;
    let mut canvas: Canvas = Canvas::new(WIDTH,HEIGHT, (0,0,0,0));
    let window = {
        let size = LogicalSize::new(WIDTH_F, HEIGHT_F);
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

    //TODO: change quadtree vec to smallvec or just an array

    let rec: Rectangle = Rectangle::new(Vector2::new(0.0f64,0.0f64),Vector2::new(WIDTH_F ,HEIGHT_F));
    let mut qt: Quadtree = Quadtree::new(rec,1);
    let mut bodies: Vec<Body> = Vec::new();
    let mut runner: BarnesHutRunner = BarnesHutRunner::from_theta(1.0f64);
    // runner.generate_bivariate_random_dist(&mut bodies, WIDTH_F, HEIGHT_F, 50000, 10.0, 0.5);

    // runner.generate_circle(&mut bodies, 450.0,450.0,50.0);
    // bodies.push(Body::with_mass_and_pos(1.0,Vector2::new((WIDTH_F/2.0) - 75.0,(HEIGHT_F/2.0))));
    // bodies.last_mut().unwrap().velocity.y = -7.0;
    // bodies.last_mut().unwrap().velocity.x = 1.0;
    // bodies.push(Body::with_mass_and_pos(100000.0,Vector2::new(WIDTH_F/2.0,HEIGHT_F/2.0)));
    // runner.generate_square(&mut bodies, 100, 450.0, 450.0);
    runner.resize(&mut qt,&mut bodies);
    println!("{:?}",qt.boundaries);
    runner.create_tree(&mut qt,&mut bodies);
    runner.paused = true;
    println!("{:?}",bodies.len());
    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {

            runner.iterate(&mut qt, &mut bodies);
            if(DEBUG) {runner.print_bodies(&qt);}

            match(draw_boxes){
                true => {recursively_draw_tree(&mut canvas, &qt);},
                false => {recursively_draw_tree_no_box(&mut canvas, &qt);}
            }
            canvas.copy_huemap_to_buffer(pixels.frame_mut());
            canvas.clear();
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
                        bodies.push(Body::with_mass_and_pos(10000.0,Vector2::new(pos.0 as f64, pos.1 as f64)));
                    }
                }
            }
            window.request_redraw();
        }


    });

}

fn recursively_draw_tree_no_box(canvas: &mut Canvas, qt: &Quadtree){
    for tree in &qt.subtrees{
        recursively_draw_tree_no_box(canvas,&tree);
    }
    draw_bodies(canvas,&qt.bodies);
}
fn recursively_draw_tree(canvas: &mut Canvas, qt: &Quadtree){
    canvas.draw_square_safe(qt.boundaries.tl.x.round() as i32, qt.boundaries.tl.y.round() as i32, qt.boundaries.width() as i32, qt.boundaries.height() as i32,&(0.0,0.0,1.0));
    for tree in &qt.subtrees{
        recursively_draw_tree(canvas,&tree);
    }
    // match qt.center_of_mass {
    //     Some(center_of_mass) => {
    //         if(canvas.pos_valid(center_of_mass.x as i32, center_of_mass.y as i32)){
    //             // canvas.set_color(center_of_mass.x as i32, center_of_mass.y as i32, &(255,0,0,255))
    //             // canvas.set_hue(center_of_mass.x as i32, center_of_mass.y as i32, 1.0,1.0,1.0);
    //         } else {
    //             //
    //         }
    //
    //     },
    //     None => {}
    // }

    draw_bodies(canvas,&qt.bodies);

}

fn draw_bodies(canvas: &mut Canvas, bodies: &Vec<Body>){
    match(bodies.is_empty()){
        true => {}
        false => {
            for body in bodies{
                update_pixel_heat(canvas, body); //not a fan of this casting
            }
        }
    }
}

fn update_pixel_heat(canvas: &mut Canvas, body: &Body){
    let x_pos: i32 = body.pos.x.round() as i32;
    let y_pos: i32 = body.pos.y.round() as i32;

    if(!canvas.pos_valid(x_pos,y_pos)){
        return;
    }

    canvas.increment_huemap(body.pos.x as i32, body.pos.y as i32, (240.0,1.0,1.0),-1.0);
}


