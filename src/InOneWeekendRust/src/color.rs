use std::io::{self, Write};
use glam::Vec3;

pub fn write_color<W: Write>(out: &mut W, pixel_color: Vec3) -> io::Result<()> {
    let r = pixel_color.x;
    let g = pixel_color.y;
    let b = pixel_color.z;

    let r_byte = (255.999 * r) as i32;
    let g_byte = (255.999 * g) as i32;
    let b_byte = (255.999 * b) as i32;

    writeln!(out, "{} {} {}", r_byte, g_byte, b_byte)
}