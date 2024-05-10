use cgmath::{Point1, Vector2};
use crate::body::Body;

struct Rectangle {
    tl: Vector2<f32>,
    br: Vector2<f32>
}
struct Quadtree{
    boundaries: Rectangle,
    limit: u8,
    subtrees: Vec<Box<Quadtree>>,
    bodies: Vec<Body>,
    center_of_mass: f32,
    total_mass: f32
}


impl Rectangle{
    pub fn new(tl: Vector2<f32>, br: Vector2<f32>) -> Self {
        Self {
            tl,
            br
        }
    }
    pub fn within(&self, obj_pos: Vector2<f32>) -> bool {
        return  obj_pos.x >= self.tl.x &&
                obj_pos.x < self.br.x &&
                obj_pos.y >= self.tl.y &&
                obj_pos.y < self.br.y
    }

    pub fn midpoint(&self) -> Vector2<f32> {
        return Vector2::new(self.tl.x + ((self.br.x-self.tl.x)/2.0f32),self.tl.y + ((self.br.y-self.tl.y)/2.0f32));
    }


}


impl Quadtree {
    pub fn new(boundaries: Rectangle, limit: u8) -> Self {
        Self {
            boundaries,
            limit,
            subtrees: Vec::new(),
            bodies: vec![],
            center_of_mass: 0.0,
            total_mass: 0.0,
        }
    }


    pub fn split(&mut self){

    }


    pub fn subranges(&self){
        let midpoint: Vector2<f32> = self.boundaries.midpoint();
    }
}

#[cfg(test)]
mod tests{
    use cgmath::Vector2;
    use crate::quadtree::Rectangle;

    #[test]
    fn test_within(){
        let pos: Vector2<f32> = Vector2::new(100.0,100.0);
        let rec: Rectangle = Rectangle::new(Vector2::new(50.0f32,50.0f32),Vector2::new(150.0f32,150.0f32));
        let rec2: Rectangle = Rectangle::new(Vector2::new(99.0f32,99.0f32),Vector2::new(101.0f32,101.0f32));
        let rec3: Rectangle = Rectangle::new(Vector2::new(100.0f32,100.0f32),Vector2::new(101.0f32,101.0f32));
        assert!(rec.within(pos));
        assert!(rec2.within(pos));
        assert!(rec3.within(pos));
    }

    #[test]
    fn test_not_within(){
        let pos: Vector2<f32> = Vector2::new(100.0,100.0);
        let rec: Rectangle = Rectangle::new(Vector2::new(200.0f32,200.0f32),Vector2::new(400.0f32,400.0f32));
        let rec2: Rectangle = Rectangle::new(Vector2::new(50.0f32,200.0f32),Vector2::new(150.0f32,400.0f32));
        let rec3: Rectangle = Rectangle::new(Vector2::new(50.0f32,50.0f32),Vector2::new(150.0f32,65.0f32));
        let rec4: Rectangle = Rectangle::new(Vector2::new(0.0f32,0.0f32),Vector2::new(50.0f32,50.0f32));
        assert_eq!(rec.within(pos),false);
        assert_eq!(rec2.within(pos),false);
        assert_eq!(rec3.within(pos),false);
        assert_eq!(rec4.within(pos),false);
    }

    #[test]
    fn rectangle_midpoint() {
        let rec: Rectangle = Rectangle::new(Vector2::new(200.0f32,200.0f32),Vector2::new(400.0f32,400.0f32));
        let rec2: Rectangle = Rectangle::new(Vector2::new(50.0f32,200.0f32),Vector2::new(150.0f32,400.0f32));
        let rec3: Rectangle = Rectangle::new(Vector2::new(50.0f32,50.0f32),Vector2::new(150.0f32,65.0f32));
        let rec4: Rectangle = Rectangle::new(Vector2::new(0.0f32,0.0f32),Vector2::new(50.0f32,50.0f32));
        assert_eq!(rec.midpoint(),Vector2::new(300.0f32,300.0f32));
        assert_eq!(rec2.midpoint(),Vector2::new(100.0f32,300.0f32));
        assert_eq!(rec3.midpoint(),Vector2::new(100.0f32,57.50f32));
        assert_eq!(rec4.midpoint(),Vector2::new(25.0f32,25.0f32));


    }
}