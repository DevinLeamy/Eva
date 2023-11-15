struct Camera {
    /// Camera to world matrix.
    camera_to_world: mat4x4f,
    position: vec3f,
    fov: f32,
};

struct Ray {
    origin: vec3f,
    direction: vec3f,
};

struct Sphere {
    radius: f32
};

struct Intersection {
    some: bool,
    t: f32, 
    ray: Ray,
    normal: vec3f,
};

@group(0) @binding(0) var colour_buffer: texture_storage_2d<rgba16float, write>;
@group(0) @binding(1) var<uniform> camera: Camera;

@compute @workgroup_size(8, 8, 1)
fn compute_main(@builtin(global_invocation_id) GlobalInvocationID: vec3<u32>) {
    let screen_size: vec2<i32> = vec2<i32>(textureDimensions(colour_buffer));

    let x = f32(GlobalInvocationID.x);
    let y = f32(GlobalInvocationID.y);

    let screen_coord = vec2<i32>(i32(GlobalInvocationID.x), i32(GlobalInvocationID.y));
    let pixel_position = compute_pixel_position(x, y);

    let ray = ray_from_points(camera.position, pixel_position);
    let ray_colour = compute_ray_colour(ray);

    textureStore(colour_buffer, screen_coord, vec4<f32>(ray_colour, 1.0));
}

fn compute_pixel_position(x: f32, y: f32) -> vec3f {
    let screen_size = vec2i(textureDimensions(colour_buffer));

    // Distance to the screen.
    let d = 1.0;

    let width = f32(screen_size.x);
    let height = f32(screen_size.y);
    let aspect = width / height;

    // Convert the pixel coordinates to NDC coordinates.
    //
    // We add 0.5 to x and y to get the center of the pixel.
    let ndc_x = (x + 0.5) / width;
    let ndc_y = (y + 0.5) / height;

    // Convert the NDC coordinates to Screen coordinates.
    let screen_x = (ndc_x - 0.5) * 2.0 * aspect;
    let screen_y = (ndc_y - 0.5) * 2.0 * -1.0;

    // Convert the Screen coordinates to Camera coordinates.
    let tan_half_fov = tan(radians(camera.fov) / 2.0);
    let camera_x = screen_x * tan_half_fov * d;
    let camera_y = screen_y * tan_half_fov * d;

    let pixel_camera_pos = vec4f(camera_x, camera_y, d, 1.0);

    // Convert the Camera coordinates to World coordinates.
    let pixel_world_pos = camera.camera_to_world * pixel_camera_pos;

    return pixel_world_pos.xyz;
}


fn ray_from_points(src: vec3f, dest: vec3f) -> Ray {
    var ray: Ray;
    ray.origin = src;
    ray.direction = normalize(dest - src);
    
    return ray;
}

fn ray_point(ray: Ray, t: f32) -> vec3f {
    return ray.origin + ray.direction * t;
}

fn compute_ray_colour(ray: Ray) -> vec3f {
    var sphere: Sphere;
    sphere.radius = 1.0;

    let sphere_position = vec3f(0.0, 0.0, -5.0);

    var transformed_ray: Ray = ray;
    transformed_ray.origin = transformed_ray.origin - sphere_position;

    let intersection = sphere_intersection(sphere, transformed_ray);

    if (intersection.some) {
        return vec3f(1.0, 0.0, 0.0);
    } else {
        return vec3f(0.0, 0.0, 1.0);
    }
}


fn sphere_intersection(sphere: Sphere, ray: Ray) -> Intersection {
    var intersection: Intersection;
    intersection.some = false; 


    let a = 1.0;
    let b = 2.0 * dot(ray.origin, ray.direction);
    let c = dot(ray.origin, ray.origin) - sphere.radius * sphere.radius;

    let disc = b * b - 4.0 * a * c;
    if (disc < 0.0) {
        // No intersection.
        return intersection;
    }

    var t: f32 = 0.0;


    if (disc == 0.0) {
        // One intersection.
        t = -b / (2.0 * a);
    } else {
        // Two intersections.
        let t0 = (-b + sqrt(disc)) / (2.0 * a);
        let t1 = (-b - sqrt(disc)) / (2.0 * a);

        if (t0 <= 0.0) {
            t = t1;
        } else if (t1 <= 0.0) {
            t = t0;
        } else {
            t = min(t0, t1);
        }
    }

    if (t <= 0.0) {
        return intersection;
    }

    let point = ray_point(ray, t);
    let surface_normal = normalize(point);

    intersection.some = true;
    intersection.ray = ray;
    intersection.t = t;
    intersection.normal = surface_normal;

    return intersection;
}

