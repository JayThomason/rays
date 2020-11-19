#[macro_use] extern crate auto_ops;
extern crate pixels;
extern crate winit;
extern crate rand;
extern crate rayon;

use pixels::{Error, Pixels, SurfaceTexture};
use rand::distributions::{Distribution, Uniform};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

mod fps;
mod hittable;
mod material;
mod ray;
mod vec3;
mod sphere;

use hittable::Hittable;
use hittable::HittableList;
use material::{Lambertian, Metal};
use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;
use vec3::Color;
use vec3::Point;

const ASPECT_RATIO: f64 = 16. / 9.;
const WIDTH: u32 = 1200;
const HEIGHT: u32 = (WIDTH as f64 / ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: u32 = 100;
const MAX_RAY_DEPTH: usize = 50;

struct Camera {
    origin: Point,
    horizontal: Vec3,
    vertical: Vec3,
    upper_left_corner: Point,
}

impl Camera {
    fn new() -> Camera {
        let height = 2.;
        let width = ASPECT_RATIO * height;
        let focal_length = 1.;
        let origin = Point::zeros();
        let horizontal = Vec3::new(width, 0., 0.);
        let vertical = Vec3::new(0., height, 0.);
        let upper_left_corner = origin - horizontal/2. + vertical/2. - Vec3::new(0., 0., focal_length);
        Camera{origin, horizontal, vertical, upper_left_corner}
    }
    
    fn get_ray(&self, x: f64, y: f64) -> Ray {
        Ray::new(self.origin, self.upper_left_corner + self.horizontal*x - self.vertical*y)
    }
}

fn ray_color(world: &HittableList, ray: &Ray, depth: usize) -> Color {
    if depth == 0 {
        Color::zeros()
    } else if let Some(hit_record) = world.hit(ray, 0.001, f64::INFINITY) {
        if let Some((scattered, attenuation)) = hit_record.material.scatter(ray, &hit_record) {
            attenuation * ray_color(world, &scattered, depth - 1)
        } else {
            Color::zeros()
        }
    } else {
        let unit_direction = Vec3::unit_vec(&ray.dir);
        let t = 0.5 * (unit_direction.y + 1.0);
        Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
    }
}

fn draw_pixels(world: &HittableList, camera: &Camera, pixel_chunk: &mut [(usize, &mut [u8])]) {
    let between = Uniform::new(0., 1.);
    let mut rng = rand::thread_rng();
    for (i, pixel) in pixel_chunk {
        let mut color = Color::zeros();
        for _ in 0..SAMPLES_PER_PIXEL {
            let i = *i as u32;
            let x = ((i % WIDTH) as f64 + between.sample(&mut rng)) / (WIDTH as f64);
            let y = ((i / WIDTH) as f64) / (HEIGHT as f64);
            let ray = camera.get_ray(x, y);
            color += ray_color(world, &ray, MAX_RAY_DEPTH);
        }
        // Divide the color by the number of samples and gamma-correct for gamma=2.0.
        let scale = 1. / (SAMPLES_PER_PIXEL as f64);
        color.x = (scale * color.x).sqrt();
        color.y = (scale * color.y).sqrt();
        color.z = (scale * color.z).sqrt();
        let color = color.clamped(0., 0.999) * 256.;
        let rgba = [color[0] as u8, color[1] as u8, color[2] as u8, 0xff];
        pixel.copy_from_slice(&rgba);
    }
}

fn draw(world: &HittableList, camera: &Camera, frame: &mut [u8]) {
    let mut pixel_list: Vec<(usize, &mut [u8])> = frame.chunks_exact_mut(4).enumerate().collect();
    let num_threads: usize = 16;
    // TODO: does this need to be adjusted, e.g. what if it's a 4x2 image but num_threads is 3?
    let chunk_size = pixel_list.len() / num_threads;
    rayon::scope(|s| {
        for chunk in pixel_list.chunks_exact_mut(chunk_size) {
            s.spawn(move |_| {
                draw_pixels(world, camera, chunk);
            });
        }
    });
}

fn main() -> Result<(), Error>{
    // World
    let material_ground = Box::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Box::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let material_left = Box::new(Metal::new(Color::new(0.8, 0.8, 0.8)));
    let material_right = Box::new(Metal::new(Color::new(0.8, 0.6, 0.2)));

    let mut objects: Vec<Box<dyn Hittable + Send + Sync>> = Vec::new();
    objects.push(Box::new(Sphere::new( 0.0, -100.5, -1.0, 100.0, material_ground)));
    objects.push(Box::new(Sphere::new( 0.0,    0.0, -1.0,   0.5, material_center)));
    objects.push(Box::new(Sphere::new(-1.0,    0.0, -1.0,   0.5, material_left)));
    objects.push(Box::new(Sphere::new( 1.0,    0.0, -1.0,   0.5, material_right)));

    let world = HittableList::new(objects);

    // Window 
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Rays")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    // Pixels
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    // Camera 
    let mut camera = Camera::new();

    // Timer
    let mut timer = fps::timer();

    // Event Loop
    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            timer.start();
            draw(&world, &camera, pixels.get_frame());
            timer.stop();
            timer.print_stats();
            if pixels.render().map_err(|e| println!("pixels.render() failed: {}", e)).is_err() {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Navigation
            if input.key_pressed(VirtualKeyCode::W) {
                camera.origin.z += -0.1;
                window.request_redraw();
            }
            if input.key_pressed(VirtualKeyCode::S) {
                camera.origin.z += 0.1;
                window.request_redraw();
            }
            if input.key_pressed(VirtualKeyCode::A) {
                camera.origin.x -= 0.1;
                window.request_redraw();
            }
            if input.key_pressed(VirtualKeyCode::D) {
                camera.origin.x += 0.1;
                window.request_redraw();
            }

            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize(size.width, size.height);
            }
        }
    });
}
