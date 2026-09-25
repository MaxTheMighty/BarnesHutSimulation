
use wgpu::BufferUsages;
use crate::{body::Body, gpu::pipeline::{ComputePipeline, ComputePipelineBuilder}};

#[derive(Clone,Debug)]
pub struct SimulationOptions {
    shader_location: String
}
pub struct SimulationRunner {
    pipeline: ComputePipeline,
    bodies: Vec<Body>,
    options: SimulationOptions,
    byte_count: usize
}

const THREADS_PER_WORKGROUP: usize = 64;

impl SimulationRunner {
    pub async fn new(options: SimulationOptions, bodies: Vec<Body>) -> Self {
        let builder = ComputePipelineBuilder::new().await.unwrap(); 
        
        // Get a shader module from the given path
        let shader_module = builder.pipeline.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Simulation runner shader"),
            source: wgpu::ShaderSource::Wgsl(std::fs::read_to_string(options.shader_location.clone()).unwrap().into())
        });

        // Get the bytes of the bodies
        let bodies_bytes: Vec<u8> = bytemuck::cast_slice(bodies.as_slice()).to_vec();
        let byte_count = bodies_bytes.len();
        // Define the usages of the buffer for WGPU
        let bodies_usages: BufferUsages = wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE;

        let mut input_builder = builder.build_shader(shader_module).unwrap();

        // Append the input data to the pipeline
        input_builder.append_input_data(bodies_bytes, bodies_usages).unwrap();

        // Maybe we can drop the bytes here?

        // Skip the stages to the pipeline builder since we dont do anything with the output builder
        let pipeline_builder = input_builder.done().unwrap().done().unwrap();

        // Let the builder make the pipeline for us, we dont have any options (for now)
        let bindgroup_builder = pipeline_builder.build_pipeline(&[]).unwrap();

        // Skip the encoder builder and go straight to the pipeline
        let pipeline = bindgroup_builder.build_bindgroup().unwrap().setup_encoder().unwrap().finalize().unwrap();

        Self {
            pipeline: pipeline,
            bodies: bodies,
            options: options,
            byte_count: byte_count 
        }
    }

    pub async fn run_n_steps(&mut self, n: u32) {
        // Get the amount of work groups to dispatch
        // Currently, a workgroup is hardcoded to have 64 threads

        // Make sure bodies is a multiple of 64
        if self.bodies.len() % 64 != 0 {
            panic!("Length of bodies must be a multiple of 64, but is not: {}",self.bodies.len());
        }

        // Get WG count
        let wg_count = self.bodies.len() / THREADS_PER_WORKGROUP;

        // Execute
        self.pipeline.execute_n_passes(wg_count as u32, n).unwrap();
    }

    pub async fn get_bodies(&mut self) -> Vec<Body> {
        // Binding to make the rust compiler happy
        let input_buffer_binding = self.pipeline.input_buffers.as_ref().unwrap();
        // Get the input buffer ref. Its hard-coded to 0 since we only send the body buffer
        let input_buffer_ref = input_buffer_binding.get(0).unwrap();

        // Define the temporary buffer for reading the bodies into
        let temporary_buffer = self.pipeline.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Temporary buffer for output"),
            size: self.byte_count.clone() as u64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false 
        });

        // Get the encoder and setup the copy operation
        let mut encoder = self.pipeline.encoder.unwrap();
        encoder.copy_buffer_to_buffer(input_buffer_ref, 0, &temporary_buffer, 0, input_buffer_ref.size());

        let finish = self.pipeline.encoder.unwrap().finish();
        self.pipeline.queue.submit([finish]);
        todo!();
    }
}

