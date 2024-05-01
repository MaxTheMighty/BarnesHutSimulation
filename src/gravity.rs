use std::ops::Add;
use cgmath::{InnerSpace, Vector2};
use cgmath::num_traits::Pow;
use crate::body::Body;

const G: f32 = 1.00;
const DT: f32 = 0.5;

const MIN: f32 = 0.05;
pub fn calculate_force(bodies: &mut[Body]){

    let mut dist: Vector2<f32> = bodies[0].pos - bodies[1].pos;
    // let dist_mag: f32 = dist.magnitude();
    let p1 = bodies[0].pos;
    let m1 = bodies[0].mass;
    let p2 = bodies[1].pos;
    let m2 = bodies[1].mass;

    let r = p2 - p1;
    let mag_sq = r.x * r.x + r.y * r.y;
    let mag = mag_sq.sqrt();
    let tmp = r / (mag_sq.max(MIN) * mag);
    bodies[0].acceleration += m2 * tmp;
    bodies[1].acceleration -= m1 * tmp;
    // let dist_mag_square = dist.x * dist.x + dist.y * dist.y;
    // let dist_mag = dist_mag_square.sqrt();
    // let gravity = (G * dist / (dist_mag * (dist_mag_square).max(MIN)));
    // bodies[0].acceleration -= gravity * bodies[1].mass;
    // bodies[1].acceleration += gravity * bodies[0].mass;
}


pub fn apply_force( body_a: &mut Body){
    body_a.pos += body_a.velocity * DT;
    body_a.velocity += body_a.acceleration * DT;
    //there must be a better way
    body_a.acceleration.x = 0.0f32;
    body_a.acceleration.y = 0.0f32;
}

pub fn calculate_force_to_indexes(bodies: &mut Vec<Body>,body_a_i: usize, body_b_i: usize){
    // calculate_force(&mut bodies[body_a_i],&mut bodies[body_b_i]);
}



