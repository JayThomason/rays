#[macro_use] extern crate auto_ops;
//extern crate raw_window_handle;
extern crate pixels;
extern crate winit;
extern crate rand;
extern crate rayon;


use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};
use winit_input_helper::WinitInputHelper;

mod camera;
mod fps;
mod hittable;
mod material;
mod ray;
mod renderer;
mod vec3;
mod sphere;

use camera::Camera;
use hittable::Hittable;
use hittable::HittableList;
use material::{Dielectric, Lambertian, Metal};
use renderer::Renderer;
use sphere::Sphere;
use vec3::Vec3;
use vec3::Color;

const ASPECT_RATIO: f64 = 16. / 9.;
const VERTICAL_FOV_DEG: f64 = 40.;
const WIDTH: u32 = 800;
const HEIGHT: u32 = (WIDTH as f64 / ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: usize = 100;
const MAX_RAY_DEPTH: usize = 50;

fn construct_world() -> HittableList {
    let material_ground = Box::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Box::new(Dielectric{ir: -0.4});
    let material_left = Box::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Box::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    let mut objects: Vec<Box<dyn Hittable + Send + Sync>> = Vec::new();
    objects.push(Box::new(Sphere::new( 0.0, -100.5, -1., 100.0, material_ground)));
    objects.push(Box::new(Sphere::new( 0.0,    0.0, -1.,   0.5, material_center)));
    objects.push(Box::new(Sphere::new(-1.0,    0.0, -1.0,   0.5, material_left)));
    objects.push(Box::new(Sphere::new( 1.0,    0.0, -1.0,   0.5, material_right)));

    HittableList::new(objects)
}

fn initialize_window(width: u32, height: u32) -> (EventLoop<()>, WinitInputHelper, Window) {
    let event_loop = EventLoop::new();
    let input = WinitInputHelper::new();
    let size = LogicalSize::new(width as f64, height as f64);
    let window: Window = WindowBuilder::new()
        .with_title("Rays")
        .with_inner_size(size)
        .with_min_inner_size(size)
        .build(&event_loop)
        .unwrap();
    (event_loop, input, window)
}

fn handle_navigation(input: &WinitInputHelper, camera: &mut Camera, window: &Window) {
    let mut camera_movement = Vec3::zeros();
    if input.key_pressed(VirtualKeyCode::W) {
        camera_movement.z = -0.1;
    }
    if input.key_pressed(VirtualKeyCode::S) {
        camera_movement.z += 0.1;
    }
    if input.key_pressed(VirtualKeyCode::A) {
        camera_movement.x -= 0.1;
    }
    if input.key_pressed(VirtualKeyCode::D) {
        camera_movement.x += 0.1;
    }
    if camera_movement != Vec3::zeros() {
       camera.shift(&camera_movement);
       window.request_redraw();
    }
}

fn main() -> Result<(), pixels::Error> {
    // Create world, camera, and renderer
    let world = construct_world();
    let mut camera = Camera::new(VERTICAL_FOV_DEG, ASPECT_RATIO);
    let renderer = Renderer::new(WIDTH, HEIGHT, MAX_RAY_DEPTH, SAMPLES_PER_PIXEL);
    let mut timer = fps::timer();
    
    // Set up window and event loop
    let (event_loop, mut input, window) = initialize_window(WIDTH, HEIGHT);

    // Initialize frame buffer
    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let mut pixels = Pixels::new(WIDTH, HEIGHT, surface_texture)?;

    // Event Loop
    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            timer.start();
            renderer.draw(&world, &camera, pixels.get_frame());
            timer.stop();
            timer.print_stats();
            if pixels.render().map_err(|e| println!("pixels.render() failed: {}", e)).is_err() {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Navigation events
            handle_navigation(&input, &mut camera, &window);

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
