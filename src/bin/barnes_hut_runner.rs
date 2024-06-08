use barnes_hut::quadtree::Quadtree;
use barnes_hut::body::Body;
use cgmath::{MetricSpace, Vector2};
pub struct BarnesHutRunner {
    pub theta: f64
}


impl BarnesHutRunner{
    pub fn new() -> BarnesHutRunner{
        Self {
            theta: 0.5
        }
    }

    pub fn from_theta(theta: f64) -> BarnesHutRunner {
        Self {
            theta
        }
    }

    pub fn calculate_force(self, qt: &mut Quadtree, body: &mut Body){
        for subtree in qt.subtrees{

            //we have to get both
            //s which is the width of the bounding box (not height)
            //and d which is the distance between the body and the st's com
            match subtree.center_of_mass {
                Some(com) => {
                    let s = subtree.boundaries.

                }
                None () => {
                    continue;
                }
            }
        }
    }

    pub fn update(qt: &mut Quadtree){

    }
}

fn main() {}