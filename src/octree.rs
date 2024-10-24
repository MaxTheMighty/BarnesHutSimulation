use glam::Vec3;
use crate::body::Body;

#[derive(Debug)]
pub struct Octree{
    pub top_left_back_corner:[f32;3],
    pub bottom_right_front_corner:[f32;3],
    pub body: Option<Body>,
    pub subregions: Option<[Box<Octree>;8]>,
    pub center:[f32;3],
    pub center_of_mass:Vec3,
    pub total_mass: f32,
}


impl Octree{
    pub fn new(left_top_back_corner: [f32;3], right_bottom_front_corner: [f32;3]) -> Octree{
        Self {
            top_left_back_corner,
            bottom_right_front_corner,
            body: None,
            subregions: None,
            center: Self::calculate_center(left_top_back_corner, right_bottom_front_corner),
            center_of_mass: Vec3::new(0.0,0.0,0.0),
            total_mass: 0.0,

        }
    }


    pub fn calculate_center(top_left_back_corner: [f32;3], bottom_right_front_corner: [f32;3]) -> [f32;3] {
        let width_h: f32 = (bottom_right_front_corner[0] - top_left_back_corner[0]).abs() / 2.0;
        let height_h: f32 = (bottom_right_front_corner[1] - top_left_back_corner[1]).abs() / 2.0;
        let depth_h: f32 = (bottom_right_front_corner[2] - top_left_back_corner[2]).abs() / 2.0;
        let center: [f32;3] = [top_left_back_corner[0] + width_h,top_left_back_corner[1] + height_h,top_left_back_corner[2] + depth_h];
        return center;
    }

    pub fn subdivide(&mut self){
        // this assumes the octs are perfect cubes and will not be negative
        // the simulation will not scale this time
        // we will think of other ways to solve this
        let length: f32 = (self.bottom_right_front_corner[0] - self.top_left_back_corner[0]).abs() / 2.0;
        // let left_bottom_front_cube: [[f32;3];2] = [
        //     [self.top_left_back_corner[0],],
        //     [] ];
    }


}



#[cfg(test)]
mod tests{
    use crate::octree::Octree;
    #[test]
    fn oct_center(){
        let tlbc = [5.0,5.0,5.0];
        let brfc = [10.0,10.0,10.0];
        let center:[f32;3] = Octree::calculate_center(tlbc,brfc);
        assert!(center==[7.5,7.5,7.5]);
        assert!(Octree::calculate_center([0.0,0.0,0.0],[20.0,20.0,20.0]) == [10.0,10.0,10.0]);
    }
}