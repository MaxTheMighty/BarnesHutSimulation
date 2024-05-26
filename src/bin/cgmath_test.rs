use std::ops::{Div, DivAssign};
use cgmath::Vector2;

fn main() {

    let mut v: Vector2<f64> = Vector2::new(100.0,50.0);
    v.div_assign(2.0);
    println!("{:?}",v);
}