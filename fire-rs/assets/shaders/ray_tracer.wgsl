@group(0) @binding(0) var colour_buffer: texture_storage_2d<rgba16float, write>;

@compute @workgroup_size(8, 8, 1)
fn compute_main(@builtin(global_invocation_id) GlobalInvocationID: vec3<u32>) {
    let screen_size: vec2<i32> = vec2<i32>(textureDimensions(colour_buffer));

    let x = sin(f32(GlobalInvocationID.x) / f32(screen_size.x));
    let y = sin(f32(GlobalInvocationID.y) / f32(screen_size.y));
    let z = x + y;

    let screen_coord = vec2<i32>(i32(GlobalInvocationID.x), i32(GlobalInvocationID.y));

    textureStore(colour_buffer, screen_coord, vec4<f32>(x, y, z, 1.0));
}
