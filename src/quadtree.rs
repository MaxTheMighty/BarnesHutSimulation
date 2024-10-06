use cgmath::{Vector2};
use crate::body::Body;

const A: usize = 0;
const B: usize = 1;
const C: usize = 2;
const D: usize = 3;

const MIN_SIZE: f64 = 1.0;

#[derive(PartialEq,Debug,Clone,Copy)]
pub struct Rectangle {
    pub tl: Vector2<f64>,
    pub br: Vector2<f64>
}
#[derive(Debug)]
pub struct Quadtree{
    pub boundaries: Rectangle,
    pub limit: usize,
    pub subtrees: Vec<Box<Quadtree>>,
    pub bodies: Vec<Body>,
    pub center_of_mass: Option<Vector2<f64>>,
    pub total_mass: f64
}


impl Rectangle{
    pub fn new(tl: Vector2<f64>, br: Vector2<f64>) -> Self {
        Self {
            tl,
            br
        }
    }
    pub fn within(&self, obj_pos: Vector2<f64>) -> bool {
        return  obj_pos.x >= self.tl.x &&
                obj_pos.x < self.br.x &&
                obj_pos.y >= self.tl.y &&
                obj_pos.y < self.br.y
    }



    pub fn midpoint(&self) -> Vector2<f64> {
        return Vector2::new(self.tl.x + ((self.br.x-self.tl.x)/2.0f64),self.tl.y + ((self.br.y-self.tl.y)/2.0f64));
    }

    pub fn width(&self) -> f64 {
        return self.br.x - self.tl.x;
    }

    pub fn height(&self) -> f64 {
        return self.br.y - self.tl.y;
    }




    pub fn subranges(&self) -> (Rectangle,Rectangle,Rectangle,Rectangle) {
        let midpoint: Vector2<f64> = self.midpoint();
        let subrect_b_tl = Vector2::new(midpoint.x,self.tl.y);
        let subrect_b_br = Vector2::new(self.br.x,midpoint.y);
        let subrect_c_tl = Vector2::new(self.tl.x,midpoint.y);
        let subrect_c_br = Vector2::new(midpoint.x,self.br.y);

        let subrect_a = Rectangle::new(self.tl,midpoint);
        let subrect_b = Rectangle::new(subrect_b_tl,subrect_b_br);
        let subrect_c = Rectangle::new(subrect_c_tl,subrect_c_br);
        let subrect_d = Rectangle::new(midpoint,self.br);
        return (subrect_a,subrect_b,subrect_c,subrect_d);
    }

}


impl Quadtree {
    pub fn new(boundaries: Rectangle, limit: usize) -> Self {
        Self {
            boundaries,
            limit,
            subtrees: Vec::new(),
            bodies: vec![],
            center_of_mass: None,
            total_mass: 0.0,
        }
    }

    pub fn clear(&mut self){
        self.bodies.clear();
        for subtree in self.subtrees.iter_mut(){
            subtree.clear();
        }
        self.subtrees.clear();

    }






    pub fn split(&mut self){
        //split into four
        if self.subtrees.len() != 0 {
            return;
        }

        let subranges = self.boundaries.subranges();
        self.subtrees.push(Box::new(Quadtree::new(subranges.0,self.limit)));
        self.subtrees.push(Box::new(Quadtree::new(subranges.1,self.limit)));
        self.subtrees.push(Box::new(Quadtree::new(subranges.2,self.limit)));
        self.subtrees.push(Box::new(Quadtree::new(subranges.3,self.limit)));
    }

    pub fn insert(&mut self, body: Body) {
        //after we insert the body, check if we need to split
        if self.subtrees.len() == 0 {
            self.bodies.push(body);
        } else {
            match self.subtree_index(body.pos){
                Some(i) => {self.subtrees[i].insert(body)},
                None => {return;}
            }
        }


        let mut body_opt: Option<Body>;
        //if we split
        if self.bodies.len() > self.limit {

            //so we dont keep dividing and get a stack overflow error
            if self.boundaries.width() <= MIN_SIZE {
                self.bodies.push(body);
                return;
            }
            self.split();
            while !self.bodies.is_empty() {
                body_opt = self.bodies.pop();
                match body_opt {
                    Some(body) => {self.insert(body)},
                    None => {}
                }
            }
        }
    }

    pub fn update_mass(&mut self){
        //base case
        if self.subtrees.is_empty() {
            self.calculate_center_leaf();
            return;
        }

        //we aren't at the bottom, so we go further down
        for subtree in &mut self.subtrees{
            subtree.update_mass();
        }

        //then on the way up we calculate the center based on its children
        self.calculate_center_node();

    }



    pub fn calculate_center_leaf(&mut self){
        if self.bodies.is_empty(){
            self.center_of_mass = None;
            return;
        }

        //if we get to this point then this leaf has some bodies
        //if the center of mass doesnt exist then we create it
        match self.center_of_mass{
            Some(_) => {},
            None => { self.center_of_mass = Some(Vector2::new(0.0,0.0));}
        }

        self.total_mass = 0.0f64;
        //then update the center of mass
        for body in &self.bodies{
            /*
            Formally, if two bodies have positions (x1 , y1) and (x2, y2), and masses m1 and m2, then their total mass and center of mass (x, y) are given by:
            m = m1 + m2
            x = (x1m1 + x2m2) / m
            y = (y1m1 + y2m2) / m
             */

            // self.center_of_mass.as_mut().unwrap().x= ((body.pos.x*body.mass) + (self.center_of_mass.unwrap().x * self.total_mass))/self.total_mass;
            // self.center_of_mass.as_mut().unwrap().y= ((body.pos.y*body.mass) + (self.center_of_mass.unwrap().y * self.total_mass))/self.total_mass;
            // self.total_mass+=body.mass;
            // self.

            //is this valid?
            self.center_of_mass = Some(self.center_between_two_points(self.center_of_mass.unwrap(), self.total_mass, body.pos, body.mass));
            self.total_mass+=body.mass;

        }

        //TODO: FIX!!!!!!
        // self.center_of_mass.as_mut().unwrap().x/=self.total_mass;
        // self.center_of_mass.as_mut().unwrap().y/=self.total_mass;

    }

    pub fn calculate_center_node(&mut self){
        let mut non_empty_count: usize = 0;
        match self.center_of_mass{
            Some(_) => {},
            None => { self.center_of_mass = Some(Vector2::new(0.0,0.0));}
        }
        for subtree in &self.subtrees{
            match subtree.center_of_mass {
                Some(center) => {
                    non_empty_count+=1;
                    // self.total_mass += subtree.total_mass;
                    // self.center_of_mass.as_mut().unwrap().x = (center.x * subtree.total_mass);
                    // self.center_of_mass.as_mut().unwrap().y = (center.y * subtree.total_mass);
                    // self.center_of_mass.as_mut().unwrap().x = (center.x * subtree.total_mass) + (self.center_of_mass.unwrap().x * self.total_mass);

                    //is this valid?
                    self.center_of_mass = Some(self.center_between_two_points(self.center_of_mass.unwrap(),self.total_mass,center,subtree.total_mass));
                    self.total_mass+=subtree.total_mass;

                },
                None => {
                    continue;
                }
            }
        }
        // self.center_of_mass.as_mut().unwrap().div_assign(non_empty_count);
        // if non_empty_count > 0{
        //     self.center_of_mass.as_mut().unwrap().x /= self.total_mass;
        //     self.center_of_mass.as_mut().unwrap().y /= self.total_mass;
        // }

    }

    pub fn center_between_two_points(&self, pos_a: Vector2<f64>, mass_a: f64, pos_b: Vector2<f64>, mass_b: f64) -> Vector2<f64>{
        let sum_mass: f64 = mass_a + mass_b;
        let center: Vector2<f64> = ((pos_a * mass_a) + (pos_b * mass_b))/sum_mass;
        // print!("{:?}",center);

        return center;

    }

    pub fn subtree_index(&self, pos: Vector2<f64>) -> Option<usize>{
        if self.subtrees.len() == 0 {
            return None;
        }
        let midpoint = self.boundaries.midpoint();
        if pos.x <= midpoint.x {
            if pos.y <= midpoint.y {
                return Some(A);
            } else {
                return Some(C);
            }
        } else {
            if pos.y <= midpoint.y {
                return Some(B);
            } else {
                return Some(D);
            }
        }
    }

    pub fn print_boundaries(&self, spacing: String){
        println!("{} {:?}",spacing,self.boundaries);
        for st in &self.subtrees{
            st.print_boundaries(spacing.to_owned() + "  ");
        }
    }

}

#[cfg(test)]
mod tests{
    use cgmath::Vector2;
    use crate::body::Body;
    use crate::quadtree::{A, D, Rectangle};
    use crate::quadtree::Quadtree;
    #[test]
    fn test_within(){
        let pos: Vector2<f64> = Vector2::new(100.0,100.0);
        let rec: Rectangle = Rectangle::new(Vector2::new(50.0f64,50.0f64),Vector2::new(150.0f64,150.0f64));
        let rec2: Rectangle = Rectangle::new(Vector2::new(99.0f64,99.0f64),Vector2::new(101.0f64,101.0f64));
        let rec3: Rectangle = Rectangle::new(Vector2::new(100.0f64,100.0f64),Vector2::new(101.0f64,101.0f64));
        assert!(rec.within(pos));
        assert!(rec2.within(pos));
        assert!(rec3.within(pos));
    }

    #[test]
    fn test_not_within(){
        let pos: Vector2<f64> = Vector2::new(100.0,100.0);
        let rec: Rectangle = Rectangle::new(Vector2::new(200.0f64,200.0f64),Vector2::new(400.0f64,400.0f64));
        let rec2: Rectangle = Rectangle::new(Vector2::new(50.0f64,200.0f64),Vector2::new(150.0f64,400.0f64));
        let rec3: Rectangle = Rectangle::new(Vector2::new(50.0f64,50.0f64),Vector2::new(150.0f64,65.0f64));
        let rec4: Rectangle = Rectangle::new(Vector2::new(0.0f64,0.0f64),Vector2::new(50.0f64,50.0f64));
        assert_eq!(rec.within(pos),false);
        assert_eq!(rec2.within(pos),false);
        assert_eq!(rec3.within(pos),false);
        assert_eq!(rec4.within(pos),false);
    }

    #[test]
    fn rectangle_midpoint() {
        let rec: Rectangle = Rectangle::new(Vector2::new(200.0f64,200.0f64),Vector2::new(400.0f64,400.0f64));
        let rec2: Rectangle = Rectangle::new(Vector2::new(50.0f64,200.0f64),Vector2::new(150.0f64,400.0f64));
        let rec3: Rectangle = Rectangle::new(Vector2::new(50.0f64,50.0f64),Vector2::new(150.0f64,65.0f64));
        let rec4: Rectangle = Rectangle::new(Vector2::new(0.0f64,0.0f64),Vector2::new(50.0f64,50.0f64));
        assert_eq!(rec.midpoint(),Vector2::new(300.0f64,300.0f64));
        assert_eq!(rec2.midpoint(),Vector2::new(100.0f64,300.0f64));
        assert_eq!(rec3.midpoint(),Vector2::new(100.0f64,57.50f64));
        assert_eq!(rec4.midpoint(),Vector2::new(25.0f64,25.0f64));


    }

    #[test]
    fn rectangle_subranges(){
        let rec: Rectangle = Rectangle::new(Vector2::new(200.0f64,200.0f64),Vector2::new(400.0f64,400.0f64));
        let expected_sub: (Rectangle,Rectangle,Rectangle,Rectangle) = (
            Rectangle::new(Vector2::new(200.0f64,200.0f64),Vector2::new(300.0f64,300.0f64)),
            Rectangle::new(Vector2::new(300.0f64,200.0f64),Vector2::new(400.0f64,300.0f64)),
            Rectangle::new(Vector2::new(200.0f64,300.0f64),Vector2::new(300.0f64,400.0f64)),
            Rectangle::new(Vector2::new(300.0f64,300.0f64),Vector2::new(400.0f64,400.0f64))
            );
        let actual_sub = rec.subranges();
        assert_eq!(expected_sub.0,actual_sub.0);
        assert_eq!(expected_sub.1,actual_sub.1);
        assert_eq!(expected_sub.2,actual_sub.2);
        assert_eq!(expected_sub.3,actual_sub.3);
    }

    #[test]
    fn subtree_index(){
        let rec: Rectangle = Rectangle::new(Vector2::new(200.0f64,200.0f64),Vector2::new(400.0f64,400.0f64));
        let mut qt: Quadtree = Quadtree::new(rec,5);
        let p1: Vector2<f64> = Vector2::new(303.0f64,350.0f64);
        let p2: Vector2<f64> = Vector2::new(203.0f64,250.0f64);
        qt.split();
        // println!("{:?}",qt.subtree_index(p1));
        assert_eq!(qt.subtree_index(p1).unwrap_or_default(),D);
        assert_eq!(qt.subtree_index(p2).unwrap_or_default(),A);
    }

    #[test]
    fn split(){
        let rec: Rectangle = Rectangle::new(Vector2::new(200.0f64,200.0f64),Vector2::new(400.0f64,400.0f64));
        let mut qt: Quadtree = Quadtree::new(rec,5);
        qt.split();
        assert_eq!(qt.subtrees.len(),4);

    }

    #[test]
    fn insert(){
        let rec: Rectangle = Rectangle::new(Vector2::new(200.0f64,200.0f64),Vector2::new(400.0f64,400.0f64));
        let mut qt: Quadtree = Quadtree::new(rec,1);
        let body: Body = Body::with_pos(Vector2::new(250.0,250.0));
        let body2: Body = Body::with_pos(Vector2::new(210.0,230.0));
        qt.insert(body);
        qt.insert(body2);
        assert_eq!(qt.subtrees.len(),4);
    }
    #[test]
    fn center_node_mass(){
        let rec: Rectangle = Rectangle::new(Vector2::new(0.0f64,0.0f64),Vector2::new(400.0f64,400.0f64));
        let mut qt: Quadtree = Quadtree::new(rec,1);
        qt.insert(Body::with_pos(Vector2::new(1.0,1.0)));
        qt.insert(Body::with_pos(Vector2::new(10.0,10.0)));
        qt.insert(Body::with_pos(Vector2::new(100.0,20.0)));
        assert_eq!(qt.subtrees.len(),4);
        assert_eq!(qt.subtrees[A].boundaries.br,Vector2::new(200.0,200.0));
        qt.insert(Body::with_pos(Vector2::new(120.0,120.0)));
        assert_eq!(qt.subtrees[A].subtrees.len(),4);
        assert_eq!(qt.subtrees[A].subtrees[A].subtrees.len(),4);
        qt.subtrees[A].update_mass();
        assert_eq!(qt.subtrees[A].center_of_mass.unwrap().x,57.75);
        assert_eq!(qt.subtrees[A].center_of_mass.unwrap().y,37.75);
    }
    #[test]
    fn center_mass(){
        let rec: Rectangle = Rectangle::new(Vector2::new(0.0f64,0.0f64),Vector2::new(400.0f64,400.0f64));
        let mut qt: Quadtree = Quadtree::new(rec,1);
        let body: Body = Body::with_pos(Vector2::new(250.0,250.0));
        let body2: Body = Body::with_pos(Vector2::new(210.0,230.0));
        let body3: Body = Body::with_pos(Vector2::new(0.0,0.0));
        let body4: Body = Body::with_pos(Vector2::new(1.0,1.0));
        // let body5: Body = Body::with_pos(Vector2::new(205.0,205.0));
        qt.insert(body);
        // qt.update_mass();
        qt.insert(body2);
        qt.insert(body3);
        qt.insert(body4);
        qt.update_mass();
        assert_eq!(qt.center_of_mass.unwrap().x, 115.25);
        assert_eq!(qt.center_of_mass.unwrap().y, 120.25);



        // qt.insert(body5);
    }
    #[test]
    fn center_mass_two(){
        let rec: Rectangle = Rectangle::new(Vector2::new(0.0f64,0.0f64),Vector2::new(400.0f64,400.0f64));
        let mut qt: Quadtree = Quadtree::new(rec,1);
        qt.bodies = Vec::new();
        qt.subtrees = Vec::new();
        let body5: Body = Body::with_pos(Vector2::new(1.0,1.0));
        let body6: Body = Body::with_pos(Vector2::new(2.0,2.0));
        let body7: Body = Body::with_pos(Vector2::new(3.0,3.0));
        let body8: Body = Body::with_pos(Vector2::new(4.0,4.0));
        let body9: Body = Body::with_pos(Vector2::new(100.0,100.0));

        qt.insert(body5);
        qt.insert(body6);
        qt.insert(body7);
        qt.insert(body8);
        qt.insert(body9);
        qt.update_mass();
        //22 22
        assert_eq!(qt.center_of_mass.unwrap().x,22.0);
        assert_eq!(qt.center_of_mass.unwrap().y,22.0);
    }

    #[test]
    fn rectangle_mid(){
        let rec: Rectangle = Rectangle::new(Vector2::new(0.0f64,0.0f64),Vector2::new(400.0f64,400.0f64));
        assert_eq!(rec.width(),400.0f64);
        assert_eq!(rec.height(),400.0f64);
        let rec: Rectangle = Rectangle::new(Vector2::new(200.0f64,200.0f64),Vector2::new(400.0f64,400.0f64));
        assert_eq!(rec.width(),200.0f64);
        assert_eq!(rec.height(),200.0f64);
        let rec: Rectangle = Rectangle::new(Vector2::new(32.0f64,98.0f64),Vector2::new(101.0f64,255.0f64));
        assert_eq!(rec.width(),69.0f64);
        assert_eq!(rec.height(),157.0f64);

    }

    #[test]
    fn clear_quadtree(){
        let rec: Rectangle = Rectangle::new(Vector2::new(0.0f64,0.0f64),Vector2::new(400.0f64,400.0f64));
        let mut qt: Quadtree = Quadtree::new(rec,1);
        let body3: Body = Body::with_pos(Vector2::new(0.0,0.0));
        let body4: Body = Body::with_pos(Vector2::new(150.0,150.0));
        qt.insert(body3);
        qt.insert(body4);
        println!("{:?}",qt.subtrees);
        assert_eq!(qt.subtrees.len(), 4);
        assert_eq!(qt.subtrees[A].subtrees[A].bodies.len(),1);
        qt.clear();
        assert_eq!(qt.subtrees.len(),0);

    }

    #[test]
    fn test_non_positive_boundaries(){
        let rec: Rectangle = Rectangle::new(Vector2::new(-100.0f64,-100.0f64),Vector2::new(100.0f64,100.0f64));
        let mut qt: Quadtree = Quadtree::new(rec,1);
        qt.split();
        assert_eq!(qt.subtrees.len(), 4);

        //finish

        // assert_eq!(qt.subtrees[A].boundaries.tl.x)
    }


}