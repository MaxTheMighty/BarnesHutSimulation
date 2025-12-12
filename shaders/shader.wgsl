// Vertex shader

// Setup the uniform buffer we passed into the pipeline from earlier
struct CameraUniform {
    view_proj: mat4x4<f32>,
};

// This is group 1 since we defined the texture group first, and then this one
@group(1) @binding(0) 
var<uniform> camera: CameraUniform;

// Update the input struct to read the vertex data
// This is aligned with how we defined the vertex buffer when we created
// the pipeline.
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>
};


/// Normally the vertex shader performs some transformation
/// But since the vertices are in clip space already,
/// This just passes it through
@vertex
fn vs_main(
   model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.clip_position = camera.view_proj * vec4<f32>(model.position, 1.0);
    return out;
}

// Fragment shader

// These match the binding layout we made in our gpu struct
@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // return vec4<f32>(in.color, 1.0);
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}

