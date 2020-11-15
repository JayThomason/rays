pub mod ppm;
pub mod vec3;

fn print_progress(height: u32, rows_left: u32) {
    let divisor: u8 = if height < 10 { 1 } else { 10 };
    let rows: u32 = height - rows_left;
    let progress: f32 = (rows as f32 / height as f32) * 100.0;
    if progress as u8 % divisor == 0 {
        eprintln!("Progress: {}% written", progress);
    }
}

fn main() {
    let width: u32 = 256;
    let height: u32 = 256;
    let max_color: u8 = 255;
    println!("P3\n{} {}\n{}", width, height, max_color);
    for j in (0..height).rev() {
        print_progress(height, j);
        for i in 0..width {
            ppm::write_color(
                vec3::Color::new((i as f64)/(width-1) as f64, (j as f64)/(height-1) as f64, 0.25,)
            );
        }
    }
}
