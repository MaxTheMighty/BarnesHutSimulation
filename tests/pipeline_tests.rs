use std::sync::mpsc::channel;

use barnes_hut::{
    body::Body,
    gpu::pipeline::*,
    simulation::{self, Simulation},
};
use std::time;
use wgpu::BufferUsages;
#[tokio::test]
async fn run_pipeline() {
    let start_time = time::Instant::now();
    let mut builder = ComputePipelineBuilder::new().await.unwrap();
    let shader_module = builder
        .pipeline
        .device
        .create_shader_module(wgpu::include_wgsl!("../shaders/n-body.wgsl"));

    let mut input_data: Vec<Body> = Vec::new();
    let body_count = 1_000_000;
    for _ in 0..body_count as usize {
        input_data.push(Body::random(0.0, 100.0));
    }
    let input_data_bytes: Vec<u8> = bytemuck::cast_slice(input_data.as_slice()).to_vec();
    let input_data_bytes_count = input_data_bytes.len();
    let input_data_usage: BufferUsages =
        wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE;

    let mut input_builder = builder.build_shader(shader_module).unwrap();

    let input_buffer_index = input_builder
        .append_input_data(input_data_bytes, input_data_usage)
        .unwrap();

    let output_builder = input_builder.done().unwrap();

    let pipeline_builder = output_builder.done().unwrap();

    let bindgroup_builder = pipeline_builder.build_pipeline(&[]).unwrap();

    let encoder_builder = bindgroup_builder.build_bindgroup().unwrap();

    let mut pipeline = encoder_builder.setup_encoder().unwrap().finalize().unwrap();

    // a workgroup has 64 threads
    // if we want one thread per body for n bodies
    // we want to do bodies/64
    let wg_count = body_count / 64;
    pipeline.execute_n_passes(wg_count, 1).unwrap();

    let input_buffer_binding = pipeline.input_buffers.unwrap();
    let input_buffer_ref = input_buffer_binding.get(input_buffer_index).unwrap();

    let temp_buffer = pipeline.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("temp buffer for output"),
        size: input_data_bytes_count as u64,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    let mut encoder = pipeline.encoder.unwrap();
    encoder.copy_buffer_to_buffer(
        input_buffer_ref,
        0,
        &temp_buffer,
        0,
        input_buffer_ref.size(),
    );

    let finish = encoder.finish();
    pipeline.queue.submit([finish]);

    let temp_slice = temp_buffer.slice(..);
    let (data_sender, data_receiver) = channel();
    temp_slice.map_async(wgpu::MapMode::Read, move |v| data_sender.send(v).unwrap());
    let _poll_result = pipeline.device.poll(wgpu::PollType::Wait);

    // let mut simulation: Simulation = Simulation::new();
    // let mut bodies_clone: Vec<Body> = input_data.clone();
    // simulation.bodies.append(&mut bodies_clone);
    // for _ in 0..1000 {
    //     simulation.update();
    //     simulation.update();
    // }
    // simulation.update_only_force();

    if let Ok(Ok(())) = data_receiver.recv() {
        let data = temp_slice.get_mapped_range();
        let result: Vec<Body> = bytemuck::cast_slice(&*data).to_vec();
        drop(data);
        // dbg!(&result);
        let finish_time = time::Instant::now();
        let execution_time = finish_time - start_time;
        println!("Execution time GPU: {:?}", execution_time);
        // Compare to brute force
        println!("Comparing brute force to GPU");
        // for (gpu_body, cpu_body) in result.iter().zip(simulation.bodies) {
        //     println!("GPU Body: {:?}", gpu_body);
        //     println!("CPU Body: {:?}", cpu_body);
        //     println!("-------------------------------------");
        // }
        // println!("Result bodies length {:?}", result.len());
        // result.iter().for_each(|b| println!("{:?}", b));
        // println!("Comparing bodies...");

        // let mut index: usize = 0;
        // for (result_body, expected_body) in result.iter().zip(input_data) {
        //     if *result_body != expected_body && (*result_body).mass != expected_body.mass + 1.0 {
        //         println!("Bodies don't match!");
        //         println!("Index {index}");
        //         println!("Result:   {:?}", result_body);
        //         println!("Expected: {:?}", expected_body);
        //     }
        //     index += 1;
        // }
    } else {
        panic!("Failed to receive data!")
    }

    return ();
}
