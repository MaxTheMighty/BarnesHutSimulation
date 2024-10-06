use cgmath::Vector2;
use barnes_hut::bh_runner::BarnesHutRunner;
use barnes_hut::body::Body;
use barnes_hut::quadtree::{Quadtree, Rectangle};

fn main() {
    let rec: Rectangle = Rectangle::new(Vector2::new(0.0f64,0.0f64),Vector2::new(1000.0 ,1000.0));
    let mut qt: Quadtree = Quadtree::new(rec,1);
    let mut bodies: Vec<Body> = Vec::new();
    let mut runner: BarnesHutRunner = BarnesHutRunner::from_theta(1.0f64);
    bodies.push(Body::with_mass_and_pos(1000.0,Vector2::new(100.0, 100.0)));
    bodies.push(Body::with_mass_and_pos(1000.0,Vector2::new(400.0, 400.0)));
    bodies.push(Body::with_mass_and_pos(500.0,Vector2::new(102.0, 103.0)));
    bodies.push(Body::with_mass_and_pos(70000.0,Vector2::new(500.0, 500.0)));
    bodies.push(Body::with_mass_and_pos(7000.0,Vector2::new(200.0, 200.0)));

    runner.resize(&mut qt,&mut bodies);
    runner.create_tree(&mut qt,&mut bodies);
    // runner.iterate(&mut qt, &mut bodies);
    println!("COM: {:?}",qt.center_of_mass);

}