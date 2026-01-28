use glam::Vec3;


pub fn to_rgb(pixel_color: Vec3) -> image::Rgb<u8> {
	let r = pixel_color.x;
	let g = pixel_color.y;
	let b = pixel_color.z;

	let r_byte = (255.999 * r) as u8;
	let g_byte = (255.999 * g) as u8;
	let b_byte = (255.999 * b) as u8;

	image::Rgb([r_byte, g_byte, b_byte])
}
