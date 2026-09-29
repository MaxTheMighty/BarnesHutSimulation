use std::{sync::mpsc::channel, time::Instant};

use barnes_hut::body::Body;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    wgt::CommandEncoderDescriptor,
    BindGroupDescriptor, BindGroupLayoutDescriptor, BindGroupLayoutEntry, PipelineLayoutDescriptor,
    ShaderStages,
};

const WORKGROUP_SIZE: usize = 64;
#[tokio::main]
async fn main() {
    let instance = wgpu::Instance::new(&Default::default());
    let adapter = instance.request_adapter(&Default::default()).await.unwrap();
    let (device, queue) = adapter.request_device(&Default::default()).await.unwrap();

    let shader = device.create_shader_module(wgpu::include_wgsl!("../../shaders/n-body.wgsl"));

    let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("compute bind group layout"),
        entries: &[
            BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            BindGroupLayoutEntry {
                binding: 1,
                visibility: ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            // BindGroupLayoutEntry {
            //     binding: 2,
            //     visibility: ShaderStages::COMPUTE,
            //     ty: wgpu::BindingType::Buffer {
            //         ty: wgpu::BufferBindingType::Storage { read_only: false },
            //         has_dynamic_offset: false,
            //         min_binding_size: None,
            //     },
            //     count: None,
            // },
        ],
    });

    let pipeline_layout_unwrapped = &device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: Some("compute pipeline layout"),
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });
    let pipeline_layout = Some(pipeline_layout_unwrapped);

    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("compute pipeline"),
        layout: pipeline_layout,
        module: &shader,
        entry_point: None,
        compilation_options: Default::default(),
        cache: Default::default(),
    });

    // If its not a multiple of 64, our rending gets messed up and we have slighly more thread executions than datapoints
    let mut input_data: Vec<Body> = Vec::new();
    for _i in 0..1_000_000 {
        input_data.push(Body::random(0.0, 20.0));
    }

    let input_data_bytes: Vec<u8> = bytemuck::cast_slice(input_data.as_slice()).to_vec();

    // Create the buffers that contain the data
    let input_buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: Some("input buffer"),
        contents: bytemuck::cast_slice(&input_data_bytes), // Cast data into [u8]
        usage: wgpu::BufferUsages::COPY_DST
            | wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_SRC,
    });

    // Create the buffers that contain the data
    let output_buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: Some("output buffer"),
        contents: bytemuck::cast_slice(&input_data_bytes), // Cast data into [u8]
        usage: wgpu::BufferUsages::COPY_DST
            | wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_SRC,
    });

    // Debug buffer
    let debug_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("debug buffer"),
        size: input_buffer.size(),
        usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::STORAGE,
        mapped_at_creation: false,
    });

    // Create the temporary buffer
    // Note that this is seperate from the output buffer because we need MAP_READ, which cant exist along side STORAGE,
    // Which our output buffer has
    let temp_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("temp buffer"),
        size: input_buffer.size(),
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ, // Must be here
        mapped_at_creation: false,
    });

    let temp_debug_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("temp debug buffer"),
        size: debug_buffer.size(),
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    // Create a bindgroup to describe the data going into the pipeline
    // We don't have the manually specify the bindgroup layout, WGPU can infer it based on the shader bindings
    let bind_group = device.create_bind_group(&BindGroupDescriptor {
        label: Some("compute bind group"),
        layout: &bind_group_layout, // Get the layout based on the group instead of defining it manually
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: input_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: output_buffer.as_entire_binding(),
            },
            // wgpu::BindGroupEntry {
            //     binding: 2,
            //     resource: debug_buffer.as_entire_binding(),
            // },
        ],
    });

    // Create a command encoder for the compute pipeline
    let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor { label: None });
    // Divide by 64 because that is the workgroup size
    // If we have N data points, and 64 workgroups, each group gets N/64 points
    let num_dispatches = input_data.len().div_ceil(WORKGROUP_SIZE) as u32;
    println!("Dispatching {num_dispatches} workgroups with {WORKGROUP_SIZE} threads each");
    println!(
        "Total threads: {:?}",
        num_dispatches * WORKGROUP_SIZE as u32
    );
    if num_dispatches as usize * WORKGROUP_SIZE > input_data.len() {
        eprintln!("!!! Warning: there are more threads than datapoints !!!");
    }
    // let formatted_str = format!("Expected: {:>width$}", expected, width = 12);
    // println!("{}", formatted_str);
    // Create the pass
    // Note: this is in its own closure, since the pass takes a reference to the encoder
    // Since its in a closure, once its over that reference goes away and we can use encoder again

    // This is where we repeatedly execute
    // Create a timestamp for easier benchmarking
    // Create query set for 2 timestamps (beginning and end)
    {
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("timed compute pass"),
            timestamp_writes: Some(wgpu::ComputePassTimestampWrites {
                query_set: &query_set,
                beginning_of_pass_write_index: Some(0),
                end_of_pass_write_index: Some(1),
            }),
        });
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &bind_group, &[]); //&[] is the offsets, we dont have any
        pass.dispatch_workgroups(num_dispatches, 1, 1); // Dimensions of the amount of work groups
    }

    // Copy the data off the GPU and submit the command to finish the pass
    encoder.copy_buffer_to_buffer(&input_buffer, 0, &temp_buffer, 0, temp_buffer.size());
    // println("Coppying between ")
    // encoder.copy_buffer_to_buffer(&debug_buffer, 0, &temp_debug_buffer, 0, debug_buffer.size());
    queue.submit([encoder.finish()]);
    println!("Elapsed shader run time: {} ms", duration.as_millis());
    println!("Finished compute pass");

    // Create a slice into the temp buffer
    let temp_slice = temp_buffer.slice(..);
    let temp_debug_slice = temp_debug_buffer.slice(..);

    // Create tx and rx so we can read from the future
    let (data_sender, data_receiver) = channel();
    // let (debug_sender, debug_receiver) = channel();
    // Define the callback that handles the slice by sending the data to the receiver
    // Note that this wont execute until we do poll
    temp_slice.map_async(wgpu::MapMode::Read, move |v| data_sender.send(v).unwrap());
    // temp_debug_slice.map_async(wgpu::MapMode::Read, move |v| debug_sender.send(v).unwrap());
    let _poll_result = device.poll(wgpu::PollType::Wait);

    if let Ok(Ok(())) = data_receiver.recv() {
        let data = temp_slice.get_mapped_range();
        // debug_receiver.recv();
        // let debug_data = temp_debug_slice.get_mapped_range();
        let result: Vec<Body> = bytemuck::cast_slice(&*data).to_vec();
        // let debug_vec: Vec<u32> = bytemuck::cast_slice(&*debug_data).to_vec();
        drop(data);
        // drop(debug_data);
        println!("Result bodies length {:?}", result.len());
        println!("Comparing bodies...");
        let mut index: usize = 0;
        for (result_body, expected_body) in result.iter().zip(input_data) {
            // if *result_body != expected_body && (*result_body).mass != expected_body.mass + 1.0 {
            //     println!("Bodies don't match!");
            //     println!("Index {index}");
            //     println!("Result:   {:?}", result_body);
            //     println!("Expected: {:?}", expected_body);
            // }
            // println!("Result body       {:?}",result_body);
            // println!("Non-iterated body {:?}",expected_body);
            index += 1;
        }
    } else {
        panic!("Failed to receive data!")
    }
    return ();
}
