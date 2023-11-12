use std::{
    sync::{Arc, Mutex},
    thread,
    time::Instant,
};

use nalgebra::Unit;
use rand::Rng;

use crate::prelude::*;

use super::background::PatternedBackground;

const DISTANCE_TO_SCREEN: f32 = 1.0;
const SAMPLES_PER_PIXEL: u32 = 9;
const TOTAL_THREADS: u32 = 14;

pub struct RayTracer<B: Buffer<Value = Vector3<f32>>> {
    buffer: B,
    scene: FlatScene,
    camera: Camera,
    background: PatternedBackground,
    /// Number of samples per pixel.
    samples: u32,
    /// Number of threads.
    threads: u32,
}

impl<B: Buffer<Value = Vector3<f32>>> RayTracer<B> {
    pub fn new(buffer: B, scene: Scene, camera: Camera) -> Self {
        Self {
            background: PatternedBackground::new(
                "./wall-e/assets/wall-e-5.png",
                buffer.width(),
                buffer.height(),
                (buffer.width() / 3).max(30).min(60),
            ),
            buffer,
            scene: scene.into(),
            camera,
            samples: SAMPLES_PER_PIXEL,
            threads: TOTAL_THREADS,
        }
    }
}

impl<B: Buffer<Value = Vector3<f32>> + Send + Sync + 'static> RayTracer<B> {
    pub fn run(self) -> B {
        let start_time = Instant::now();
        let total_pixels = self.buffer.width() * self.buffer.height();
        let traced: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));

        let mut handles = vec![];
        let width = self.buffer.width();
        let height = self.buffer.width();
        let buffer_arc = Arc::new(Mutex::new(self.buffer.clone()));
        let thread_count = self.threads;
        let self_arc = Arc::new(self);

        for i in 0..thread_count {
            let self_clone = self_arc.clone();
            let buffer_clone = buffer_arc.clone();
            let traced_clone = traced.clone();

            let handle = thread::spawn(move || {
                let mut pixel = i;
                while pixel < total_pixels {
                    let x = pixel / width;
                    let y = pixel % height;
                    *traced_clone.lock().unwrap() += 1;
                    println!(
                        "⏳ Completed {:.1}%",
                        *traced_clone.lock().unwrap() as f32 / total_pixels as f32 * 100.0
                    );
                    let color = self_clone.color_pixel(x, y);
                    buffer_clone.lock().unwrap().set(x, y, color);
                    pixel += thread_count;
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let duration = Instant::now().duration_since(start_time);
        println!("📷 Render time {:.2}s", duration.as_secs_f32());
        let buffer = buffer_arc.lock().unwrap().clone();

        buffer
    }
}

impl<B: Buffer<Value = Vector3<f32>>> RayTracer<B> {
    fn color_pixel(&self, x: u32, y: u32) -> Vector3<f32> {
        let mut random = rand::thread_rng();
        let mut total_light = Vector3::zeros();
        for _ in 0..self.samples {
            let xx = x as f32 + random.gen_range(0.0..=1.0) - 0.5;
            let yy = y as f32 + random.gen_range(0.0..=1.0) - 0.5;
            let pixel_position = self.compute_pixel_position(xx, yy);

            let ray = Ray::from_points(self.camera.origin(), pixel_position);

            // Cast the primary ray into the scene to intersect with the scene's geometry.
            let Some(intersection) = self.cast_primary_ray(ray) else {
                total_light += self.background.color(x, y);
                continue;
            };

            // Compute the light at the point of intersection by casting secondary, shadow,
            // rays directly towards the lights in the scene.
            let mut sample_light = Vector3::<f32>::zeros();
            for light in self.scene.lights() {
                sample_light += self.light_contribution_at_intersection(light, &intersection);
            }

            total_light += sample_light;
        }

        let average_light = 1.0 / (self.samples as f32) * total_light;
        average_light
    }

    fn cast_primary_ray(&self, ray: Ray) -> Option<Intersection> {
        let mut nearest: Option<Intersection> = None;

        for geometry in self.scene.geometry() {
            if let Some(intersection) = geometry.intersect(&ray) {
                if nearest.is_none() || nearest.as_ref().unwrap().t() > intersection.t() {
                    nearest = Some(intersection);
                }
            }
        }

        nearest
    }

    fn light_contribution_at_intersection(
        &self,
        light: &Light,
        intersection: &Intersection,
    ) -> Vector3<f32> {
        let light_ray = Unit::new_normalize(light.transform().translation() - intersection.point());
        let ray = Ray::from_points(
            intersection.point() + light_ray.into_inner() * 0.1,
            light.transform().translation(),
        );
        let light_t = ray.t(&light.transform().translation());

        let mut in_shadow = false;
        for geometry in self.scene.geometry() {
            if let Some(intersection) = geometry.intersect(&ray) {
                if intersection.t() < light_t {
                    in_shadow = true;
                    break;
                }
            }
        }

        super::shader::phong_illumination(
            self.camera.origin(),
            intersection.point(),
            light.transform().translation(),
            intersection.normal(),
            intersection.material(),
            self.scene.ambient(),
            light.colour(),
            light.attenuation(),
            in_shadow,
        )
    }

    fn compute_pixel_position(&self, x: f32, y: f32) -> Vector3<f32> {
        let d = DISTANCE_TO_SCREEN;

        // Convert the pixel coordinates to NDC coordinates.
        //
        // We add 0.5 to x and y to get the center of the pixel.
        let ndc_x = (x + 0.5) / self.buffer.width() as f32;
        let ndc_y = (y + 0.5) / self.buffer.height() as f32;

        // Convert the NDC coordinates to Screen coordinates.
        let screen_x = (ndc_x - 0.5) * 2.0;
        let screen_y = (ndc_y - 0.5) * 2.0 * -1.0;

        // Correct for the aspect ratio.
        let screen_x = screen_x * self.buffer.aspect();

        // Convert the Screen coordinates to Camera coordinates.
        let tan_half_fov = (self.camera.fov().to_radians() / 2.0).tan();
        let camera_x = screen_x * tan_half_fov * d;
        let camera_y = screen_y * tan_half_fov * d;

        let pixel_camera_pos = Vector3::new(camera_x, camera_y, d);

        // Convert the Camera coordinates to World coordinates.
        let pixel_world_pos = self.camera.camera_to_world_mat() * pixel_camera_pos.push(1.0);

        pixel_world_pos.xyz()
    }
}
