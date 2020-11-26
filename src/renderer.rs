use crate::camera::Camera;
use crate::hittable::{Hittable, HittableList};
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};

use rand::distributions::{Distribution, Uniform};

const T_MIN: f64 = 0.001;

pub struct Renderer {
    width: u32,
    height: u32,
    t_min: f64, // 0.001
    max_ray_depth: usize,
    samples_per_pixel: usize,
}

impl Renderer {
    pub fn new(width: u32, height: u32, max_ray_depth: usize, samples_per_pixel: usize) -> Renderer {
        let t_min = T_MIN;
        Renderer{width, height, t_min, max_ray_depth, samples_per_pixel}
    }

    fn ray_color(&self, world: &HittableList, ray: &Ray, depth: usize) -> Color {
        if depth == 0 {
            Color::zeros()
        } else if let Some(hit_record) = world.hit(ray, self.t_min, f64::INFINITY) {
            if let Some((scattered, attenuation)) = hit_record.material.scatter(ray, &hit_record) {
                attenuation * self.ray_color(world, &scattered, depth - 1)
            } else {
                Color::zeros()
            }
        } else {
            let unit_direction = Vec3::unit_vec(&ray.dir);
            let t = 0.5 * (unit_direction.y + 1.0);
            Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
        }
    }

    fn draw_pixels(&self, world: &HittableList, camera: &Camera, pixel_chunk: &mut [(usize, &mut [u8])]) {
        let between = Uniform::new(0., 1.);
        let mut rng = rand::thread_rng();
        for (i, pixel) in pixel_chunk {
            let mut color = Color::zeros();
            for _ in 0..self.samples_per_pixel {
                let i = *i as u32;
                let x = ((i % self.width) as f64 + between.sample(&mut rng)) / (self.width as f64);
                let y = ((i / self.width) as f64) / (self.height as f64);
                let ray = camera.get_ray(x, y);
                color += self.ray_color(world, &ray, self.max_ray_depth);
            }
            // Divide the color by the number of samples and gamma-correct for gamma=2.0.
            let scale = 1. / (self.samples_per_pixel as f64);
            color.x = (scale * color.x).sqrt();
            color.y = (scale * color.y).sqrt();
            color.z = (scale * color.z).sqrt();
            let color = color.clamped(0., 0.999) * 256.;
            let rgba = [color[0] as u8, color[1] as u8, color[2] as u8, 0xff];
            pixel.copy_from_slice(&rgba);
        }
    }

    pub fn draw(&self, world: &HittableList, camera: &Camera, frame: &mut [u8]) {
        let mut pixel_list: Vec<(usize, &mut [u8])> = frame.chunks_exact_mut(4).enumerate().collect();
        let num_threads: usize = 16;
        // TODO: does this need to be adjusted, e.g. what if it's a 4x2 image but num_threads is 3?
        let chunk_size = pixel_list.len() / num_threads;
        rayon::scope(|s| {
            for chunk in pixel_list.chunks_exact_mut(chunk_size) {
                s.spawn(move |_| {
                    self.draw_pixels(world, camera, chunk);
                });
            }
        });
    }
}
