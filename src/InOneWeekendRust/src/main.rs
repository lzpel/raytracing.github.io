mod ray;

/*
class ray {
  public:
	ray() {}

	ray(const point3& origin, const vec3& direction) : orig(origin), dir(direction) {}

	const point3& origin() const  { return orig; }
	const vec3& direction() const { return dir; }

	point3 at(double t) const {
		return orig + t*dir;
	}

  private:
	point3 orig;
	vec3 dir;
};
*/
fn main() {
	let image_width=1280;
	let image_height =720;
	let aspect_ratio=image_width as f32/image_height as f32;
	// Camera
	let focal_length=1.0;
	let viewport_size = {
		let width=2.0;
		glam::vec2(width, width/aspect_ratio)
	};
	let camera_center=glam::vec3(0.,0.,0.);
	// Calculate the vectors across the horizontal and down the vertical viewport edges.
	let [viewport_u,viewport_v]=[glam::vec3(viewport_size.x, 0., 0.), glam::vec3(0., -viewport_size.y, 0.)];
	// Calculate the horizontal and vertical delta vectors from pixel to pixel.
	let [pixel_delta_u, pixel_delta_v]=[viewport_u/image_width as f32, viewport_v/image_height as f32];
	// Calculate the location of the upper left pixel
	let img= {
		let img=image::ImageBuffer::from_pixel(image_width as u32, image_height as u32, image::Rgb([200, 255, 200]));
		image::DynamicImage::from(img)
	};
	img.save_with_format("out.png", image::ImageFormat::Png).unwrap();
}
