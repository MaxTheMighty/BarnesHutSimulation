use std::{sync::mpsc::channel, time::Instant};

use crate::{
    body::Body,
    gpu::pipeline::{ComputePipeline, ComputePipelineBuilder},
};
use log::log;
use wgpu::{Buffer, BufferUsages};

#[derive(Clone, Debug)]
pub struct SimulationOptions {
    pub shader_location: String,
}
pub struct SimulationRunner {
    pipeline: ComputePipeline,
    bodies: Vec<Body>,
    options: SimulationOptions,
    byte_count: usize,
    read_buffer: Buffer,
    start_instant: Option<Instant>,
}

const THREADS_PER_WORKGROUP: usize = 128;

impl SimulationRunner {
    pub async fn new(options: SimulationOptions, bodies: Vec<Body>) -> Self {
        let builder = ComputePipelineBuilder::new().await.unwrap();

        // Get a shader module from the given path
        let shader_module =
            builder
                .pipeline
                .device
                .create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some("Simulation runner shader"),
                    source: wgpu::ShaderSource::Wgsl(
                        std::fs::read_to_string(options.shader_location.clone())
                            .unwrap()
                            .into(),
                    ),
                });

        // Get the bytes of the bodies
        let bodies_bytes: Vec<u8> = bytemuck::cast_slice(bodies.as_slice()).to_vec();
        let byte_count = bodies_bytes.len();
        // Define the usages of the buffer for WGPU
        let input_bodies_usages: BufferUsages = wgpu::BufferUsages::COPY_SRC
            | wgpu::BufferUsages::COPY_DST
            | wgpu::BufferUsages::STORAGE;

        let output_bodies_usages: BufferUsages = wgpu::BufferUsages::COPY_SRC
            | wgpu::BufferUsages::COPY_DST
            | wgpu::BufferUsages::STORAGE;

        let mut input_builder = builder.build_shader(shader_module).unwrap();

        // Append the input data to the pipeline
        input_builder
            .append_input_data(bodies_bytes.clone(), input_bodies_usages)
            .unwrap();

        let mut output_builder = input_builder.done().unwrap();

        output_builder
            .append_output_data(bodies_bytes, output_bodies_usages)
            .unwrap();

        let pipeline_builder = output_builder.done().unwrap();

        // Let the builder make the pipeline for us, we dont have any options (for now)
        let bindgroup_builder = pipeline_builder.build_pipeline(&[]).unwrap();

        // Build the bindgroup and encoder
        let pipeline = bindgroup_builder
            .build_bindgroup()
            .unwrap()
            .setup_encoder()
            .unwrap()
            .finalize()
            .unwrap();

        // Build the temporary buffer
        let read_buffer = pipeline.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Temporary buffer for output"),
            size: byte_count.clone() as u64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        Self {
            pipeline: pipeline,
            bodies: bodies,
            options: options,
            byte_count: byte_count,
            read_buffer,
            start_instant: None,
        }
    }

    pub async fn queue_n_steps(&mut self, n: u32) {
        // Get the amount of work groups to dispatch
        // Currently, a workgroup is hardcoded to have 64 threads

        // Make sure bodies is a multiple of 64
        if self.bodies.len() % THREADS_PER_WORKGROUP != 0 {
            panic!(
                "Length of bodies must be a multiple of {}, but is not: {}",
                THREADS_PER_WORKGROUP,
                self.bodies.len()
            );
        }

        // Get WG count
        let wg_count = self.bodies.len() / THREADS_PER_WORKGROUP;

        //

        // Queue up wg X 1 X 1 workgroups
        // If each workgroup works on THREADS_PER_WORKGROUP (64) bodies, then we need to dispatch the proper amount of groups so that
        // We have bodies.len() executions per PASS
        self.pipeline
            .queue_n_passes_alternate_groups(n, wg_count as u32)
            .unwrap();

        // Queue the copy command
        // Binding to make the rust compiler happy
        let input_buffer_binding = self.pipeline.input_buffers.as_ref().unwrap();
        // Get the input buffer ref. Its hard-coded to 0 since we only send the body buffer
        let input_buffer_ref = input_buffer_binding.get(0).unwrap();

        // Get the encoder and setup the copy operation
        let encoder = self.pipeline.encoder.as_mut().unwrap();
        encoder.copy_buffer_to_buffer(
            input_buffer_ref,
            0,
            &self.read_buffer,
            0,
            input_buffer_ref.size(),
        );
    }

    pub async fn run_queued(&mut self) {
        let encoder = self
            .pipeline
            .encoder
            .take()
            .expect("No encoder when trying to run the queue");
        println!("Submitting finish command!");
        self.start_instant = Some(Instant::now());
        self.pipeline.queue.submit([encoder.finish()]);
        // Wait till the submitted queue is done
        println!("Waiting until submitted command is done");
    }

    pub async fn get_bodies(&mut self) -> Result<Vec<Body>, String> {
        // Get a slice of that buffer
        let read_buffer_slice = self.read_buffer.slice(..);

        // create an MPSC channel to send
        let (data_sender, data_receiver) = channel();

        // Read everything from the read buffer slice and send it through the channel
        read_buffer_slice.map_async(wgpu::MapMode::Read, move |v| {
            data_sender.send(v).expect("Send failed")
        });

        let _poll_result = self.pipeline.device.poll(wgpu::PollType::Wait);
        println!("Took {:?}", self.start_instant.unwrap().elapsed());

        // If we recieved okay
        match data_receiver.recv() {
            Ok(Ok(())) => {
                // Get a view into the buffer
                let data = read_buffer_slice.get_mapped_range();

                // Cast the buffer view into a vec of bodies
                let result: Vec<Body> = bytemuck::cast_slice(&*data).to_vec();
                drop(data);
                return Ok(result);
            }
            Ok(Err(e)) => return Err(e.to_string()),
            Err(e) => return Err(e.to_string()),
        }
    }
}
