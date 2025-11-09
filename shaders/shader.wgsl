// Vertex shader

// Update the input struct to read the vertex data
// This is aligned with how we defined the vertex buffer when we created
// the pipeline.
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>
};


/// Normally the vertex shader performs some transformation
/// But since the vertices are in clip space already,
/// This just passes it through
@vertex
fn vs_main(
   model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color;
    out.clip_position = vec4<f32>(model.position, 1.0);
    return out;
}

// Fragment shader

/// The colors are already interpolated so do nothing

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}

