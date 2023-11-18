struct Camera {
    /// Camera to world matrix.
    camera_to_world: mat4x4f,
    position: vec3f,
    fov: f32,
};

struct Transform {
    /// Model transformation. 
    m: mat4x4f,
    /// Invervse model translation.
    m_inverse: mat4x4f,
    /// Post-intersection normal transformation.
    m_normal_inverse: mat3x3f,
};

struct PhongMaterial {
    diffuse: vec3f,
    specular: vec3f,
    shininess: f32,
};

struct CubeModels {
    length: u32,
    cubes: array<CubeModel>,
};

struct CubeModel {
    cube: Cube,
    transform: Transform,
    material: PhongMaterial
};

struct Cube {
    min: vec3f,
    max: vec3f,
};

struct SphereModels {
    length: u32,
    spheres: array<SphereModel>,
}

struct SphereModel {
    sphere: Sphere,
    transform: Transform,
    material: PhongMaterial,
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
    material: PhongMaterial,
};

struct PointLights {
    length: u32,
    lights: array<PointLight>,
};

struct PointLight {
    position: vec3f,
    colour: vec3f,
};

struct GlobalConfig {
    ambient_light: vec3f,
};

struct MeshPoints {
    length: u32,
    points: array<vec3f>
};

struct MeshTriangles {
    length: u32,
    // vec3u -> triangle
    triangles: array<vec3u> 
};

struct MeshHeaders {
    length: u32,
    headers: array<MeshModelHeader>
};

struct MeshModelHeader {
    vertex_offset: u32,
    vertex_count: u32,
    triangle_offset: u32,
    triangle_count: u32,
    material: PhongMaterial,
    transform: Transform,
    bounding_box: Cube
};


@group(0) @binding(0) var colour_buffer: texture_storage_2d<rgba16float, write>;
@group(0) @binding(1) var<uniform> camera: Camera;
@group(0) @binding(2) var<storage, read> spheres: SphereModels; 
@group(0) @binding(3) var<storage, read> lights: PointLights;
@group(0) @binding(4) var<uniform> config: GlobalConfig;
@group(0) @binding(5) var<storage, read> cubes: CubeModels; 

@group(1) @binding(0) var<storage, read> mesh_points: MeshPoints;
@group(1) @binding(1) var<storage, read> mesh_triangles: MeshTriangles;
@group(1) @binding(2) var<storage, read> mesh_headers: MeshHeaders;

@compute @workgroup_size(3, 3, 1)
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

fn ray_t(ray: Ray, point: vec3f) -> f32 {
    return length(point - ray.origin);
}

fn ray_inverse_transform(ray: Ray, transform: Transform) -> Ray {
    let p1 = vec4f(ray.origin, 1.0);
    let p2 = vec4f(ray.origin + ray.direction, 1.0);

    let new_p1 = transform.m_inverse * p1;
    let new_p2 = transform.m_inverse * p2;

    return ray_from_points(new_p1.xyz, new_p2.xyz);
}

fn ray_transform(ray: Ray, transform: Transform) -> Ray {
    let p1 = vec4f(ray.origin, 1.0);
    let p2 = vec4f(ray.origin + ray.direction, 1.0);

    let new_p1 = transform.m * p1;
    let new_p2 = transform.m * p2;

    return ray_from_points(new_p1.xyz, new_p2.xyz);
}

fn intersection_transform(intersection: Intersection, transform: Transform) -> Intersection {
    var new_intersection: Intersection;
    new_intersection.some = intersection.some;
    new_intersection.ray = ray_transform(intersection.ray, transform);
    new_intersection.material = intersection.material;

    let new_point = transform.m * vec4f(ray_point(intersection.ray, intersection.t), 1.0);
    new_intersection.t = ray_t(new_intersection.ray, new_point.xyz);
    new_intersection.normal = normalize(transform.m_normal_inverse * intersection.normal);

    return new_intersection;
}

fn compute_ray_colour(_ray: Ray) -> vec3f {
    var ray: Ray = _ray;

    var total_light: vec3f = vec3f(0.0, 0.0, 0.0);
    var reflectance: f32 = 0.5; 
    var total_reflectance: f32 = 0.5; 

    for (var i: i32 = 0; i < 2; i = i + 1) {
        let intersection = compute_ray_intersection(ray);
        if (!intersection.some) {
            // return vec3f(0.1, 0.1, 0.1);
            // return vec3f(0.0, 0.0, 0.0);
            break;
        }

        total_light = total_light + total_reflectance * compute_light_at_intersection(intersection);
        total_reflectance = total_reflectance * reflectance;

        ray = compute_reflected_ray(ray, intersection);
    }

    return total_light;
}

fn compute_reflected_ray(ray: Ray, intersection: Intersection) -> Ray {
    var reflected_ray: Ray;
    reflected_ray.direction = normalize(ray.direction - intersection.normal * 2.0 * dot(ray.direction, intersection.normal));
    reflected_ray.origin = ray_point(ray, intersection.t) + 0.1 * reflected_ray.direction;

    return reflected_ray;
}

fn compute_ray_intersection(ray: Ray) -> Intersection {
    var intersection: Intersection;
    intersection.some = false; 

    // Sphere intersection tests.
    for (var i: i32 = 0; i < i32(spheres.length); i = i + 1) {
        let sphere_model = spheres.spheres[i];
        let sphere = sphere_model.sphere;

        let transformed_ray: Ray = ray_inverse_transform(ray, sphere_model.transform);
        let new_intersection = sphere_intersection(sphere, transformed_ray);
        var transformed_intersection: Intersection = intersection_transform(new_intersection, sphere_model.transform);

        // Set the material of the object that was intersected with.
        transformed_intersection.material = sphere_model.material;
    
        if (transformed_intersection.some) {
            if (!intersection.some) {
                intersection = transformed_intersection;
            } else if (intersection.t >= transformed_intersection.t) {
                // Take the nearer intersection.
                intersection = transformed_intersection;
            }
        }
    }

    // Cube intersection tests.
    for (var i: i32 = 0; i < i32(cubes.length); i = i + 1) {
        let cube_model = cubes.cubes[i];
        let cube = cube_model.cube;

        let transformed_ray: Ray = ray_inverse_transform(ray, cube_model.transform);
        let new_intersection = cube_intersection(cube, transformed_ray);
        var transformed_intersection: Intersection = intersection_transform(new_intersection, cube_model.transform);

        // Set the material of the object that was intersected with.
        transformed_intersection.material = cube_model.material;
    
        if (transformed_intersection.some) {
            if (!intersection.some) {
                intersection = transformed_intersection;
            } else if (intersection.t >= transformed_intersection.t) {
                // Take the nearer intersection.
                intersection = transformed_intersection;
            }
        }
    }
    
    // Mesh intersection tests.
    for (var i: i32 = 0; i < i32(mesh_headers.length); i = i + 1) {
        let mesh = mesh_headers.headers[i];
        let transformed_ray: Ray = ray_inverse_transform(ray, mesh.transform);

        let bounding_box_intersection = cube_intersection(mesh.bounding_box, transformed_ray);
        if (!bounding_box_intersection.some) {
            // Missed the bounding box.
            continue;
        } 

        for (var j: i32 = 0; j < i32(mesh.triangle_count); j = j + 1) {
            let triangle = mesh_triangles.triangles[i32(mesh.triangle_offset) + j];
            let p1 = mesh_points.points[mesh.vertex_offset + triangle.x];
            let p2 = mesh_points.points[mesh.vertex_offset + triangle.y];
            let p3 = mesh_points.points[mesh.vertex_offset + triangle.z];

            let new_intersection = triangle_intersection(p1, p2, p3, transformed_ray);
            var transformed_intersection: Intersection = intersection_transform(new_intersection, mesh.transform);

            // Set the material of the object that was intersected with.
            transformed_intersection.material = mesh.material;
        
            if (transformed_intersection.some) {
                if (!intersection.some) {
                    intersection = transformed_intersection;
                } else if (intersection.t >= transformed_intersection.t) {
                    // Take the nearer intersection.
                    intersection = transformed_intersection;
                }
            }
        }
    }

    return intersection;
}

fn compute_light_at_intersection(intersection: Intersection) -> vec3f {
    if (!intersection.some) {
        return vec3f(0.0, 0.0, 0.0);
    }

    var total_light: vec3f = vec3f();

    for (var i: i32 = 0; i < i32(lights.length); i = i + 1) {
        let light = lights.lights[i];
        total_light = total_light + compute_light_contribution_at_intersection(intersection, light);
    }

    return total_light;
}

fn compute_light_contribution_at_intersection(intersection: Intersection, light: PointLight) -> vec3f {
    if (!intersection.some) {
        return vec3f(0.0, 0.0, 0.0);
    }

    let intersection_point = ray_point(intersection.ray, intersection.t);
    let to_light = normalize(light.position - intersection_point); 

    // Move lightly away from the intersection point in the direction of the light to avoid
    // intersecting with the inside of an object.
    let offset_intersection_point = intersection_point + to_light * 0.1;
    let ray_to_light = ray_from_points(offset_intersection_point, light.position);

    let ray_to_light_intersection = compute_ray_intersection(ray_to_light);
    return phong_illumination(intersection, light, ray_to_light_intersection.some);
}

fn phong_illumination(intersection: Intersection, light: PointLight, in_shadow: bool) -> vec3f {
    let ambient = config.ambient_light * light.colour * intersection.material.diffuse;
    if (in_shadow) {
        return ambient;
    }

    let intersection_point = ray_point(intersection.ray, intersection.t);
    let to_light = normalize(light.position - intersection_point);
    let to_view = normalize(camera.position - intersection_point);


    let diffuse_strength = max(0.0, dot(intersection.normal, to_light));
    let diffuse = diffuse_strength * light.colour * intersection.material.diffuse;

    let half_vector = normalize(to_view + to_light);
    let specular_strength = pow(max(0.0, dot(intersection.normal, half_vector)), intersection.material.shininess);
    let specular = light.colour * specular_strength * intersection.material.specular;

    return diffuse + specular + ambient;
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

fn cube_intersection(cube: Cube, ray: Ray) -> Intersection {
    let min = cube.min;
    let max = cube.max;

    var intersection: Intersection;
    intersection.some = false;

    if (cube_point_inside(cube, ray.origin)) {
        return intersection;
    }

    var tmin: f32 = (min.x - ray.origin.x) / ray.direction.x;
    var tmax: f32 = (max.x - ray.origin.x) / ray.direction.x;
    if (tmin > tmax) {
        let t = tmin;
        tmin = tmax;
        tmax = t;
    }

    var tmin_y: f32 = (min.y - ray.origin.y) / ray.direction.y;
    var tmax_y: f32 = (max.y - ray.origin.y) / ray.direction.y;
    if (tmin_y > tmax_y) {
        let t = tmin_y;
        tmin_y = tmax_y;
        tmax_y = t;
    }

    if (tmin > tmax_y || tmax < tmin_y) {
        return intersection;
    }

    tmin = max(tmin, tmin_y);
    tmax = min(tmax, tmax_y);

    var tmin_z: f32 = (min.z - ray.origin.z) / ray.direction.z;
    var tmax_z: f32 = (max.z - ray.origin.z) / ray.direction.z;
    if (tmin_z > tmax_z) {
        let t = tmin_z;
        tmin_z = tmax_z;
        tmax_z = t;
    }

    if (tmin > tmax_z || tmax < tmin_z) {
        return intersection;
    }

    tmin = max(tmin, tmin_z);
    tmax = min(tmax, tmax_z);

    if (tmin <= 0.0) {
        return intersection;
    }

    var normal: vec3f = vec3(0.0, 0.0, 0.0);

    // Flip the sign based on the direction of the incoming ray.
    if (tmin > tmin_y && tmin > tmin_z) {
        normal.x = opposite_sign(ray.direction.x);
    } else if tmin_y > tmin_z {
        normal.y = opposite_sign(ray.direction.y);
    } else {
        normal.z = opposite_sign(ray.direction.z);
    }

    intersection.some = true;
    intersection.t = tmin;
    intersection.normal = normal;
    intersection.ray = ray;

    return intersection;
}

fn cube_point_inside(cube: Cube, point: vec3f) -> bool {
    if (point.x < cube.min.x || point.y < cube.min.y || point.z < cube.min.z) {
        return false;
    } 
    if (point.x > cube.max.x || point.y > cube.max.y || point.z > cube.max.z) {
        return false;
    }
    return true;
}

fn opposite_sign(v: f32) -> f32 {
    if (v == 0.0) {
        return 0.0;
    } else if (v < 0.0) {
        return 1.0;
    } else {
        return -1.0;
    }
}


fn triangle_intersection(p1: vec3f, p2: vec3f, p3: vec3f, ray: Ray) -> Intersection {
    let EPSILON: f32 = 0.0000001;

    var intersection: Intersection;
    intersection.some = false;

    let edge1 = p2 - p1;
    let edge2 = p3 - p1;

    let cross_dir_edge2 = cross(ray.direction, edge2);
    let det = dot(edge1, cross_dir_edge2);

    if (abs(det) < EPSILON) {
        // Parallel or lies in triangle plane
        return intersection;
    }

    let inv_det = 1.0 / det;
    let to_origin = ray.origin - p1;
    let u = inv_det * dot(to_origin, cross_dir_edge2);

    if (u < 0.0 || u > 1.0) {
        return intersection;
    }

    let cross_origin_edge1 = cross(to_origin, edge1);
    let v = inv_det * dot(ray.direction, cross_origin_edge1);

    if (v < 0.0 || u + v > 1.0) {
        return intersection;
    }

    let t = inv_det * dot(edge2, cross_origin_edge1);

    if (t <= EPSILON) {
        return intersection;
    }

    intersection.some = true;
    intersection.t = t;
    intersection.ray = ray;
    intersection.normal = normalize(cross(edge1, edge2));
    
    return intersection;
}
