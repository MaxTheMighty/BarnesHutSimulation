mod quadtree;
mod body;
use cgmath::Vector2;

fn main() {
    println!("Hello, world!");
    let vec_one: Vector2<u8> = Vector2::new(0,1);
    let _vec_two: Vector2<u8> = Vector2::new(2,5);
    println!("{:?}", vec_one);

}
