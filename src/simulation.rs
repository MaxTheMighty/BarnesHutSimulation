use crate::body;
use crate::body::Body;
use crate::gravity;

#[derive(Debug)]
pub struct Simulation{
    pub bodies: Vec<Body>,
    seed: u64
}

impl Simulation {
    pub fn new() -> Self{
        Self {
            bodies: Vec::new(),
            seed: 0
        }
    }

    pub fn generate(&mut self) {
        for _ in 1..1000 {
            self.bodies.push(Body::random());
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