#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32;3], // top, bottom left, bottom right
    color: [f32;3] // r g b
}


impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        let vertex_buffer_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32;3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3
                }
            ]
        };

        vertex_buffer_layout
    }
}
pub const VERTICES: &[Vertex] = &[
    Vertex { position: [0.0,   0.5,  0.0], color: [1.0,  0.0,  0.0]},
    Vertex { position: [-0.5, -0.5,  0.0], color: [0.0,  1.0,  0.0]},
    Vertex { position: [0.5,  -0.5,  0.0], color: [0.0,  0.0,  1.0]}
];