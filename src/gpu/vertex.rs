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
            // Tell the GPU how to read the vertex struct
            // the first 3 f32's are right at the start
            // the next 3 f32's are right after that
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0, // index in the shader to access this
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

// Clip coordinates
// X →
// Y ↑

//     -1.0   -0.5    0.0    0.5    1.0
// 1.0                 |
//                     |
// 0.5                 |
//                     |
// 0.0 ----------------+----------------
//                     |
// -0.5                |
//                     |
// -1.0                |



// REMEMBER CCW!
// TOP, BOTTOM LEFT, BOTTOM RIGHT
pub const VERTICES: &[Vertex] = &[
    Vertex { position: [0.0, 0.75, 0.0], color: [0.0, 1.0, 0.0] }, 
    Vertex { position: [-0.5, 0.25, 0.0], color: [0.0, 0.0, 1.0] }, 
    Vertex { position: [0.0, 0.25, 0.0], color: [1.0, 0.0, 0.0] }, 
   
    Vertex { position: [0.0, 0.25, 0.0], color: [0.0, 1.0, 0.0] }, 
    Vertex { position: [-0.5, 0.25, 0.0], color: [1.0, 0.0, 0.0] }, 
    Vertex { position: [0.0, -0.25, 0.0], color: [0.0, 0.0, 1.0] }, 

    Vertex { position: [-0.5, 0.25, 0.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [-0.5, -0.25, 0.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [0.0, -0.25, 0.0], color: [0.0, 0.0, 1.0] },

    Vertex { position: [0.0, -0.25, 0.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [-0.5, -0.25, 0.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [0.0, -0.75, 0.0], color: [0.0, 0.0, 1.0] },

    
    Vertex { position: [0.0, 0.75, 0.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [0.0, 0.25, 0.0], color: [0.0, 0.0, 1.0] },
    Vertex { position: [0.5, 0.25, 0.0], color: [1.0, 0.0, 0.0] },


    Vertex { position: [0.5, 0.25, 0.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [0.0, 0.25, 0.0], color: [0.0, 0.0, 1.0] },
    Vertex { position: [0.0, -0.25, 0.0], color: [1.0, 0.0, 0.0] },


    Vertex { position: [0.5, 0.25, 0.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [0.0, -0.25, 0.0], color: [0.0, 0.0, 1.0] },
    Vertex { position: [0.5, -0.25, 0.0], color: [1.0, 0.0, 0.0] },

    Vertex { position: [0.5, -0.25, 0.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [0.0, -0.25, 0.0], color: [0.0, 0.0, 1.0] },
    Vertex { position: [0.0, -0.75, 0.0], color: [1.0, 0.0, 0.0] },

];

pub(crate) const INDICES: &[u16] = &[
    0, 1, 2,
    2, 1, 5,
    1, 7, 5,
    5, 7, 11,
    
    0, 2, 14,
    14, 2, 5,
    14, 5, 20,
    20, 5, 11
    // 1, 3, 2
];
