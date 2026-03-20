
struct Body {
    pos: vec2<f32>,
    acceleration: vec2<f32>,
    velocity: vec2<f32>,
    force: vec2<f32>,
    mass: f32,
    padding: f32 // body must be aligned by a multiple of the largest variable, so 8 bytes in this case
}
const EPSILON: f32 = 0.001;
const G: f32 = 1.00;
const DT: f32 = 0.001;

@group(0) @binding(0) var<storage, read_write> bodies: array<Body>;
// @group(0) @binding(1) var<storage, read_write> bodies_out: array<Body>;
const WORKGROUP_SIZE_X: u32 = 64;
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
    let global_invocation_index = global_invocation_id.x; 

    // // Calculate the thread count and the chunk size
    // let array_len = arrayLength(&bodies);
    // let workgroup_count = num_workgroups.x * num_workgroups.y * num_workgroups.z;
    // let thread_count = workgroup_count * WORKGROUP_SIZE;
    // let chunk_size = array_len/thread_count;

    // // Get start and end
    // let start = chunk_size * global_invocation_index;
    // let end = chunk_size * (global_invocation_index + 1);
    if (global_invocation_index >= arrayLength(&bodies)) {
        return;
    }
    // Go through the slice of bodies we have 
    // For each body we do the following
    // 1. Get the current body from the body array using the calculated index (current_body_index)
    // 2. Iterate through ALL of the other bodies in the input (other_body)
    // 3. Calculate the force between the current body and the other body
    // 4. Apply the force to the current body

   
        var current_body: Body = bodies[global_invocation_index];
        current_body.force= vec2<f32>(0.0,0.0);
        // current_body.padding += 1.0f;
        // // Go through ALL the bodies 
        for(var other_body_index = u32(0); other_body_index < arrayLength(&bodies); other_body_index++){
            if(global_invocation_index == other_body_index) {
                continue;
            }

            // Get the other body
            var other_body: Body = bodies[other_body_index];

            let diff_x = (current_body.pos.x - other_body.pos.x);
            let diff_y = (current_body.pos.y - other_body.pos.y);
            // if (diff_x > current_body.padding) {
            //     current_body.padding = diff_x;
            // }
            let r2 = ((diff_x * diff_x) + (diff_y * diff_y)) + EPSILON;
            let inv_r = inverseSqrt(r2);
            let inv_r3 = inv_r * inv_r * inv_r;
            let mi = current_body.mass;
            let mj = other_body.mass;
            let current_force_x = ((G * mi * mj) * inv_r3)  * diff_x;
            let current_force_y = ((G * mi * mj) * inv_r3) * diff_y;
            current_body.force.x -= (current_force_x);
            current_body.force.y -= (current_force_y);
        }

        // make all bodies wait till calculations are done
        workgroupBarrier();

        // integrate self
        current_body.acceleration = current_body.force / current_body.mass;
        current_body.velocity += current_body.acceleration * DT; 
        current_body.pos += current_body.velocity * DT;
        current_body.force.x = 0.0;
        current_body.force.y = 0.0;
        current_body.padding += 1.0f;
        // Reassign
        bodies[global_invocation_index] = current_body;
}