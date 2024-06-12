mod quadtree;
mod body;


use cgmath::Vector2;
use barnes_hut::bh_runner::BarnesHutRunner;
use barnes_hut::quadtree::Rectangle;
use crate::body::Body;
use crate::quadtree::Quadtree;

fn main() {
    // println!("Hello, world!");
    // let vec_one: Vector2<u8> = Vector2::new(0,1);
    // let _vec_two: Vector2<u8> = Vector2::new(2,5);
    // println!("{:?}", vec_one);
    let rec: Rectangle = Rectangle::new(Vector2::new(0.0f64,0.0f64),Vector2::new(400.0f64,400.0f64));
    let mut qt: Quadtree = Quadtree::new(rec,1);
    let bodies: Vec<Body> = Vec::new();
    let mut runner: BarnesHutRunner = BarnesHutRunner::from_theta_bodies_tree(0.5f64,bodies,qt);
    runner.generate_square(200,100.0f64);
    runner.iterate();
}
