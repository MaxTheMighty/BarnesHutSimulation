
use std::env;
use cgmath::Vector2;
use criterion::{criterion_group, criterion_main, Criterion};
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

const WIDTH: u32 = 1000;
const WIDTH_F: f64 = WIDTH as f64;
const HEIGHT: u32 = 1000;

const HEIGHT_F: f64 = HEIGHT as f64;

const LIMIT: u32 = ((WIDTH + 1)) * ((HEIGHT + 1) );
fn criterion_benchmark(c: &mut Criterion) {
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
        Pixels::new(WIDTH, HEIGHT, surface_texture)
    }.unwrap();

    //TODO: change quadtree vec to smallvec or just an array

    let rec: Rectangle = Rectangle::new(Vector2::new(0.0f64,0.0f64),Vector2::new(WIDTH_F ,HEIGHT_F));
    let mut qt: Quadtree = Quadtree::new(rec,1);
    let mut bodies: Vec<Body> = Vec::new();
    let mut runner: BarnesHutRunner = BarnesHutRunner::from_theta(1.0f64);
    // runner.generate_bivariate_random_dist(&mut bodies, WIDTH_F/2.0, HEIGHT_F/2.0, 40000, 10.0, WIDTH_F/4.0);
    // runner.generate_circle(&mut bodies, 450.0,450.0,50.0);
    // bodies.push(Body::with_mass_and_pos(100000.0,Vector2::new(WIDTH_F,HEIGHT_F)));
    runner.generate_square(&mut bodies, 100, 450.0, 450.0);
    runner.resize(&mut qt,&mut bodies);
    runner.create_tree(&mut qt,&mut bodies);
    // runner.toggle_pause();
    println!("{:?}",bodies.len());
    c.bench_function("draw 100", |b| {
        b.iter(|| {
            runner.iterate(&mut qt, &mut bodies);

            match (draw_boxes) {
                true => { recursively_draw_tree(&mut canvas, &qt); },
                false => { recursively_draw_tree_no_box(&mut canvas, &qt); }
            }
            canvas.copy_huemap_to_buffer(pixels.frame_mut());
            canvas.clear();

        });

    } );
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




criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
