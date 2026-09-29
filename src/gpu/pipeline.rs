use std::fmt::Display;

use bytemuck::{Pod, Zeroable};
use thiserror::Error;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Adapter, BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, Buffer, BufferUsages,
    CommandEncoder, CommandEncoderDescriptor, Device, Instance, PipelineLayout, PushConstantRange,
    Queue, ShaderModule,
};

#[derive(Error, Debug)]
pub enum ComputePipelineError {
    #[error("Could not request adapter")]
    RequestAdapter(#[from] wgpu::RequestAdapterError),
    #[error("Could not request device")]
    RequestDevice(#[from] wgpu::RequestDeviceError),
    #[error("Field not initialized {0}")]
    NotInitialized(ComputePipelineStage),
    #[error("Not ready to run operation {0}")]
    NotReady(String),
    #[error("Invalid index {0}")]
    InvalidIndex(usize),
}

#[derive(Debug)]
pub enum ComputePipelineStage {
    Shader,
    BindgroupLayout,
    InputBuffer,
    OutputBuffer,
    Encoder,
    Pipeline,
}

impl Display for ComputePipelineStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComputePipelineStage::Shader => {
                write!(f, "Shader")
            }
            ComputePipelineStage::BindgroupLayout => {
                write!(f, "Bindgroup layout")
            }
            ComputePipelineStage::InputBuffer => {
                write!(f, "Input buffer")
            }
            ComputePipelineStage::OutputBuffer => {
                write!(f, "Output buffer")
            }
            ComputePipelineStage::Encoder => {
                write!(f, "Encoder")
            }
            ComputePipelineStage::Pipeline => {
                write!(f, "Pipeline")
            }
        }
    }
}

pub type Result<T> = std::result::Result<T, ComputePipelineError>;
pub struct New;
pub struct Shader;
pub struct InputData;

pub struct OutputData;
pub struct Pipeline;
pub struct Bindgroup;

pub struct Encoder;
pub struct Complete;

pub struct ComputePipelineBuilder<State = New> {
    pub state: State,
    pub pipeline: ComputePipeline,
}

impl ComputePipelineBuilder<New> {
    pub async fn new() -> Result<ComputePipelineBuilder<Shader>> {
        Ok(ComputePipelineBuilder {
            state: Shader,
            pipeline: ComputePipeline::new().await?,
        })
    }
}

impl ComputePipelineBuilder<Shader> {
    pub fn build_shader(
        mut self,
        shader: ShaderModule,
    ) -> Result<ComputePipelineBuilder<InputData>> {
        self.pipeline.shader = Some(shader);
        return Ok(ComputePipelineBuilder {
            state: InputData,
            pipeline: self.pipeline,
        });
    }
}

impl ComputePipelineBuilder<InputData> {
    pub fn append_input_data<T: AsRef<[u8]>>(
        &mut self,
        input_data: T,
        usage: BufferUsages,
    ) -> Result<usize> {
        //        self.pipeline.setup_input_data(input_data, usage)
        let descriptor = BufferInitDescriptor {
            label: Some("Input buffer"),
            contents: input_data.as_ref(),
            usage: usage,
        };
        let input_buffer = self.pipeline.device.create_buffer_init(&descriptor);
        match self.pipeline.input_buffers {
            Some(ref mut buffers) => buffers.push(input_buffer),
            None => {
                self.pipeline.input_buffers = Some(vec![input_buffer]);
            }
        }

        return Ok(self.pipeline.input_buffers.iter().len() - 1);
    }

    pub fn done(self) -> Result<ComputePipelineBuilder<OutputData>> {
        return Ok(ComputePipelineBuilder {
            state: OutputData,
            pipeline: self.pipeline,
        });
    }
}

impl ComputePipelineBuilder<OutputData> {
    pub fn append_output_data<T: AsRef<[u8]>>(
        &mut self,
        output_data: T,
        usage: BufferUsages,
    ) -> Result<usize> {
        let descriptor = BufferInitDescriptor {
            label: Some("Output buffer"),
            contents: output_data.as_ref(),
            usage: usage,
        };
        let output_buffer = self.pipeline.device.create_buffer_init(&descriptor);
        match self.pipeline.output_buffers {
            Some(ref mut buffers) => buffers.push(output_buffer),
            None => {
                self.pipeline.output_buffers = Some(vec![output_buffer]);
            }
        }

        return Ok(self.pipeline.output_buffers.iter().len() - 1);
    }

    pub fn done(self) -> Result<ComputePipelineBuilder<Pipeline>> {
        return Ok(ComputePipelineBuilder {
            state: Pipeline,
            pipeline: self.pipeline,
        });
    }
}

impl ComputePipelineBuilder<Pipeline> {
    pub fn build_pipeline(
        mut self,
        _push_constant_ranges: &[PushConstantRange],
    ) -> Result<ComputePipelineBuilder<Bindgroup>> {
        let compute_pipeline_descriptor =
            wgpu::ComputePipelineDescriptor {
                label: Some("Compute pipeline"),
                layout: self.pipeline.pipeline_layout.as_ref(),
                module: self.pipeline.shader.as_ref().ok_or(
                    ComputePipelineError::NotInitialized(ComputePipelineStage::Shader),
                )?,
                entry_point: None,
                compilation_options: Default::default(),
                cache: Default::default(),
            };

        let compute_pipeline = self
            .pipeline
            .device
            .create_compute_pipeline(&compute_pipeline_descriptor);
        self.pipeline.pipeline = Some(compute_pipeline);

        return Ok(ComputePipelineBuilder {
            state: Bindgroup,
            pipeline: self.pipeline,
        });
    }
}

impl ComputePipelineBuilder<Bindgroup> {
    pub fn build_bindgroup(mut self) -> Result<ComputePipelineBuilder<Encoder>> {
        let mut bind_group_entries: Vec<BindGroupEntry> = Vec::new();
        let mut index: u32 = 0;
        // Safety: We already checked at the start of the function if input is some
        for input in self.pipeline.input_buffers.as_ref().unwrap() {
            let entry = BindGroupEntry {
                binding: index,
                resource: input.as_entire_binding(),
            };
            bind_group_entries.push(entry);
            index += 1;
        }

        if self.pipeline.output_buffers.is_some() {
            for output in self.pipeline.output_buffers.as_ref().unwrap() {
                let entry = BindGroupEntry {
                    binding: index,
                    resource: output.as_entire_binding(),
                };
                bind_group_entries.push(entry);
                index += 1;
            }
        }

        let bind_group_descriptor = BindGroupDescriptor {
            label: Some("Compute pipeline bind group"),
            layout: &(self
                .pipeline
                .pipeline
                .as_ref()
                .unwrap()
                .get_bind_group_layout(0)),
            entries: &bind_group_entries,
        };

        let bind_group = self
            .pipeline
            .device
            .create_bind_group(&bind_group_descriptor);

        self.pipeline.bind_group = Some(bind_group);

        return Ok(ComputePipelineBuilder {
            state: Encoder,
            pipeline: self.pipeline,
        });
    }
}

impl ComputePipelineBuilder<Encoder> {
    pub fn setup_encoder(mut self) -> Result<ComputePipelineBuilder<Complete>> {
        let command_encoder_descriptor = &CommandEncoderDescriptor {
            label: Some("Compute pipeline command encoder "),
        };
        let encoder = self
            .pipeline
            .device
            .create_command_encoder(command_encoder_descriptor);
        self.pipeline.encoder = Some(encoder);
        return Ok(ComputePipelineBuilder {
            state: Complete,
            pipeline: self.pipeline,
        });
    }
}

impl ComputePipelineBuilder<Complete> {
    pub fn finalize(self) -> Result<ComputePipeline> {
        return Ok(self.pipeline);
    }
}
pub struct ComputePipeline {
    pub instance: Instance,
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
    pub shader: Option<ShaderModule>,
    pub bind_group_layout: Option<BindGroupLayout>,
    pub bind_group: Option<BindGroup>,
    pub pipeline_layout: Option<PipelineLayout>,
    pub pipeline: Option<wgpu::ComputePipeline>,
    pub input_buffers: Option<Vec<Buffer>>,
    pub output_buffers: Option<Vec<Buffer>>,
    pub encoder: Option<CommandEncoder>,
}

impl ComputePipeline {
    pub async fn new() -> Result<ComputePipeline> {
        let instance = wgpu::Instance::new(&Default::default());
        let adapter = instance.request_adapter(&Default::default()).await?;

        // Specifically create a descriptor with higher limits
        let descriptor = &wgpu::DeviceDescriptor {
            label: Some("Custom descriptor"),
            required_features: wgpu::Features::TIMESTAMP_QUERY,
            required_limits: wgpu::Limits {
                max_buffer_size: adapter.limits().max_buffer_size,
                max_storage_buffer_binding_size: adapter.limits().max_storage_buffer_binding_size,
                ..wgpu::Limits::default()
            },
            memory_hints: Default::default(),
            trace: wgpu::Trace::default(),
        };

        let (device, queue) = adapter.request_device(descriptor).await?;

        Ok(ComputePipeline {
            instance: instance,
            adapter: adapter,
            device: device,
            queue: queue,
            pipeline_layout: None,
            shader: None,
            bind_group_layout: None,
            bind_group: None,
            pipeline: None,
            input_buffers: None,
            output_buffers: None,
            encoder: None,
        })
    }

    pub fn execute_single_pass(&mut self, dispatch_count: u32) -> Result<()> {
        // debug only

        {
            let mut pass = self
                .encoder
                .as_mut()
                .ok_or(ComputePipelineError::NotInitialized(
                    ComputePipelineStage::Encoder,
                ))?
                .begin_compute_pass(&Default::default());
            pass.set_pipeline(self.pipeline.as_ref().ok_or(
                ComputePipelineError::NotInitialized(ComputePipelineStage::Pipeline),
            )?);
            pass.set_bind_group(0, &self.bind_group, &[]); //&[] is the offsets, we dont have any
            pass.dispatch_workgroups(dispatch_count, 1, 1); // Dimensions of the amount of work groups
        }
        Ok(())
    }

    pub fn queue_n_passes(&mut self, pass_count: u32, workgroup_size_x: u32) -> Result<()> {
        {
            let command_encoder =
                self.encoder
                    .as_mut()
                    .ok_or(ComputePipelineError::NotInitialized(
                        ComputePipelineStage::Encoder,
                    ))?;

            let mut pass = command_encoder.begin_compute_pass(&Default::default());
            pass.set_pipeline(self.pipeline.as_ref().ok_or(
                ComputePipelineError::NotInitialized(ComputePipelineStage::Pipeline),
            )?);
            pass.set_bind_group(0, &self.bind_group, &[]); //&[] is the offsets, we dont have any
            for i in 1..pass_count + 1 {
                // println!("Dispatch number {i}");

                pass.dispatch_workgroups(workgroup_size_x, 1, 1); // Dimensions of the amount of work groups
            }
        }
        Ok(())
    }

    pub fn queue_n_passes_alternate_groups(
        &mut self,
        pass_count: u32,
        workgroup_size_x: u32,
    ) -> Result<()> {
        let mut command_encoder =
            self.encoder
                .as_mut()
                .ok_or(ComputePipelineError::NotInitialized(
                    ComputePipelineStage::Encoder,
                ))?;

        let query_set = self.device.create_query_set(&wgpu::QuerySetDescriptor {
            label: Some("compute_timestamp_query_set"),
            count: 2,
            ty: wgpu::QueryType::Timestamp,
        });

        // Buffer to store resolved query data (2 * 8 bytes = 16 bytes)
        let query_resolve_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("query_resolve_buffer"),
            size: 16,
            usage: wgpu::BufferUsages::QUERY_RESOLVE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        // Staging buffer to read results back on CPU
        let cpu_staging_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("cpu_staging_buffer"),
            size: 16,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        // Here is where the magic happens
        // We create two different bindGroups
        // A: input is 0 and output is 1
        // B: input is 1 and output is 0
        // This lets us constantly swap between input and output groups without having to complete the whole pipeline first

        // We have to manually define the bind groups and their layouts as they differ from the default pipeline

        let bindgroup_layout: BindGroupLayout = self
            .pipeline
            .as_ref()
            .expect("No pipeline")
            .get_bind_group_layout(0);
        let bind_group_a = self.device.create_bind_group(&BindGroupDescriptor {
            label: Some("Bind Group A"),
            layout: &bindgroup_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: self
                        .input_buffers
                        .as_ref()
                        .expect("No input buffers")
                        .get(0)
                        .expect("No input buffers")
                        .as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: self
                        .output_buffers
                        .as_ref()
                        .expect("No output buffers")
                        .get(0)
                        .expect("No output buffers")
                        .as_entire_binding(),
                },
            ],
        });

        let bind_group_b = self.device.create_bind_group(&BindGroupDescriptor {
            label: Some("Bind Group B"),
            layout: &bindgroup_layout,
            entries: &[
                BindGroupEntry {
                    binding: 1,
                    resource: self
                        .input_buffers
                        .as_ref()
                        .expect("No input buffers")
                        .get(0)
                        .expect("No input buffers")
                        .as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 0,
                    resource: self
                        .output_buffers
                        .as_ref()
                        .expect("No output buffers")
                        .get(0)
                        .expect("No output buffers")
                        .as_entire_binding(),
                },
            ],
        });

        {
            let mut pass = command_encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("timed_compute_pass"),
                timestamp_writes: Some(wgpu::ComputePassTimestampWrites {
                    query_set: &query_set,
                    beginning_of_pass_write_index: Some(0), // Slot 0 for start
                    end_of_pass_write_index: Some(1),       // Slot 1 for end
                }),
            });
            pass.set_pipeline(self.pipeline.as_ref().ok_or(
                ComputePipelineError::NotInitialized(ComputePipelineStage::Pipeline),
            )?);

            for i in 0..pass_count {
                println!("Dispatch number {i}");
                if i % 2 == 0 {
                    pass.set_bind_group(0, &bind_group_a, &[]); //&[] is the offsets, we dont have any
                } else {
                    pass.set_bind_group(0, &bind_group_b, &[]); //&[] is the offsets, we dont have any
                }
                pass.dispatch_workgroups(workgroup_size_x, 1, 1); // Dimensions of the amount of work groups
            }
        }
        // Resolve the query set into the GPU buffer
        command_encoder.resolve_query_set(&query_set, 0..2, &query_resolve_buffer, 0);

        // Copy to CPU-mappable staging buffer
        command_encoder.copy_buffer_to_buffer(&query_resolve_buffer, 0, &cpu_staging_buffer, 0, 16);
        Ok(())
    }

    // We can move the function for reading data to the compute pipeline itself, that way we dont have ownership issues
    pub fn read_data<T: Pod + Zeroable>(
        &mut self,
        buffer_index: usize,
        read_input: bool,
        byte_count: u64,
    ) -> Result<Vec<T>> {
        // Get the buffer array that we want to read from
        let buffer_binding = match read_input {
            true => self
                .input_buffers
                .as_ref()
                .ok_or(ComputePipelineError::NotInitialized(
                    ComputePipelineStage::InputBuffer,
                ))?,
            false => self
                .output_buffers
                .as_ref()
                .ok_or(ComputePipelineError::NotInitialized(
                    ComputePipelineStage::OutputBuffer,
                ))?,
        };

        // Get the buffer based on the index passed
        let buffer_ref = buffer_binding
            .get(buffer_index)
            .ok_or(ComputePipelineError::InvalidIndex(buffer_index))?;

        // Create a temporary buffer
        let temp_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("temp buffer for output"),
            size: byte_count,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        // The encoder needs to be owned for the finish operation, so maybe we dont create it until we are here?
        let mut encoder = self.encoder.take().unwrap();
        encoder.copy_buffer_to_buffer(buffer_ref, 0, &temp_buffer, 0, buffer_ref.size());

        let finish = encoder.finish();
        self.queue.submit([finish]);
        todo!();
    }
}
