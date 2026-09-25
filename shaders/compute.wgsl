@group(0) @binding(0) var<storage, read> input: array<u32>;
@group(0) @binding(1) var<storage, read_write> output: atomic<u32>;
@group(0) @binding(2) var<storage, read_write> debug: array<atomic<u32>>;

const WORKGROUP_SIZE_X: u32 = 64;
const WORKGROUP_SIZE_Y: u32 = 1;
const WORKGROUP_SIZE_Z: u32 = 1;
const WORKGROUP_SIZE = WORKGROUP_SIZE_X*WORKGROUP_SIZE_Y*WORKGROUP_SIZE_Z;
@compute
// Workgroup size?
// This means we define a workgroup by 10 x 1 x 1 threads
@workgroup_size(WORKGROUP_SIZE_X,WORKGROUP_SIZE_Y,WORKGROUP_SIZE_Z)

fn main(
    @builtin(global_invocation_id) global_invocation_id: vec3<u32>,
    @builtin(workgroup_id) workgroup_id: vec3<u32>,
    @builtin(local_invocation_index) local_invocation_index: u32,
    @builtin(num_workgroups) num_workgroups: vec3<u32> 
) {

    // Get the index of the current workgroup
    let workgroup_index = workgroup_id.x;

    // Get the global invocation index like so:
    // current work group * workgroup size + local invocation
    // ex workgroup 0 local_invocation 1 =
    // 0 * 10 + 1
    let global_invocation_index = (workgroup_index * WORKGROUP_SIZE) + local_invocation_index;
    // Get range
    let total = arrayLength(&input);


    let total_workgroups = num_workgroups.x * num_workgroups.y * num_workgroups.z;
    let total_executions = total_workgroups * WORKGROUP_SIZE;
    // Chunk size = (length/num_workgroups) 
    let chunk_size = total / total_executions;

    // Range = (chunk_size) * glo_index to (chunk_size * (glo_index + 1))-1
    let start = chunk_size * global_invocation_index;

    // Do we need the clamp?
    let end = (chunk_size * (global_invocation_index + 1));

    for (var i = start; i < end; i++) {
        atomicAdd(&output,input[i]);
        atomicAdd(&debug[i],1);
    }
}