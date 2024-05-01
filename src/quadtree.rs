use crate::body::Body;
struct Quadtree{
    limit: u8,
    subtrees: [Box<Quadtree>; 4],
    bodies: Vec<Body>,
    center_of_mass: f32,
    total_mass: f32
}