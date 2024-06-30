use std::f32::consts::PI;
use std::ops::{Deref, DerefMut};
use crate::quadtree::Quadtree;
use crate::gravity;
use cgmath::{MetricSpace, Vector2};
use crate::body::Body;
pub struct BarnesHutRunner {

    pub theta: f64,
    pub paused: bool
}


impl BarnesHutRunner{
    pub fn new() -> BarnesHutRunner{
        Self {
            theta: 0.5,
            paused: false
        }
    }

    pub fn from_theta(theta: f64) -> BarnesHutRunner {
        Self {
            theta,
            paused: false
        }
    }



    pub fn generate_square(&mut self, bodies: &mut Vec<Body>, length: u16, top_left: f64){
        for x in 0..length{
            for y in 0..length{
                bodies.push(Body::with_mass_and_pos(10.0, Vector2::new(x as f64 + top_left, y as f64 + top_left)));
            }
        }
    }

    pub fn generate_circle(&mut self, bodies: &mut Vec<Body>, center: f64, radius: f64){
        let mut x_pos: f64 = 0.0;
        let mut y_pos: f64 = 0.0;

        for ring in 1..radius as u16{
            for i in 0..360{
                /*
                            x1 = r * cos(angle * PI / 180);
                    y1 = r * sin(angle * PI / 180);

                 */
                x_pos = (radius-ring as f64 * f64::cos(i as f64 * std::f64::consts::PI / 180.0)) + center as f64;
                y_pos = (radius-ring as f64 * f64::sin(i as f64 * std::f64::consts::PI / 180.0)) + center as f64;
                bodies.push(Body::with_mass_and_pos(1.0,Vector2::new(x_pos,y_pos)));
            }
        }

    }

    pub fn barnes_hut_force(&mut self, quadtree: &mut Quadtree, body: &mut Body){
        if quadtree.subtrees.is_empty() {
            //just sum forces from bodies in subtree
            for other_body in quadtree.bodies.iter_mut(){
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
                    for other_subtree in quadtree.subtrees.iter_mut(){
                        self.barnes_hut_force(other_subtree.deref_mut(),body);
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

    pub fn update(&mut self, quadtree: &mut Quadtree, bodies: &mut Vec<Body>){
        for body in bodies.iter_mut(){
            self.barnes_hut_force(quadtree,body);
        }

        for body in bodies.iter_mut(){
            gravity::apply_force(body);
        }
    }

    pub fn iterate(&mut self, quadtree: &mut Quadtree, bodies: &mut Vec<Body>){
        if(self.paused){
            return;
        }

        self.force_iterate(quadtree,bodies);
    }

    pub fn force_iterate(&mut self, quadtree: &mut Quadtree, bodies: &mut Vec<Body>){
        quadtree.clear();
        self.create_tree(quadtree, bodies);
        self.update(quadtree,bodies);
    }

    pub fn toggle_pause(&mut self){
        self.paused = !self.paused;
    }

    pub fn print_bodies(&self, quadtree: &Quadtree){
        for body in &quadtree.bodies{
            println!("{}",body);
        }
        for subtree in &quadtree.subtrees{
            self.print_bodies(&subtree);
        }
    }

}

