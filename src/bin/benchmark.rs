use barnes_hut::bh_runner::BarnesHutRunner;
use barnes_hut::body::Body;
use barnes_hut::quadtree::{Quadtree, Rectangle};
use cgmath::Vector2;

fn main() {
    let rec: Rectangle = Rectangle::new(Vector2::new(0.0f64,0.0f64),Vector2::new(1000.0 ,1000.0));
    let mut qt: Quadtree = Quadtree::new(rec,1);
    let mut bodies: Vec<Body> = Vec::new();
    let mut runner: BarnesHutRunner = BarnesHutRunner::from_theta(0.5f64);
    runner.generate_circle(&mut bodies,450.0,500.0,30.0);
    runner.generate_circle(&mut bodies,550.0,500.0,30.0);
    runner.resize(&mut qt,&mut bodies);
    runner.create_tree(&mut qt,&mut bodies);
    for _ in 1..100{
        runner.iterate(&mut qt,&mut bodies);
    }
}