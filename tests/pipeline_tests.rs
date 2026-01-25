use barnes_hut::{body::Body, gpu::pipeline::*};
use wgpu::BufferUsages;

#[tokio::test]
async fn run_pipeline() {
    let mut pipeline: ComputePipeline = ComputePipeline::new().await.unwrap();
    let shader_module = pipeline
        .device
        .create_shader_module(wgpu::include_wgsl!("../shaders/n-body.wgsl"));
    pipeline.setup_shader_module(shader_module).unwrap();
    pipeline.setup_pipeline().unwrap();

    let mut input_data: Vec<Body> = Vec::new();
    for _ in 0..1024 {
        input_data.push(Body::random(0.0, 100.0));
    }
    let input_data_bytes: Vec<u8> = bytemuck::cast_slice(input_data.as_slice()).to_vec();
    let input_data_usage: BufferUsages = wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE;

    let output_data_bytes: Vec<u8> = Vec::with_capacity(input_data_bytes.len());
    let output_data_usage: BufferUsages =
        wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::STORAGE;

    pipeline
        .setup_input_data(input_data_bytes, input_data_usage)
        .unwrap();
    pipeline
        .setup_output_data(output_data_bytes, output_data_usage)
        .unwrap();

    pipeline.setup_encoder().unwrap();

    pipeline.execute_single_pass(32).unwrap();
}
