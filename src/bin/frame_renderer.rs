use std::error::Error;
use image::{ImageBuffer, Rgb};
use barnes_hut::bh_runner::BarnesHutRunner;
use barnes_hut::body::Body;
use barnes_hut::quadtree::{Quadtree, Rectangle};
use cgmath::Vector2;
use hsv::hsv_to_rgb;
use barnes_hut::canvas::Canvas;

fn main() -> Result<(), image::ImageError> {


    let width = 2000;
    let height = 2000;
    let mut img = ImageBuffer::from_pixel(width, height, Rgb([255u8, 255u8, 255u8]));
    // let mut img = ImageBuffer::new(width, height);

    let rec: Rectangle = Rectangle::new(Vector2::new(0.0f64,0.0f64),Vector2::new(width as f64 ,height as f64));
    let mut qt: Quadtree = Quadtree::new(rec,1);
    let mut bodies: Vec<Body> = Vec::new();
    let mut runner: BarnesHutRunner = BarnesHutRunner::from_theta(1.0f64);
    let mut canvas: Canvas = Canvas::new(width ,height, (0,0,0,0));

    // I set dt in gravity.rs to 0.001 instead of 0.01
    // this makes the sim much slower but more accurate

    runner.generate_bivariate_random_dist(&mut bodies, width as f64, height as f64, 100000, 10.0, 0.75);
    runner.resize(&mut qt,&mut bodies);
    runner.create_tree(&mut qt,&mut bodies);
    runner.paused = false;
    for i in 0..10000{
        canvas.clear();
        runner.iterate(&mut qt, &mut bodies);
        print!("{:?}\n",i);
        draw_bodies(&mut canvas, &bodies);

        // Copy simulation
        for (pixel,hue) in img.pixels_mut().zip(canvas.huemap.iter()) {
            let (h,s,v) = *hue;
            let rgb = hsv_to_rgb(h,s,v);
            *pixel = Rgb([rgb.0 as u8,rgb.1 as u8,rgb.2 as u8]);

        }

        img.save(format!("frames/output{i}.png"))?;
    }
    /*
    ffmpeg -framerate 60 -pattern_type glob -i "output*.png" -c:v libx264 -pix_fmt yuv420p output.mp4
     */

    Ok(())

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

