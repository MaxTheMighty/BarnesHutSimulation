mod quadtree;
mod body;


use cgmath::Vector2;
use barnes_hut::bh_runner::BarnesHutRunner;
use barnes_hut::body::Body;
use barnes_hut::quadtree::{Quadtree, Rectangle};


fn main() {
    // println!("Hello, world!");
    // let vec_one: Vector2<u8> = Vector2::new(0,1);
    // let _vec_two: Vector2<u8> = Vector2::new(2,5);
    // println!("{:?}", vec_one);
    let rec: Rectangle = Rectangle::new(Vector2::new(0.0f64,0.0f64),Vector2::new(400.0f64,400.0f64));
    let mut qt: Quadtree = Quadtree::new(rec,1);
    let mut bodies: Vec<Body> = Vec::new();
    let mut runner: BarnesHutRunner = BarnesHutRunner::from_theta(0.5f64);
    runner.create_tree(&mut qt, &bodies);
    //runner.generate_square(&mut bodies, 200, 100.0f64);
    runner.iterate(&mut qt, &mut bodies);
}
