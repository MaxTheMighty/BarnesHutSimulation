use std::ops::Index;
use cgmath::Vector2;
use barnes_hut::body::Body;
use barnes_hut::gravity::*;

fn main() {
    let mut bodies: Vec<Body> = Vec::new();

    bodies.push(Body::with_mass_and_pos(10.0,Vector2::new(100.0,100.0)));
    bodies.push(Body::with_mass_and_pos(500.0,Vector2::new(200.0,400.0)));
    //pre update
    for _ in 0..100{
        for body in bodies.iter() {
            println!("{:?}",body);
        }

        for i in 0..bodies.len(){
            for j in i..bodies.len(){
                if i == j {
                    continue;
                }
                println!("{} {}",i,j);
                calculate_force(&mut bodies[i..j+1]);
            }
        }



        for body in bodies.iter_mut(){
            apply_force(body);
        }

        //post update
        for body in bodies.iter(){
            println!("{:?}",body);
        }
    }

}