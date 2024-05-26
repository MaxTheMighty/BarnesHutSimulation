use cgmath::Vector2;
use barnes_hut::body::Body;
use barnes_hut::quadtree::{Rectangle, Quadtree};
fn main() {
    let rec: Rectangle = Rectangle::new(Vector2::new(0.0f64,0.0f64),Vector2::new(400.0f64,400.0f64));
    let mut qt: Quadtree = Quadtree::new(rec,1);
    //let body: Body = Body::with_pos(Vector2::new(250.0,250.0));
    // let body2: Body = Body::with_pos(Vector2::new(210.0,230.0));
    let body1: Body = Body::with_pos(Vector2::new(0.0,0.0));
    let body2: Body = Body::with_pos(Vector2::new(1.0,1.0));
    let body5: Body = Body::with_pos(Vector2::new(205.0,205.0));
    qt.insert(body1);
    qt.insert(body5);
    qt.update_mass();
    // println!("{:?}",qt.subtrees.len());
    // println!("{:?}",qt.subtrees[0].subtrees.len());
    assert!(qt.center_of_mass.is_some());
    assert!(qt.subtrees[0].center_of_mass.is_some());
    assert!(qt.subtrees[1].center_of_mass.is_none());
    assert!(qt.subtrees[2].center_of_mass.is_none());
    assert!(qt.subtrees[3].center_of_mass.is_some());
    println!("{:?}",qt.center_of_mass);
    println!("{:?}",qt.subtrees[3].center_of_mass);
    // qt.update_mass();
    //qt.insert(body3);
    //qt.insert(body4);
    // qt.update_mass();
    // qt.insert(body5);


}