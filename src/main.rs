#[macro_use] extern crate auto_ops;
extern crate pixels;
extern crate winit;

use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;


mod hittable;
mod ray;
mod vec3;
mod sphere;

use ray::Ray;
use vec3::Vec3;
use vec3::Color;
use vec3::Point;

const ASPECT_RATIO: f64 = 16. / 9.;
const WIDTH: u32 = 400;
const HEIGHT: u32 = (WIDTH as f64 / ASPECT_RATIO) as u32;

fn ray_color(ray: &Ray) -> Color {
    let t = hit_sphere(&Point::new(0., 0., -1.), 0.5, ray);
    if t > 0. {
        let n = (ray.at(t) - Vec3::new(0., 0., -1.)).unit_vec();
        0.5*Color::new(n.x+1., n.y+1., n.z+1.)
    } else {
        let unit_direction = Vec3::unit_vec(ray.dir);
        let t = 0.5 * (unit_direction.y + 1.0);
        Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
    }
}
            

fn hit_sphere(center: &Point, radius: f64, ray: &Ray) -> f64 {
    let oc: Vec3 = ray.origin - center;
    let a = ray.dir.length_squared();
    let half_b = oc.dot(&ray.dir);
    let c = oc.length_squared() - radius*radius;
    let discriminant = half_b*half_b - a*c;
    if discriminant < 0. {
        -1.
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn draw(frame: &mut [u8]) {
   // Camera
   let viewport_height: f64 = 2.0;
   let viewport_width: f64 = ASPECT_RATIO * viewport_height;
   let focal_length: f64 = 1.0;

   let origin = vec3::Point::zeros();
   let horizontal = Vec3::new(viewport_width, 0., 0.);
   let vertical = Vec3::new(0., viewport_height, 0.);
   let upper_left_corner = origin - horizontal/2. + vertical/2. - Vec3::new(0., 0., focal_length);

   // Render
   for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
       let i = i as u32;
       let x = ((i % WIDTH) as f64) / (WIDTH as f64);
       let y = ((i / WIDTH) as f64) / (HEIGHT as f64);
       let ray = Ray::new(origin, upper_left_corner + horizontal*x - vertical*y);
       let color = ray_color(&ray) * 256.;
       let rgba = [color[0] as u8, color[1] as u8, color[2] as u8, 0xff];
       pixel.copy_from_slice(&rgba);
   }
}

fn main() -> Result<(), Error>{

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

    // Event Loop
    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            draw(pixels.get_frame());
            if pixels
                .render()
                .map_err(|e| println!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize(size.width, size.height);
            }

            // Update internal state and request a redraw
            window.request_redraw();
        }
    });
}
