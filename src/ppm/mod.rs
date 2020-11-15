use crate::vec3::Color;

pub fn write_color(color: Color) {
    let ir: u8 = (255.999 * color[0]) as u8;
    let ig: u8 = (255.999 * color[1]) as u8;
    let ib: u8 = (255.999 * color[2]) as u8;
    println!("{} {} {}", ir, ig, ib);
}
