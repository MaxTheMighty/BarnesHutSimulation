use glam::Vec3;
use crate::body::Body;

#[derive(Debug)]
pub struct Octree{
    pub left_top_back_corner:Vec3,
    pub right_bottom_front_corner:Vec3,
    pub body: Option<Body>,
    pub subregions: Option<[Box<Octree>;8]>,
    pub center:Vec3,
    pub center_of_mass:Vec3,
    pub total_mass: f32,
}


impl Octree{
    pub fn new(corners: [Vec3;2]) -> Octree{
        Self {
            left_top_back_corner: corners[0],
            right_bottom_front_corner: corners[1],
            body: None,
            subregions: None,
            center: Self::calculate_center(corners[0], corners[1]),
            center_of_mass: Vec3::new(0.0,0.0,0.0),
            total_mass: 0.0,

        }
    }


    pub fn calculate_center(top_left_back_corner: Vec3, bottom_right_front_corner: Vec3) -> Vec3 {
        // let differences = (bottom_right_front_corner + top_left_back_corner) * 0.5;
        // let center: Vec3 = Vec3::new(top_left_back_corner[0] + differences.x,top_left_back_corner[1] + differences.y,top_left_back_corner[2] + differences.z);
        // return center;
        return (bottom_right_front_corner + top_left_back_corner) * 0.5;
    }

    pub fn pos_within(&self, pos: Vec3) -> bool{
        return pos.x >= self.left_top_back_corner.x && pos.x < self.right_bottom_front_corner.x &&
            pos.y >= self.left_top_back_corner.y && pos.y < self.right_bottom_front_corner.y &&
            pos.z >= self.left_top_back_corner.z && pos.z < self.right_bottom_front_corner.z;
    }

    pub fn subdivide(&mut self){
        // this assumes the octs are perfect cubes and will not be negative
        // the simulation will not scale this time
        // we will think of other ways to solve this
        // let length: f32 = (self.right_bottom_front_corner.x - self.left_top_back_corner.x).abs() / 2.0;


        self.subregions = Some([
            //A
            Box::new(Octree::new(
                [Vec3::new(self.left_top_back_corner.x, self.right_bottom_front_corner.y, self.center.z),
                    Vec3::new(self.center.x, self.center.y, self.right_bottom_front_corner.z)])),

            //B
            Box::new(Octree::new(
                [
                    Vec3::new(self.center.x, self.left_top_back_corner.y, self.center.z),
                    Vec3::new(self.right_bottom_front_corner.x, self.center.y, self.right_bottom_front_corner.z)
                ]
            )),

            //C
            Box::new(Octree::new(
                [
                    Vec3::new(self.left_top_back_corner.x, self.center.y, self.center.z),
                    Vec3::new(self.center.x, self.right_bottom_front_corner.y, self.right_bottom_front_corner.z)
                ]
            )),

            //D
            Box::new(Octree::new(
                [
                    Vec3::new(self.center.x, self.center.y, self.center.z),
                    Vec3::new(self.right_bottom_front_corner.x, self.left_top_back_corner.y, self.right_bottom_front_corner.z)
                ]
            )),

            //E
            Box::new(Octree::new(
                [
                    Vec3::new(self.left_top_back_corner.x, self.left_top_back_corner.y, self.left_top_back_corner.z),
                    Vec3::new(self.center.x, self.center.y, self.center.z)
                ]
            )),

            //F
            Box::new(Octree::new(
                [
                    Vec3::new(self.center.x, self.left_top_back_corner.y, self.left_top_back_corner.z),
                    Vec3::new(self.right_bottom_front_corner.x, self.center.y, self.center.z)
                ]
            )),

            //G
            Box::new(Octree::new(
                [
                    Vec3::new(self.left_top_back_corner.x, self.center.y, self.left_top_back_corner.z),
                    Vec3::new(self.center.x, self.right_bottom_front_corner.y, self.center.z)
                ]
            )),

            //H
            Box::new(Octree::new(
                [
                    Vec3::new(self.center.x, self.center.y, self.left_top_back_corner.z),
                    Vec3::new(self.right_bottom_front_corner.x, self.right_bottom_front_corner.y, self.center.z)
                ]
            ))
        ]);
    }


}



#[cfg(test)]
mod tests{
    use crate::octree::Octree;
    use glam::{ Vec3};
    #[test]
    fn oct_center(){
        // let tlbc = [5.0,5.0,5.0];
        // let brfc = [10.0,10.0,10.0];
        let corners = [
          Vec3::new(5.0,5.0,5.0),
            Vec3::new(10.0,10.0,10.0),
        ];
        let center = Octree::calculate_center(Vec3::new(5.0,5.0,5.0), Vec3::new(10.0,10.0,10.0));
        let expected_center = Vec3::new(7.5,7.5,7.5);
        assert_eq!(center, expected_center);
        // assert!(Octree::calculate_center([0.0,0.0,0.0],[20.0,20.0,20.0]) == [10.0,10.0,10.0]);
    }

    #[test]
    fn oct_subdivide(){

    }


    #[test]
    fn pos_within(){
        let tree: Octree = Octree::new([Vec3::new(0.0,0.0,0.0),Vec3::new(10.0,10.0,10.0)]);
        let position_a: Vec3 = Vec3::new(5.0,5.0,5.0);
        assert_eq!(tree.pos_within(position_a), true);
        assert_eq!(tree.pos_within(Vec3::new(0.0,0.0,0.0)), true);
        assert_eq!(tree.pos_within(Vec3::new(10.0,10.0,10.0)), false);
        assert_eq!(tree.pos_within(Vec3::new(11.0,10.0,10.0)), false);
        assert_eq!(tree.pos_within(Vec3::new(9.9,9.9,11.0)), false);
    }
}