
struct Body {
    pos: vec2<f32>,
    acceleration: vec2<f32>,
    velocity: vec2<f32>,
    force: vec2<f32>,
    mass: f32,
    padding: f32 // body must be aligned by a multiple of the largest variable, so 8 bytes in this case
}
const EPSILON: f32 = 1.0;
const G: f32 = 1.00;

@group(0) @binding(0) var<storage, read> bodies: array<Body>;
@group(0) @binding(1) var<storage, read_write> bodies_out: array<Body>;
const WORKGROUP_SIZE_X: u32 = 16;
const WORKGROUP_SIZE_Y: u32 = 1;
const WORKGROUP_SIZE_Z: u32 = 1;
const WORKGROUP_SIZE = WORKGROUP_SIZE_X*WORKGROUP_SIZE_Y*WORKGROUP_SIZE_Z;
@compute
@workgroup_size(WORKGROUP_SIZE_X,WORKGROUP_SIZE_Y,WORKGROUP_SIZE_Z)

fn main(
    @builtin(global_invocation_id) global_invocation_id: vec3<u32>,
    @builtin(workgroup_id) workgroup_id: vec3<u32>,
    @builtin(local_invocation_index) local_invocation_index: u32,
    @builtin(num_workgroups) num_workgroups: vec3<u32> 
) {

    // Get current workgroup index
    let workgroup_index = workgroup_id.x;

    // Global index
    let global_invocation_index = (workgroup_index * WORKGROUP_SIZE) + local_invocation_index;

    // Calculate the thread count and the chunk size
    let array_len = arrayLength(&bodies);
    let workgroup_count = num_workgroups.x * num_workgroups.y * num_workgroups.z;
    let thread_count = workgroup_count * WORKGROUP_SIZE;
    let chunk_size = array_len/thread_count;

    // Get start and end
    let start = chunk_size * global_invocation_index;
    let end = chunk_size * (global_invocation_index + 1);

    for (var current_body_index = start; current_body_index < end; current_body_index++) {
        // Calculate the force on ALL the bodies
        var current_body: Body = bodies[current_body_index];
        var force: vec2<f32> = vec2<f32>(0.0,0.0);
        for(var other_body_index = u32(0); other_body_index < (array_len); other_body_index++){
            if(current_body_index == other_body_index) {
                continue;
            }
            let other_body: Body = bodies[other_body_index];


            // Do some force operations
            let distance = current_body.pos - other_body.pos;
            let distance_magnitude = sqrt((distance.x * distance.x) + (distance.y * distance.y) + EPSILON);
            let current_force = distance * ((G * current_body.mass * other_body.mass)/(pow(distance_magnitude,3)));
            force += current_force;
        }
        current_body.force = force;
        bodies_out[current_body_index] = current_body;
    }
}