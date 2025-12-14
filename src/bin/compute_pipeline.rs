use std::sync::mpsc::channel;

use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    wgt::CommandEncoderDescriptor,
    BindGroupDescriptor, ComputePassDescriptor,
};

#[pollster::main]
async fn main() {
    let instance = wgpu::Instance::new(&Default::default());
    let adapter = instance.request_adapter(&Default::default()).await.unwrap();
    let (device, queue) = adapter.request_device(&Default::default()).await.unwrap();

    let shader = device.create_shader_module(wgpu::include_wgsl!("../../shaders/compute.wgsl"));
    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("compute pipeline"),
        layout: None,
        module: &shader,
        entry_point: None,
        compilation_options: Default::default(),
        cache: Default::default(),
    });

    let input_data: Vec<u32> = Vec::from_iter(0..1_000_000);

    // Create the buffers that contain the data
    let input_buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: Some("input buffer"),
        contents: bytemuck::cast_slice(&input_data), // Cast data into [u8]
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
    });

    // Create the output buffer
    let output_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("output buffer"),
        size: input_buffer.size(), // Since we're just copying the data
        usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::STORAGE, //Depends on the modes set within the shader (var<storage,read_write>)
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

    // Create a bindgroup to describe the data going into the pipeline
    // We don't have the manually specify the bindgroup layout, WGPU can infer it based on the shader bindings
    let bind_group = device.create_bind_group(&BindGroupDescriptor {
        label: Some("compute bind group"),
        layout: &pipeline.get_bind_group_layout(0), // Get the layout based on the group instead of defining it manually
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: input_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: output_buffer.as_entire_binding(),
            },
        ],
    });

    // Create a command encoder for the compute pipeline
    let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor { label: None });
    // Divide by 64 because that is the workgroup size
    // If we have N data points, and 64 workgroups, each group gets N/64 points
    let num_dispatches = input_data.len().div_ceil(64) as u32;

    // Create the pass
    // Note: this is in its own closure, since the pass takes a reference to the encoder
    // Since its in a closure, once its over that reference goes away and we can use encoder again
    {
        let mut pass = encoder.begin_compute_pass(&Default::default());
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &bind_group, &[]); //&[] is the offsets, we dont have any
        pass.dispatch_workgroups(num_dispatches, 1, 1); // Dimensions of the amount of work groups
    }

    // Copy the data off the GPU and submit the command to finish the pass
    encoder.copy_buffer_to_buffer(&output_buffer, 0, &temp_buffer, 0, output_buffer.size());
    queue.submit([encoder.finish()]);

    // Create a slice into the temp buffer
    let temp_slice = temp_buffer.slice(..);

    // Create tx and rx so we can read from the future
    let (sender, receiver) = channel();

    // Define the callback that handles the slice by sending the data to the receiver
    // Note that this wont execute until we do poll
    temp_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());

    let _poll_result = device.poll(wgpu::PollType::Wait);

    if let Ok(Ok(())) = receiver.recv() {
        let data = temp_slice.get_mapped_range();

        let result: Vec<u32> = bytemuck::cast_slice(&data).to_vec();

        // Drop the data (mapped view) before we drop the buffer
        drop(data);
        temp_buffer.unmap();

        dbg!(&result.len());
        Some(result);
    } else {
        panic!("Failed to receive data!")
    }
}
