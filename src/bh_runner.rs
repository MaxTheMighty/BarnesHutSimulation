use std::ops::Deref;
use crate::quadtree::Quadtree;
use crate::gravity;
use cgmath::{MetricSpace, Vector2};
use crate::body::Body;
pub struct BarnesHutRunner {

    pub theta: f64,
}


impl BarnesHutRunner{
    pub fn new() -> BarnesHutRunner{
        Self {
            theta: 0.5,
        }
    }

    pub fn from_theta(theta: f64) -> BarnesHutRunner {
        Self {
            theta,

        }
    }



    pub fn generate_square(&mut self, bodies: &mut Vec<Body>, length: u16, top_left: f64){
        for x in 0..length{
            for y in 0..length{
                bodies.push(Body::with_mass_and_pos(1.0, Vector2::new(x as f64 + top_left, y as f64 + top_left)));
            }
        }


    }

    pub fn barnes_hut_force(&mut self, quadtree: &Quadtree, body: &mut Body){
        if quadtree.subtrees.is_empty() {
            //just sum forces from bodies in subtree
            for other_body in quadtree.bodies.iter(){
                //ignore self
                if body.pos == other_body.pos {
                    continue;
                }
                gravity::calculate_force_single(body,other_body);
            }
            return;
        }

        //at this point the node is not external, because it has subtrees
        //so we see if it has a center of mass
        match quadtree.center_of_mass {
            //center of mass exists,
            //calculate distance from body to region of space
            //if far enough we can use its center of mass (COM)
            //if not, we drill down and all of its subtrees
            Some(com) => {
                let s = quadtree.boundaries.width();
                let d = body.pos.distance(com);
                let theta_body = s/d;
                if theta_body > self.theta {
                    //go further into the tree
                    //how can we do this if the runner owns the tree?
                    for other_subtree in &quadtree.subtrees{
                        self.barnes_hut_force(other_subtree.deref(),body);
                    }
                    return;
                } else {
                    //use the COM
                    gravity::calculate_force_mass_center(body,com,quadtree.total_mass);
                    return;
                }
            }
            None => {
                return;
            }
        }

    }

    pub fn create_tree(&mut self, quadtree: &mut Quadtree, bodies: &Vec<Body>){
        //I need to check if creating a new quadtree this way safely disposes of the old one
        //Does this mean the reference only exists for the lifetime of this function?
        // quadtree = &Quadtree::new(quadtree.boundaries, quadtree.limit); //Works because of the Clone trait (￣▽￣)"
        for body in bodies{
            quadtree.insert(body.clone()); //I think clone here makes a new body
        }

        quadtree.update_mass();
    }

    pub fn update(&mut self, quadtree: &Quadtree, bodies: &mut Vec<Body>){
        for body in bodies.iter_mut(){
            self.barnes_hut_force(quadtree,body);
        }

        for body in bodies.iter_mut(){
            gravity::apply_force(body);
        }
    }

    pub fn iterate(&mut self, quadtree: &mut Quadtree, bodies: &mut Vec<Body>){
        self.create_tree(quadtree, bodies);
        self.update(quadtree,bodies);
    }
}

