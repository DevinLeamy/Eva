@group(0) @binding(0) var colour_buffer: texture_storage_2d<rgba16float, write>;

@compute @workgroup_size(8, 8, 1)
fn compute_main(@builtin(global_invocation_id) GlobalInvocationID: vec3<u32>) {
    let screen_coord = vec2<i32>(i32(GlobalInvocationID.x), i32(GlobalInvocationID.y));

    textureStore(colour_buffer, screen_coord, vec4<f32>(255.0, 0.0, 0.0, 1.0));
}
