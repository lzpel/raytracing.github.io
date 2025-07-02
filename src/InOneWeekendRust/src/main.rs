mod ray;

/*
#include "rtweekend.h"

#include "camera.h"
#include "hittable.h"
#include "hittable_list.h"
#include "material.h"
#include "sphere.h"


int main() {
    hittable_list world;

    auto ground_material = make_shared<lambertian>(color(0.5, 0.5, 0.5));
    world.add(make_shared<sphere>(point3(0,-1000,0), 1000, ground_material));

    for (int a = -11; a < 11; a++) {
        for (int b = -11; b < 11; b++) {
            auto choose_mat = random_double();
            point3 center(a + 0.9*random_double(), 0.2, b + 0.9*random_double());

            if ((center - point3(4, 0.2, 0)).length() > 0.9) {
                shared_ptr<material> sphere_material;

                if (choose_mat < 0.8) {
                    // diffuse
                    auto albedo = color::random() * color::random();
                    sphere_material = make_shared<lambertian>(albedo);
                    world.add(make_shared<sphere>(center, 0.2, sphere_material));
                } else if (choose_mat < 0.95) {
                    // metal
                    auto albedo = color::random(0.5, 1);
                    auto fuzz = random_double(0, 0.5);
                    sphere_material = make_shared<metal>(albedo, fuzz);
                    world.add(make_shared<sphere>(center, 0.2, sphere_material));
                } else {
                    // glass
                    sphere_material = make_shared<dielectric>(1.5);
                    world.add(make_shared<sphere>(center, 0.2, sphere_material));
                }
            }
        }
    }

    auto material1 = make_shared<dielectric>(1.5);
    world.add(make_shared<sphere>(point3(0, 1, 0), 1.0, material1));

    auto material2 = make_shared<lambertian>(color(0.4, 0.2, 0.1));
    world.add(make_shared<sphere>(point3(-4, 1, 0), 1.0, material2));

    auto material3 = make_shared<metal>(color(0.7, 0.6, 0.5), 0.0);
    world.add(make_shared<sphere>(point3(4, 1, 0), 1.0, material3));

    camera cam;

    cam.aspect_ratio      = 16.0 / 9.0;
    cam.image_width       = 1200;
    cam.samples_per_pixel = 10;
    cam.max_depth         = 20;

    cam.vfov     = 20;
    cam.lookfrom = point3(13,2,3);
    cam.lookat   = point3(0,0,0);
    cam.vup      = vec3(0,1,0);

    cam.defocus_angle = 0.6;
    cam.focus_dist    = 10.0;

    cam.render(world);
}

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
	let viewport_upper_left = camera_center - glam::vec3(0., 0., focal_length) - viewport_u/2. - viewport_v/2.;
	let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

	//Render

	println!("P3\n{image_width} {image_height}\n255\n");

	for j in 0..image_height{
		println!("Scanlines_remaining: {}", image_height - j);
		for i in 0..image_width{
			let pixel_center = pixel00_loc+(i as f32*pixel_delta_u)+(j as f32*pixel_delta_v);
			let ray_direction=pixel_center-camera_center;
			let r=ray::Ray{
				orig: camera_center,
				dir: ray_direction
			};
			let pixel_color=ray_color(r);
		}
	}
}

fn ray_color(r: ray::Ray)->glam::Vec3{
	Default::default()
}