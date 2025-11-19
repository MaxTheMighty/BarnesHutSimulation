#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32;3], // top, bottom left, bottom right
    tex_coords: [f32; 2] // texture coordinates
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
                    format: wgpu::VertexFormat::Float32x2
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
// pub const VERTICES: &[Vertex] = &[
//     Vertex { position: [0.0, 0.75, 0.0], color: [0.0, 1.0, 0.0] }, 
//     Vertex { position: [-0.5, 0.25, 0.0], color: [0.0, 0.0, 1.0] }, 
//     Vertex { position: [0.0, 0.25, 0.0], color: [1.0, 0.0, 0.0] }, 
   
//     Vertex { position: [0.0, 0.25, 0.0], color: [0.0, 1.0, 0.0] }, 
//     Vertex { position: [-0.5, 0.25, 0.0], color: [1.0, 0.0, 0.0] }, 
//     Vertex { position: [0.0, -0.25, 0.0], color: [0.0, 0.0, 1.0] }, 

//     Vertex { position: [-0.5, 0.25, 0.0], color: [1.0, 0.0, 0.0] },
//     Vertex { position: [-0.5, -0.25, 0.0], color: [0.0, 1.0, 0.0] },
//     Vertex { position: [0.0, -0.25, 0.0], color: [0.0, 0.0, 1.0] },

//     Vertex { position: [0.0, -0.25, 0.0], color: [0.0, 1.0, 0.0] },
//     Vertex { position: [-0.5, -0.25, 0.0], color: [1.0, 0.0, 0.0] },
//     Vertex { position: [0.0, -0.75, 0.0], color: [0.0, 0.0, 1.0] },

    
//     Vertex { position: [0.0, 0.75, 0.0], color: [0.0, 1.0, 0.0] },
//     Vertex { position: [0.0, 0.25, 0.0], color: [0.0, 0.0, 1.0] },
//     Vertex { position: [0.5, 0.25, 0.0], color: [1.0, 0.0, 0.0] },


//     Vertex { position: [0.5, 0.25, 0.0], color: [0.0, 1.0, 0.0] },
//     Vertex { position: [0.0, 0.25, 0.0], color: [0.0, 0.0, 1.0] },
//     Vertex { position: [0.0, -0.25, 0.0], color: [1.0, 0.0, 0.0] },


//     Vertex { position: [0.5, 0.25, 0.0], color: [0.0, 1.0, 0.0] },
//     Vertex { position: [0.0, -0.25, 0.0], color: [0.0, 0.0, 1.0] },
//     Vertex { position: [0.5, -0.25, 0.0], color: [1.0, 0.0, 0.0] },

//     Vertex { position: [0.5, -0.25, 0.0], color: [0.0, 1.0, 0.0] },
//     Vertex { position: [0.0, -0.25, 0.0], color: [0.0, 0.0, 1.0] },
//     Vertex { position: [0.0, -0.75, 0.0], color: [1.0, 0.0, 0.0] },

// ];

// Changed
pub(crate) const VERTICES: &[Vertex] = &[
    // Changed
    Vertex { position: [-0.0868241, 0.49240386, 0.0], tex_coords: [0.4131759, 0.00759614], }, // A
    Vertex { position: [-0.49513406, 0.06958647, 0.0], tex_coords: [0.0048659444, 0.43041354], }, // B
    Vertex { position: [-0.21918549, -0.44939706, 0.0], tex_coords: [0.28081453, 0.949397], }, // C
    Vertex { position: [0.35966998, -0.3473291, 0.0], tex_coords: [0.85967, 0.84732914], }, // D
    Vertex { position: [0.44147372, 0.2347359, 0.0], tex_coords: [0.9414737, 0.2652641], }, // E
];


pub(crate) const INDICES: &[u16] = &[
    0,1,4,
    1,2,4,
    2,3,4
];
