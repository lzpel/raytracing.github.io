use glam;
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
pub struct Ray {
	pub orig: glam::Vec3,
	pub dir: glam::Vec3,
}
impl Ray {
	pub fn origin(&self) -> glam::Vec3 {
		self.orig
	}
	pub fn direction(&self) -> glam::Vec3 {
		self.dir
	}
	pub fn at(&self, t: f32) -> glam::Vec3 {
		self.orig + t * self.dir
	}
}
#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn at() {
		let r = Ray {
			orig: glam::Vec3::new(0.0, 1.0, 2.0),
			dir: glam::Vec3::new(1.0, 0.0, 0.0),
		};
		r.origin();
		r.direction();
		assert_eq!(r.at(0.5), glam::Vec3::new(0.5, 1.0, 2.0))
	}
}
