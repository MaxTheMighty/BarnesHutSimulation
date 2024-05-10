use cgmath::{Vector2};
use crate::body::Body;

const G: f64 = 1.00;
const DT: f64 = 0.01;

const EPSILON: f64 = 0.5f64;
const MIN: f64 = 1.0f64;
pub fn calculate_force(bodies: &mut[Body]){

    // let p1 = bodies[0].pos;
    // let m1 = bodies[0].mass;
    // let p2 = bodies[1].pos;
    // let m2 = bodies[1].mass;
    //
    // let r = p2 - p1;
    // let mag_sq = r.x * r.x + r.y * r.y;
    // let mag = mag_sq.sqrt();
    // let tmp = r / (mag_sq.max(MIN) * mag);
    // bodies[0].acceleration += m2 * tmp;
    // bodies[1].acceleration -= m1 * tmp;
    let mut dist: Vector2<f64> = bodies[0].pos - bodies[1].pos;
    let dist_mag_square = ((dist.x * dist.x) + (dist.y * dist.y)) + EPSILON;
    let dist_mag = dist_mag_square.sqrt();
    let gravity = (G * dist / (dist_mag * (dist_mag_square)));
    bodies[0].acceleration -= (gravity * bodies[1].mass);
    bodies[1].acceleration += (gravity * bodies[0].mass);
}


pub fn apply_force( body_a: &mut Body){
    body_a.pos += body_a.velocity * DT;
    body_a.velocity += body_a.acceleration * DT;
    //there must be a better way
    body_a.acceleration.x = 0.0f64;
    body_a.acceleration.y = 0.0f64;
}




