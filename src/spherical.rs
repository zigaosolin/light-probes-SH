use rand::Rng;

// A direction, represented as floats. Invariant is length == 1
#[derive(Debug, Copy, Clone)]
pub struct Direction {
	pub x: f32,
	pub y: f32,
	pub z: f32
}

impl Direction {
	pub fn new(x: f32, y: f32, z: f32) -> Direction {
		assert!( (x*x + y*y + z*z - 1f32).abs() < 1e-5f32, "Direction is not normalized");
		Direction {x: x, y: y, z: z }
	}

	// We use rejection method for generation. Generate in cube, and retry
	// if we get the point outside the sphere
	pub fn generate_random_on_sphere<R>(rng : &mut R) -> Direction
		where R : Rng {

		loop {
			let x = rng.gen::<f32>() * 2f32 - 1f32;
			let y = rng.gen::<f32>() * 2f32 - 1f32;
			let z = rng.gen::<f32>() * 2f32 - 1f32;

			let r2 = x*x + y*y + z*z;
			if r2 > 1f32 {
				continue;
			}

			let r = r2.sqrt();
			return Direction {x: x/r, y: y/r, z: z/r};
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	#[should_panic]
	fn direction_initialize_non_normalized() {
		let _direction = Direction::new(2f32, 0f32, 1f32);
	}

	#[test]
	fn direction_sampling() {
		let mut rng = rand::thread_rng();

		let mut sum_x = 0f32;
		let mut sum_y = 0f32;
		let mut sum_z = 0f32;

		let count = 20000;

		for _i in 0..count {
			let d = Direction::generate_random_on_sphere(&mut rng);
			assert!( (d.x*d.x + d.y*d.y + d.z*d.z - 1f32).abs() < 1e-5f32, "Direction is not normalized");

			sum_x += d.x;
			sum_y += d.y;
			sum_z += d.z;
		}

		sum_x /= count as f32;
		sum_y /= count as f32;
		sum_z /= count as f32;

		assert!(sum_x.abs() < 0.05, "Distribution not equal in x, {0}", sum_x);
		assert!(sum_y.abs() < 0.05, "Distribution not equal in y, {0}", sum_y);
		assert!(sum_z.abs() < 0.05, "Distribution not equal in z, {0}", sum_z);
	}
}