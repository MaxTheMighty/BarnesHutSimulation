use cgmath::Vector2;
use barnes_hut::bh_runner::BarnesHutRunner;
use barnes_hut::body::Body;
use barnes_hut::quadtree::{Quadtree, Rectangle};
use barnes_hut::canvas::Canvas;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn simulation(iterations: i32) {

    // bodies.push(Body::with_mass_and_pos(10000000.0,Vector2::new(1350.0,700.0)));
    // runner.generate_square(&mut bodies, 100, 1250.0 - 100.0, 720.0 - 100.0);



}

fn criterion_benchmark(c: &mut Criterion) {
    let rec: Rectangle = Rectangle::new(Vector2::new(0.0f64,0.0f64),Vector2::new(1000.0 ,1000.0));
    let mut qt: Quadtree = Quadtree::new(rec,1);
    let mut bodies: Vec<Body> = Vec::new();
    let mut runner: BarnesHutRunner = BarnesHutRunner::from_theta(0.5f64);
    runner.generate_circle(&mut bodies,450.0,500.0,30.0);
    runner.generate_circle(&mut bodies,550.0,500.0,30.0);
    runner.resize(&mut qt,&mut bodies);
    runner.create_tree(&mut qt,&mut bodies);
    c.bench_function("iterate 100", |b| {
        b.iter(||
            runner.iterate(&mut qt,&mut bodies)
        );

    } );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
