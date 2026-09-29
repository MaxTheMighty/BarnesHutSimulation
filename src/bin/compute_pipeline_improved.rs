use barnes_hut::{
    body::Body,
    canvas::Canvas,
    gpu::simulation_runner::{SimulationOptions, SimulationRunner},
};
use hsv::hsv_to_rgb;
use image::{ImageBuffer, Rgb};

#[tokio::main]
async fn main() {
    const BODY_COUNT: usize = 1_000_0128;
    const PRE_COMPUTE_FILE_PATH: &'static str =
        "/home/maxwellk/Projects/BarnesHutSimulation/renders/pre_compute.png";
    const POST_COMPUTE_FILE_PATH: &'static str =
        "/home/maxwellk/Projects/BarnesHutSimulation/renders/post_compute.png";
    let mut bodies: Vec<Body> = Vec::with_capacity(BODY_COUNT);

    for _ in 0..BODY_COUNT {
        bodies.push(Body::random(0.0, 1000.0));
    }
    let options: SimulationOptions = SimulationOptions {
        shader_location: String::from(
            "/home/maxwellk/Projects/BarnesHutSimulation/shaders/n-body.wgsl",
        ),
    };

    draw_bodies_to_file(1000, 1000, &bodies, PRE_COMPUTE_FILE_PATH);
    let mut runner = SimulationRunner::new(options, bodies).await;
    runner.queue_n_steps(2).await;
    runner.run_queued().await;
    let computed_bodies = runner.get_bodies().await.unwrap();
    draw_bodies_to_file(1000, 1000, &computed_bodies, POST_COMPUTE_FILE_PATH);
}

fn draw_bodies_to_file(width: u32, height: u32, bodies: &Vec<Body>, file_path: &str) {
    // let mut img

    let mut img: ImageBuffer<Rgb<u8>, Vec<u8>> =
        ImageBuffer::from_pixel(width, height, Rgb([255u8, 255u8, 255u8]));
    let mut canvas = Canvas::new(width, height, (0, 0, 0, 0));
    draw_bodies(&mut canvas, bodies);
    write_canvas_to_file(&mut canvas, &mut img, file_path);
}

fn draw_bodies(canvas: &mut Canvas, bodies: &Vec<Body>) {
    match (bodies.is_empty()) {
        true => {}
        false => {
            for body in bodies {
                update_pixel_heat(canvas, body); //not a fan of this casting
            }
        }
    }
}

fn update_pixel_heat(canvas: &mut Canvas, body: &Body) {
    let x_pos: i32 = body.pos.x.round() as i32;
    let y_pos: i32 = body.pos.y.round() as i32;

    if (!canvas.pos_valid(x_pos, y_pos)) {
        return;
    }

    canvas.increment_huemap(
        body.pos.x as i32,
        body.pos.y as i32,
        (240.0, 1.0, 1.0),
        -1.0,
    );
}

fn write_canvas_to_file(
    canvas: &mut Canvas,
    img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    file_path: &str,
) {
    for (pixel, hue) in img.pixels_mut().zip(canvas.huemap.iter()) {
        let (h, s, v) = *hue;
        let rgb = hsv_to_rgb(h, s, v);
        *pixel = Rgb([rgb.0 as u8, rgb.1 as u8, rgb.2 as u8]);
    }

    img.save(file_path).expect("Could not write to file");
}
