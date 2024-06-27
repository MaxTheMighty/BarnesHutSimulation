use cgmath::{Vector2};
use crate::body::Body;

const G: f64 = 5.00;
const DT: f64 = 0.01;

const EPSILON: f64 = 1.0;
//const MIN: f64 = 0.0f64;
pub fn calculate_force(bodies: &mut[Body]){
    let d = bodies[0].pos - bodies[1].pos; //r21
    let d_mag = ((d.x * d.x) + (d.y * d.y) + EPSILON).sqrt();
    // let d_mag = d.magnitude(); // | r21 |
    let force: Vector2<f64> = d * ((G * bodies[0].mass * bodies[1].mass)/(d_mag.powi(3)));
    //TODO bring back the force vector to figure out the issue with DT
    bodies[0].force-=force;
    bodies[1].force+=force;
}

pub fn calculate_force_single(body_a: &mut Body, body_b: &mut Body){
    let d = body_a.pos - body_b.pos; //r21
    let d_mag = ((d.x * d.x) + (d.y * d.y) + EPSILON).sqrt();
    let force: Vector2<f64> = d * ((G * body_a.mass * body_b.mass)/(d_mag.powi(3)));
    body_a.force-=force;
    body_b.force+=force;
}

//Although this is similar to the code above, I wanted to have a different function
//to avoid having to allocate a new body when I want to calculate the force,
//I can also just pass the center and mass fields to this function from a body
//But this can be fixed later
pub fn calculate_force_mass_center(body_a: &mut Body, center: Vector2<f64>, mass: f64){
    let d = body_a.pos - center; //r21
    let d_mag = ((d.x * d.x) + (d.y * d.y) + EPSILON).sqrt();
    let force: Vector2<f64> = d * ((G * body_a.mass * mass)/(d_mag.powi(3)));
    body_a.force-=force;
}

pub fn apply_force( body_a: &mut Body){
    // F = mA -> A = F/m
    body_a.acceleration = body_a.force/body_a.mass;
    body_a.pos += body_a.velocity * DT;
    body_a.velocity += body_a.acceleration * DT;
    //there must be a better way
    body_a.force.x = 0.0f64;
    body_a.force.y = 0.0f64;
}






