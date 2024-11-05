use cgmath::{MetricSpace, Vector2};

fn main() {

    let v: Vector2<f64> = Vector2::new(100.0,50.0);
    let x: Vector2<f64> = Vector2::new(200.0,20.0);
    println!("{:?}",v.distance(x));

}