@group(0) @binding(0) var<storage, read> input: array<u32>;
@group(0) @binding(1) var<storage, read_write> output: array<u32>;

@compute
// Workgroup size?
// This means we define a workgroup by 64 x 1 x 1 threads
@workgroup_size(64,1,1)

fn main(
    @builtin(global_invocation_id) global_invocation_id: vec3<u32>
) {
    // Get the index based on the current invocation id
    // Since our workgroup is one dimensional we just get the x value
    let index = global_invocation_id.x;
    let total = arrayLength(&input);

    // Check out of bounds
    if (index >= total) {
        return;
    }

    // Copy
    output[global_invocation_id.x] = input[global_invocation_id.x] * 2;
}