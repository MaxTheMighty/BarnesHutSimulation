use std::fmt::Display;

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

        return Ok(self.pipeline.output_buffers.as_ref().unwrap().len() - 1);
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
        let (device, queue) = adapter.request_device(&Default::default()).await?;

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

    // pub fn setup_shader_module(&mut self, shader: ShaderModule) -> Result<()> {
    //     self.shader = Some(shader);
    //     Ok(())
    // }

    // // We might not need to do this?
    // pub fn setup_bind_groups(&mut self) -> Result<()> {
    //     // if self.input_buffers.is_none() && self.output_buffers.is_none() {
    //     //     return Err(ComputePipelineError::NotInitialized(
    //     //         "Input or output buffers".to_string(),
    //     //     ));
    //     // }

    //     // if self.pipeline.is_none() {
    //     //     return Err(ComputePipelineError::NotInitialized("Pipeline".to_string()));
    //     // }

    //     // Bind group layouts:
    //     // all the inputs are bound 0 to J
    //     // all the outputs are bound J+1 to N

    //     let mut bind_group_entries: Vec<BindGroupEntry> = Vec::new();
    //     let mut index: u32 = 0;
    //     // Safety: We already checked at the start of the function if input is some
    //     for input in self.input_buffers.as_ref().unwrap() {
    //         let entry = BindGroupEntry {
    //             binding: index,
    //             resource: input.as_entire_binding(),
    //         };
    //         bind_group_entries.push(entry);
    //         index += 1;
    //     }

    //     for output in self.output_buffers.as_ref().unwrap() {
    //         let entry = BindGroupEntry {
    //             binding: index,
    //             resource: output.as_entire_binding(),
    //         };
    //         bind_group_entries.push(entry);
    //         index += 1;
    //     }

    //     let bind_group_descriptor = BindGroupDescriptor {
    //         label: Some("Compute pipeline bind group"),
    //         layout: &(self.pipeline.as_ref().unwrap().get_bind_group_layout(0)),
    //         entries: &bind_group_entries,
    //     };

    //     let bind_group = self.device.create_bind_group(&bind_group_descriptor);
    //     self.bind_group = Some(bind_group);
    //     return Ok(());
    // }

    // // Currently unneeded as we automatically infer the layouts, may have to change in the future
    // fn setup_pipeline_layout(&mut self, push_constant_ranges: &[PushConstantRange]) -> Result<()> {
    //     let bind_group_layouts =
    //         [&self
    //             .bind_group_layout
    //             .clone()
    //             .ok_or(ComputePipelineError::NotInitialized(
    //                 ComputePipelineStage::BindgroupLayout,
    //             ))?];

    //     let pipeline_layout_descriptor = &PipelineLayoutDescriptor {
    //         bind_group_layouts: &bind_group_layouts,
    //         label: None,
    //         push_constant_ranges: push_constant_ranges,
    //     };

    //     let pipeline_layout = self
    //         .device
    //         .create_pipeline_layout(&pipeline_layout_descriptor);

    //     self.pipeline_layout = Some(pipeline_layout);
    //     Ok(())
    // }

    // pub fn setup_pipeline(&mut self) -> Result<()> {
    //     let compute_pipeline_descriptor = wgpu::ComputePipelineDescriptor {
    //         label: Some("Compute pipeline"),
    //         layout: self.pipeline_layout.as_ref(),
    //         module: self
    //             .shader
    //             .as_ref()
    //             .ok_or(ComputePipelineError::NotInitialized(
    //                 ComputePipelineStage::Shader,
    //             ))?,
    //         entry_point: None,
    //         compilation_options: Default::default(),
    //         cache: Default::default(),
    //     };

    //     let pipline = self
    //         .device
    //         .create_compute_pipeline(&compute_pipeline_descriptor);
    //     self.pipeline = Some(pipline);

    //     Ok(())
    // }

    // pub fn setup_input_data<T: AsRef<[u8]>>(&mut self, data: T, usage: BufferUsages) -> Result<()> {
    //     let descriptor = BufferInitDescriptor {
    //         label: Some("Input buffer"),
    //         contents: data.as_ref(),
    //         usage: usage,
    //     };
    //     let input_buffer = self.device.create_buffer_init(&descriptor);

    //     match self.input_buffers {
    //         Some(ref mut buffers) => buffers.push(input_buffer),
    //         None => {
    //             self.input_buffers = Some(vec![input_buffer]);
    //         }
    //     }

    //     Ok(())
    // }

    // pub fn setup_output_data<T: AsRef<[u8]>>(
    //     &mut self,
    //     data: T,
    //     usage: BufferUsages,
    // ) -> Result<()> {
    //     let descriptor = BufferInitDescriptor {
    //         label: Some("Output buffer"),
    //         contents: data.as_ref(),
    //         usage: usage,
    //     };
    //     let output_buffer = self.device.create_buffer_init(&descriptor);

    //     match self.output_buffers {
    //         Some(ref mut buffers) => buffers.push(output_buffer),
    //         None => {
    //             self.output_buffers = Some(vec![output_buffer]);
    //         }
    //     }

    //     Ok(())
    // }

    // pub fn setup_encoder(&mut self) -> Result<()> {
    //     let command_encoder_descriptor = &CommandEncoderDescriptor {
    //         label: Some("Compute pipeline command encoder "),
    //     };
    //     let encoder = self
    //         .device
    //         .create_command_encoder(command_encoder_descriptor);
    //     self.encoder = Some(encoder);
    //     Ok(())
    // }

    // fn ready_to_compute(&self) -> bool {
    //     self.pipeline.is_some()
    //         && self.input_buffers.is_some()
    //         && self.output_buffers.is_some()
    //         && self.encoder.is_some()
    //         && self.bind_group.is_some()
    // }

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

    pub fn execute_n_passes(&mut self, dispatch_count: u32, pass_count: u32) -> Result<()> {
        {
            let command_encoder =
                self.encoder
                    .as_mut()
                    .ok_or(ComputePipelineError::NotInitialized(
                        ComputePipelineStage::Encoder,
                    ))?;
            for i in 1..pass_count + 1 {
                println!("Dispatch number {i}");
                let mut pass = command_encoder.begin_compute_pass(&Default::default());
                pass.set_pipeline(self.pipeline.as_ref().ok_or(
                    ComputePipelineError::NotInitialized(ComputePipelineStage::Pipeline),
                )?);
                pass.set_bind_group(0, &self.bind_group, &[]); //&[] is the offsets, we dont have any
                pass.dispatch_workgroups(dispatch_count, 1, 1); // Dimensions of the amount of work groups
            }
        }
        Ok(())
    }
}
