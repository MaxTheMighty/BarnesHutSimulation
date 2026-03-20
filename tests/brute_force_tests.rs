use std::time;

use barnes_hut::{body::Body, simulation::Simulation};

#[test]
fn run_bruteforce() {
    let start_time = time::Instant::now();
    let mut input_data: Vec<Body> = Vec::new();
    let body_count = 2048;
    for _ in 0..body_count {
        input_data.push(Body::random(0.0, 100.0));
    }

    let mut simulation: Simulation = Simulation::new();
    simulation.bodies = input_data;
    for _ in 0..2000 {
        simulation.update();
    }
    let finish_time = time::Instant::now();
    let execution_time = finish_time - start_time;
    println!("Execution time CPU: {:?}", execution_time);
}
