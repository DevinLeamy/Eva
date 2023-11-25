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

struct CubeModels {
    length: u32,
    cubes: array<CubeModel>,
};

struct CubeModel {
    cube: Cube,
    transform: Transform,
    material_id: u32 
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
    material_id: u32,
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
    material_id: u32,
    uv: vec2f,
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
    ambient: vec3f,
};

struct MeshPositions {
    length: u32,
    positions: array<vec3f>
};

struct MeshTriangles {
    length: u32,
    // u32 -> MeshVertex
    triangles: array<vec3u> 
};

struct MeshHeaders {
    length: u32,
    headers: array<MeshModelHeader>
};

struct MeshModelHeader {
    material_id: u32,
    transform: Transform,
    bounding_box: Cube,

    vertex_offset: u32,
    triangle_offset: u32,
    triangle_count: u32,

    position_offset: u32,
    normal_offset: u32,

    // ShaderBool (0=false, 1=true)
    has_normals: u32
};

struct MeshVertices {
    length: u32,
    vertices: array<MeshVertex>
};

struct MeshVertex {
    position: u32,
    normal: u32,
};

struct MeshNormals {
    length: u32,
    normals: array<vec3f>
};

struct Materials {
    length: u32,
    materials: array<Material>
}

struct Material {
    albedo: vec3f,
    roughness: f32,
    metallic: f32
}

@group(0) @binding(0) var colour_buffer: texture_storage_2d<rgba16float, write>;
@group(0) @binding(1) var<uniform> camera: Camera;
@group(0) @binding(2) var<storage, read> spheres: SphereModels; 
@group(0) @binding(3) var<storage, read> lights: PointLights;
@group(0) @binding(4) var<uniform> config: GlobalConfig;
@group(0) @binding(5) var<storage, read> cubes: CubeModels; 
@group(0) @binding(6) var<storage, read> materials: Materials; 

@group(1) @binding(0) var<storage, read> mesh_headers: MeshHeaders;
@group(1) @binding(1) var<storage, read> mesh_triangles: MeshTriangles;
@group(1) @binding(2) var<storage, read> mesh_vertices: MeshVertices;
@group(1) @binding(3) var<storage, read> mesh_positions: MeshPositions;
@group(1) @binding(4) var<storage, read> mesh_normals: MeshNormals;

@group(2) @binding(0) var textures: binding_array<texture_2d<f32>, 12>;
@group(2) @binding(1) var texture_samplers: binding_array<sampler, 12>;

@group(3) @binding(0) var skybox: texture_cube<f32>;
@group(3) @binding(1) var skybox_sampler: sampler;


@compute @workgroup_size(1, 1, 1)
fn compute_main(@builtin(global_invocation_id) GlobalInvocationID: vec3<u32>) {
    let screen_coord = vec2<i32>(i32(GlobalInvocationID.x), i32(GlobalInvocationID.y));
    let samples_per_row = 3;

    let segment = (1.0 / f32(samples_per_row + 2));  
    var colour: vec3f = vec3f(0.0, 0.0, 0.0);

    for (var i: i32 = 0; i < samples_per_row; i = i + 1) {
        for (var j: i32 = 0; j < samples_per_row; j = j + 1) {
            var x = f32(screen_coord.x) + segment + segment * f32(i);
            var y = f32(screen_coord.y) + segment + segment * f32(j);

            let pixel_position = compute_pixel_position(x, y);

            let ray = ray_from_points(camera.position, pixel_position);
            let ray_colour = compute_ray_colour(ray);
            colour = colour + ray_colour;
        }
    }

    textureStore(colour_buffer, screen_coord, vec4<f32>(colour / f32(samples_per_row * samples_per_row), 1.0));
}

fn compute_pixel_position(x: f32, y: f32) -> vec3f {
    let screen_size = vec2i(textureDimensions(colour_buffer));

    // Distance to the screen.
    let d = 1.0;

    let width = f32(screen_size.x);
    let height = f32(screen_size.y);
    let aspect = width / height;

    // Convert the pixel coordinates to NDC coordinates.
    let ndc_x = x / width;
    let ndc_y = y / height;

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
    var new_intersection: Intersection = intersection;
    new_intersection.ray = ray_transform(intersection.ray, transform);

    let new_point = transform.m * vec4f(ray_point(intersection.ray, intersection.t), 1.0);
    new_intersection.t = ray_t(new_intersection.ray, new_point.xyz);
    new_intersection.normal = normalize(transform.m_normal_inverse * intersection.normal);

    return new_intersection;
}

fn compute_ray_colour(_ray: Ray) -> vec3f {
    var ray: Ray = _ray;
    // let max_reflections = 3;
    let max_reflections = 1;

    var total_light: vec3f = vec3f(0.0, 0.0, 0.0);

    for (var i: i32 = 0; i < max_reflections; i = i + 1) {
        let intersection = compute_ray_intersection(ray);
        if (!intersection.some) {
            total_light = total_light + compute_skybox_colour(ray.direction);
            break;
        }

        total_light = total_light + compute_light_at_intersection(intersection);
        ray = compute_reflected_ray(ray, intersection);
    }

    return total_light;
}

fn compute_skybox_colour(coords: vec3f) -> vec3f {
    let colour = textureSampleLevel(skybox, skybox_sampler, coords, 0.0).rgb; 
    return colour;
}

fn compute_reflected_ray(ray: Ray, intersection: Intersection) -> Ray {
    var reflected_ray: Ray;
    reflected_ray.direction = normalize(ray.direction - intersection.normal * 2.0 * dot(ray.direction, intersection.normal));
    // Offset to avoid floating point errors.
    reflected_ray.origin = ray_point(ray, intersection.t) + reflected_ray.direction;

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
        transformed_intersection.material_id = sphere_model.material_id;
    
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
        transformed_intersection.material_id = cube_model.material_id;
    
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
            let v1 = mesh_vertices.vertices[mesh.vertex_offset + triangle.x];
            let v2 = mesh_vertices.vertices[mesh.vertex_offset + triangle.y];
            let v3 = mesh_vertices.vertices[mesh.vertex_offset + triangle.z];

            let new_intersection = mesh_triangle_intersection(mesh, v1, v2, v3, transformed_ray);
            var transformed_intersection: Intersection = intersection_transform(new_intersection, mesh.transform);

            // Set the material of the object that was intersected with.
            transformed_intersection.material_id = mesh.material_id;
        
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
    let albedo = intersection_material_colour(intersection);
    let ambient = config.ambient * albedo;
    if (!intersection.some) {
        return ambient;
    }

    var total_light: vec3f = vec3f(0.0);
    for (var i: i32 = 0; i < i32(lights.length); i = i + 1) {
        let light = lights.lights[i];
        total_light = total_light + compute_light_contribution_at_intersection(intersection, light);
    }


    var colour: vec3f = ambient + total_light;
    colour = colour / (colour + vec3(1.0));
    colour = pow(colour, vec3(1.0/2.2));

    return colour;
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
    return pbr_illumination(intersection, light, ray_to_light_intersection.some);
}

fn pbr_illumination(intersection: Intersection, light: PointLight, in_shadow: bool) -> vec3f {
    if (in_shadow) {
        return vec3f(0.0);
    }

    let albedo = intersection_material_colour(intersection);
    let material = material_by_id(intersection.material_id);
    let metallic = material.metallic;
    let roughness = material.roughness;

    let intersection_point = ray_point(intersection.ray, intersection.t);
    let to_view = normalize(camera.position - intersection_point);
    let f0 = lerp_vec3(vec3f(0.04), albedo, metallic);

    let to_light = normalize(light.position - intersection_point);
    let half_vector = normalize(to_view + to_light);

    let distance = length(light.position - camera.position);
    let attenuation = 1.0; 
    let radiance = light.colour * attenuation;

    // Cook-Torrance BRDF
    let N = intersection.normal;

    let NDF = distribution_ggx(N, half_vector, roughness);
    let G = geometry_smith(N, to_view, to_light, roughness);
    let F = fresnel_schlick(max(0.0, dot(to_view, half_vector)), f0);

    let ks = F;
    let kd = (vec3f(1.0, 1.0, 1.0) - ks) * (1.0 - metallic); 

    let numerator = NDF * G * F;
    let denominator = 4.0 * max(0.0, dot(to_view, N)) + 0.0001;
    let specular = numerator / denominator;

    let PI = radians(180.0);
    return (kd * albedo / PI + specular) * radiance * max(0.0, dot(N, to_light));
}

fn distribution_ggx(n: vec3f, h: vec3f, roughness: f32) -> f32 {
    let PI = radians(180.0);
    let alpha = roughness * roughness;
    let alpha_2 = alpha * alpha;
    let n_dot_h = max(0.0, dot(n, h));
    let n_dot_h_2 = n_dot_h * n_dot_h;

    let numerator = alpha_2;
    let denominator = (n_dot_h_2 * (alpha_2 - 1.0) + 1.0) * (n_dot_h_2 * (alpha_2 - 1.0) + 1.0) * PI;
    return numerator / denominator;
}

fn geometry_smith(n: vec3f, v: vec3f, l: vec3f, roughness: f32) -> f32 {
    let n_dot_v = max(0.0, dot(n, v));
    let n_dot_l = max(0.0, dot(n, l));

    return geometry_schlick_ggx(n_dot_v, roughness) * geometry_schlick_ggx(n_dot_l, roughness);
}

fn geometry_schlick_ggx(n_dot_v: f32, roughness: f32) -> f32 {
    let r = (roughness + 1.0);
    let k = (r * r) / 8.0;

    let numerator = n_dot_v;
    let denominator = n_dot_v * (1.0 - k) + k;

    return numerator / denominator;
}

fn fresnel_schlick(cos_theta: f32, f0: vec3f) -> vec3f {
    return f0 + (1.0 - f0) * pow(clamp(1.0 - cos_theta, 0.0, 1.0), 5.0);
}

fn intersection_material_colour(intersection: Intersection) -> vec3f {
    let material = material_by_id(intersection.material_id);
    var colour: vec3f = material.albedo;

    if (material.albedo.x == -1.0) {
        let texture_id = u32(material.albedo.y);
        let texture_coords = vec2f(intersection.uv.x, 1.0 - intersection.uv.y);
        colour = sample_texture(texture_id, texture_coords);
    }

    return colour;
}

fn sample_texture(texture_id: u32, uv: vec2f) -> vec3f {
    return textureSampleLevel(textures[texture_id], texture_samplers[texture_id], uv, 0.0).rgb;
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

    var point: vec3f = ray_point(ray, t);
    let surface_normal = normalize(point);
    point = surface_normal * sphere.radius;

    intersection.some = true;
    intersection.ray = ray;
    intersection.t = ray_t(ray, point);
    intersection.normal = surface_normal;
    intersection.uv = sphere_uv(sphere, point);

    return intersection;
}

fn sphere_uv(sphere: Sphere, point: vec3f) -> vec2f {
    let PI = radians(180.0);

    let theta = acos(-point.y);
    let phi = atan2(-point.z, point.x) + PI;

    let u = phi / (2.0 * PI);
    let v = theta / PI;
    return vec2f(u, v);
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
    let p = ray_point(ray, tmin);
    let p_normalized = p - min;
    let size = max - min;
    var uv: vec2f = vec2f(0.0, 0.0);

    // Flip the sign based on the direction of the incoming ray.
    if (tmin > tmin_y && tmin > tmin_z) {
        normal.x = opposite_sign(ray.direction.x);
        uv = p_normalized.yz / size.yz;
    } else if tmin_y > tmin_z {
        normal.y = opposite_sign(ray.direction.y);
        uv = p_normalized.xz / size.xz;
    } else {
        normal.z = opposite_sign(ray.direction.z);
        uv = p_normalized.xy / size.xy;
    }

    intersection.some = true;
    intersection.t = tmin;
    intersection.normal = normal;
    intersection.ray = ray;
    intersection.uv = uv;

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


fn mesh_triangle_intersection(mesh: MeshModelHeader, v1: MeshVertex, v2: MeshVertex, v3: MeshVertex, ray: Ray) -> Intersection {
    let p1 = mesh_positions.positions[mesh.position_offset + v1.position];
    let p2 = mesh_positions.positions[mesh.position_offset + v2.position];
    let p3 = mesh_positions.positions[mesh.position_offset + v3.position];

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

    let normal = normalize(cross(edge1, edge2));
    intersection.some = true;
    intersection.t = t;
    intersection.ray = ray;
    intersection.normal = normal;
    intersection.uv = vec2f(0.0, 0.0);

    if (shader_bool(mesh.has_normals)) {
        // Perform Phong Shading.
        let n1 = mesh_normals.normals[mesh.normal_offset + v1.normal];
        let n2 = mesh_normals.normals[mesh.normal_offset + v2.normal];
        let n3 = mesh_normals.normals[mesh.normal_offset + v3.normal];
        let p = ray_point(ray, t);

        // Compute the barycentric coordinates using the triangle-area method.
        let area = dot(normal, cross(p2 - p1, p3 - p1));
        let area_23 = dot(normal, cross(p2 - p, p3 - p));
        let area_13 = dot(normal, cross(p3 - p, p1 - p));

        let bx = area_23 / area;
        let by = area_13 / area;
        let bz = 1.0 - bx - by;

        // Compute the interpolated normal.
        intersection.normal = normalize(n1 * bx + n2 * by + n3 * bz);
    } 
    
    return intersection;
}

fn random01(seed: vec2<f32>) -> f32 {
    let x = dot(seed, vec2<f32>(12.9898, 78.233)) * 0.01267123 + 54.54321;
    return fract(sin(x) * 43758.5453);
}

fn shader_bool(boolean: u32) -> bool {
    return boolean == u32(1);
}

fn material_by_id(material_id: u32) -> Material {
    return materials.materials[material_id];
}

fn lerp_vec3(a: vec3f, b: vec3f, l: f32) -> vec3f {
    return vec3f(
        lerp(a.x, b.x, l),
        lerp(a.y, b.y, l),
        lerp(a.z, b.z, l),
    );
}

fn lerp(a: f32, b: f32, l: f32) -> f32 {
    return a + (b - a) * l;
}
