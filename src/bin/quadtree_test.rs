use cgmath::Vector2;
use deepsize::DeepSizeOf;
use barnes_hut::body::Body;
use barnes_hut::quadtree::{Rectangle, Quadtree};
fn main() {
    let rec: Rectangle = Rectangle::new(Vector2::new(0.0f64,0.0f64),Vector2::new(400.0f64,400.0f64));
    let mut qt: Quadtree = Quadtree::new(rec,1);
    let body3: Body = Body::with_pos(Vector2::new(0.0,0.0));
    let body4: Body = Body::with_pos(Vector2::new(150.0,150.0));
    qt.insert(body3);
    qt.insert(body4);
    println!("{:?}",qt.subtrees);
    assert_eq!(qt.subtrees.len(), 4);
    assert_eq!(qt.subtrees[0].subtrees[0].bodies.len(),1);


}