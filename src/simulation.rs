use cgmath::{Vector2};

use crate::body::Body;
use crate::gravity;

#[derive(Debug)]
pub struct Simulation{
    pub bodies: Vec<Body>,
}

impl Simulation {
    pub fn new() -> Self{
        Self {
            bodies: Vec::new(),
        }
    }

    pub fn generate(&mut self, count: u16, offset: f64, area: f64) {
        for _ in 1..count {
            self.bodies.push(Body::random(offset, area));
        }
    }


    pub fn square(&mut self, length: u16, top_left: f64){
        for x in 0..length{
            for y in 0 ..length{
                self.bodies.push(Body::with_mass_and_pos(1.0, Vector2::new(x as f64 + top_left, y as f64 + top_left)));
            }
        }
    }

    pub fn cube(&mut self, length: f64, width: f64, _depth: f64, top_left: f64, count: u16){
        let mut body: Body;
        for _ in 0..count{
            body = Body::new();
            body.pos = Vector2::new(fastrand::f64()*length + top_left, fastrand::f64()*width + top_left);
            body.mass = 1.0;
            self.bodies.push(body);
        }
    }


    pub fn update(&mut self) {
        for i in 0..self.bodies.len() {
            for j in i+1..self.bodies.len() {
                gravity::calculate_force(&mut self.bodies[i..j + 1]);
            }
        }

        for body in self.bodies.iter_mut(){
            gravity::apply_force(body);
        }
    }




}