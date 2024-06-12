use std::ops::Deref;
use crate::quadtree::{Quadtree, Rectangle};
use crate::gravity;
use cgmath::{MetricSpace, Vector2};
use crate::gravity::calculate_force;
use crate::body::Body;
pub struct BarnesHutRunner {

    pub theta: f64,
    pub bodies: Vec<Body>,
    pub quadtree: Quadtree,
}


impl BarnesHutRunner{
    pub fn new() -> BarnesHutRunner{
        Self {
            theta: 0.5,
            bodies: Vec::new(),
            quadtree: Quadtree::new(Rectangle::new(Vector2::new(0.0f64,0.0f64),Vector2::new(1000.0f64,1000.0f64)), 1)
        }
    }

    pub fn from_theta(theta: f64) -> BarnesHutRunner {
        Self {
            theta,
            bodies: Vec::new(),
            quadtree: Quadtree::new(Rectangle::new(Vector2::new(0.0f64,0.0f64),Vector2::new(1000.0f64,1000.0f64)), 1)

        }
    }

    pub fn from_theta_bodies(theta: f64, bodies: Vec<Body>) -> BarnesHutRunner{
        Self{
            theta,
            bodies,
            quadtree: Quadtree::new(Rectangle::new(Vector2::new(0.0f64,0.0f64),Vector2::new(1000.0f64,1000.0f64)), 1)
        }
    }

    pub fn from_theta_bodies_tree(theta: f64, bodies: Vec<Body>, quadtree: Quadtree) -> BarnesHutRunner{
        Self {
            theta,
            bodies,
            quadtree
        }
    }

    pub fn generate_square(&mut self, length: u16, top_left: f64){
        for x in 0..length{
            for y in 0..length{
                self.bodies.push(Body::with_mass_and_pos(1.0, Vector2::new(x as f64 + top_left, y as f64 + top_left)));
            }
        }


    }

    pub fn barnes_hut_force(&mut self, body: &mut Body){
        if(self.quadtree.subtrees.is_empty()){
            //just sum forces from bodies in subtree
            for other_body in self.quadtree.bodies.iter(){
                //ignore self
                if(body.pos == other_body.pos) {
                    continue;
                }
                gravity::calculate_force_single(body,other_body);
            }
            return;
        }

        //at this point the node is not external, because it has subtrees
        //so we see if it has a center of mass
        match self.quadtree.center_of_mass {
            //center of mass exists,
            //calculate distance from body to region of space
            //if far enough we can use its center of mass (COM)
            //if not, we drill down and all of its subtrees
            Some(com) => {
                let s = self.quadtree.boundaries.width();
                let d = body.pos.distance(com);
                let theta_body = s/d;
                if(theta_body > self.theta){
                    //go further into the tree
                    //how can we do this if the runner owns the tree?
                    for other_subtree in self.quadtree.subtrees{
                        let x = other_subtree.deref();

                    }
                    return;
                } else {
                    //use the COM
                    gravity::calculate_force_mass_center(body,com,self.quadtree.total_mass);
                    return;
                }
            }
            None => {
                return;
            }
        }

    }

    pub fn create_tree(&mut self){
        //I need to check if creating a new quadtree this way safely disposes of the old one
        self.quadtree = Quadtree::new(self.quadtree.boundaries,self.quadtree.limit); //Works because of the Clone trait (￣▽￣)"
        for body in self.bodies{
            self.quadtree.insert(body.clone()); //I think clone here makes a new body
        }

        self.quadtree.update_mass();
    }

    pub fn update(&mut self){
        for body in self.bodies.iter_mut(){
            self.barnes_hut_force(body);
        }

        for body in self.bodies.iter_mut(){
            gravity::apply_force(body);
        }
    }

    pub fn iterate(&mut self){
        self.create_tree();
        self.update();
    }
}

fn main() {}