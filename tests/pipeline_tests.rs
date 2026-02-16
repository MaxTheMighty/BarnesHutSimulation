use std::sync::mpsc::channel;

use barnes_hut::{
    body::Body,
    gpu::pipeline::*,
    simulation::{self, Simulation},
};
use wgpu::{BindGroupEntry, BufferUsages};

#[tokio::test]
async fn run_pipeline() {
    let mut builder = ComputePipelineBuilder::new().await.unwrap();
    let shader_module = builder
        .pipeline
        .device
        .create_shader_module(wgpu::include_wgsl!("../shaders/n-body.wgsl"));

    let mut input_data: Vec<Body> = Vec::new();
    for _ in 0..1024 {
        input_data.push(Body::random(0.0, 100.0));
    }
    let input_data_bytes: Vec<u8> = bytemuck::cast_slice(input_data.as_slice()).to_vec();
    let input_data_usage: BufferUsages = wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE;

    let output_data_bytes: Vec<u8> = vec![0; input_data_bytes.len()];
    let output_data_usage: BufferUsages =
        wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::STORAGE;

    let mut input_builder = builder.build_shader(shader_module).unwrap();

    input_builder
        .append_input_data(input_data_bytes, input_data_usage)
        .unwrap();
    let mut output_builder = input_builder.done().unwrap();

    let output_buffer_index = output_builder
        .append_output_data(output_data_bytes, output_data_usage)
        .unwrap();

    let pipeline_builder = output_builder.done().unwrap();

    let bindgroup_builder = pipeline_builder.build_pipeline(&[]).unwrap();

    let encoder_builder = bindgroup_builder.build_bindgroup().unwrap();

    let mut pipeline = encoder_builder.setup_encoder().unwrap().finalize().unwrap();

    pipeline.execute_single_pass(32).unwrap();

    let output_buffers_binding = pipeline.output_buffers.unwrap();
    let output_buffer_ref = output_buffers_binding.get(output_buffer_index).unwrap();

    let temp_buffer = pipeline.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("temp buffer for output"),
        size: output_buffer_ref.size(),
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    let mut encoder = pipeline.encoder.unwrap();
    encoder.copy_buffer_to_buffer(
        output_buffer_ref,
        0,
        &temp_buffer,
        0,
        output_buffer_ref.size(),
    );

    let finish = encoder.finish();
    pipeline.queue.submit([finish]);

    let temp_slice = temp_buffer.slice(..);
    let (data_sender, data_receiver) = channel();
    temp_slice.map_async(wgpu::MapMode::Read, move |v| data_sender.send(v).unwrap());
    let _poll_result = pipeline.device.poll(wgpu::PollType::Wait);

    let mut simulation: Simulation = Simulation::new();
    simulation.bodies.append(&mut input_data);
    simulation.update_only_force();

    if let Ok(Ok(())) = data_receiver.recv() {
        let data = temp_slice.get_mapped_range();
        let result: Vec<Body> = bytemuck::cast_slice(&*data).to_vec();
        drop(data);

        // Compare to brute force
        println!("Comparing brute force to GPU");
        for (gpu_body, cpu_body) in result.iter().zip(simulation.bodies) {
            println!("GPU Body: {:?}", gpu_body.force);
            println!("CPU Body: {:?}", cpu_body.force);
            println!("-------------------------------------");
        }
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
