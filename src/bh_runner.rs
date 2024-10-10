use std::f32::consts::PI;
use std::ops::{Deref, DerefMut};
use crate::quadtree::{Quadtree, Rectangle};
use crate::gravity;
use cgmath::{MetricSpace, Vector2};
use crate::body::Body;
use rand::prelude::*;
use rand_distr::{Distribution, Normal, StandardNormal};
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



    pub fn generate_square(&mut self, bodies: &mut Vec<Body>, length: u16, x_top_left: f64, y_top_left: f64){
        for x in 0..length{
            for y in 0..length{
                bodies.push(Body::with_mass_and_pos(1.0, Vector2::new(x as f64 + x_top_left, y as f64 + y_top_left)));
            }
        }
    }

    pub fn generate_circle(&mut self, bodies: &mut Vec<Body>, x_center: f64, y_center: f64, radius: f64) {
        let mut x_pos: f64 = 0.0;
        let mut y_pos: f64 = 0.0;

        for ring in 1..radius as u16 {
            for i in 0..360 {
                x_pos = (radius - ring as f64 * f64::cos(i as f64 * std::f64::consts::PI / 180.0)) + x_center;
                y_pos = (radius - ring as f64 * f64::sin(i as f64 * std::f64::consts::PI / 180.0)) + y_center;
                bodies.push(Body::with_mass_and_pos(1.0, Vector2::new(x_pos, y_pos)));
            }
        }
    }

    pub fn generate_bivariate_random_dist(&mut self, bodies: &mut Vec<Body>, width: f64, height: f64, body_count: i32, body_mass: f64, spread: f64) {
        let mut rng = thread_rng();


        // Calculate the center of the space
        let center_x = width / 2.0;
        let center_y = height / 2.0;

        // Define the standard deviation based on the spread parameter
        let std_dev = spread * width / 6.0;

        // Create normal distributions centered at the middle of the space
        let x_dist = Normal::new(center_x, std_dev).unwrap();
        let y_dist = Normal::new(center_y, std_dev).unwrap();

        for _ in 0..body_count {
            let x = x_dist.sample(&mut rng).clamp(0.0, width);
            let y = y_dist.sample(&mut rng).clamp(0.0, height);

            bodies.push(Body::with_mass_and_pos(body_mass, Vector2::new(x, y)));
        }
    }


    pub fn barnes_hut_force(&mut self, quadtree: &mut Quadtree, body: &mut Body){
        if quadtree.subtrees.is_empty() {
            // just sum forces from bodies in subtree
            for other_body in quadtree.bodies.iter_mut(){
                //ignore self
                if body.pos == other_body.pos {
                    continue;
                }
            }
            // quadtree.bodies.par_iter_mut().for_each(|other_body|{
            //     if body.pos != other_body.pos {
            //         gravity::calculate_force_single(body,other_body);
            //     }
            // })

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

    pub fn resize(&mut self, quadtree: &mut Quadtree, bodies: &mut Vec<Body>){
        //can we just edit in place?
        let mut smallest: f64 = quadtree.boundaries.tl.x;
        let mut largest: f64 = quadtree.boundaries.br.x;
        for body in bodies{
            if body.pos.x > largest{
                largest = body.pos.x;
            }

            if body.pos.x < smallest{
                smallest = body.pos.x;
            }

            if body.pos.y > largest{
                largest = body.pos.y;
            }

            if body.pos.y < smallest{
                smallest = body.pos.y;
            }
        }

        quadtree.boundaries.tl.x = smallest;
        quadtree.boundaries.tl.y = smallest;
        quadtree.boundaries.br.x = largest;
        quadtree.boundaries.br.y = largest;
    }


    pub fn iterate(&mut self, quadtree: &mut Quadtree, bodies: &mut Vec<Body>){
        if(self.paused){
            return;
        }

        self.force_iterate(quadtree,bodies);

    }

    pub fn force_iterate(&mut self, quadtree: &mut Quadtree, bodies: &mut Vec<Body>){
        quadtree.clear();
        self.resize(quadtree,bodies);
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

#[cfg(test)]
mod tests{
    use cgmath::Vector2;
    use crate::bh_runner::BarnesHutRunner;
    use crate::body::Body;
    use crate::quadtree::{Quadtree, Rectangle};

    #[test]
    fn test_resize(){
        let rec: Rectangle = Rectangle::new(Vector2::new(0.0f64,0.0f64),Vector2::new(100.0f64,100.0f64));
        let mut qt: Quadtree = Quadtree::new(rec,1);
        let mut bodies: Vec<Body> = Vec::new();
        let mut runner: BarnesHutRunner = BarnesHutRunner::from_theta(0.5f64);
        // runner.insert(Body::with_pos(Vector2::new(1.0,1.0)));
        bodies.push(Body::with_pos(Vector2::new(110.0,1.0)));
        bodies.push(Body::with_pos(Vector2::new(1.0,150.0)));
        bodies.push(Body::with_pos(Vector2::new(-10.0,120.0)));
        bodies.push(Body::with_pos(Vector2::new(-10.0,-20.0)));
        runner.resize(&mut qt, &mut bodies);
        runner.create_tree(&mut qt, &mut bodies);
        assert_eq!(qt.boundaries.tl.x,-20.0);
        assert_eq!(qt.boundaries.tl.y,-20.0);
        assert_eq!(qt.boundaries.br.x,150.0);
        assert_eq!(qt.boundaries.br.y,150.0);
        for st in qt.subtrees{
            println!("{:?}",st.boundaries);
        }
    }

}

/*

 */

