use rand::Rng;

// A direction, represented as floats. The constraint it keeps is
// that the length is always 1
#[derive(Debug, Copy, Clone)]
pub struct Direction {
	x: f32,
	y: f32,
	z: f32
}

impl Direction {
	pub fn new(x: f32, y: f32, z: f32) -> Direction {
		assert!( (x*x + y*y + z*z - 1f32).abs() < 1e-5f32, "Direction is not normalized");
		Direction {x: x, y: y, z: z }
	}

	// We use rejection method for generation. Generate in cube, and retry
	// if we get the point outside the sphere
	fn generate_random_on_sphere<R>(rng : &mut R) -> Direction
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

// Spherical harmonic coefficients. Represents coefficients only for
// one channel. You can represent any function on sphere as this structure
#[derive(Debug, Clone)]
pub struct SHFuncApproximation {
	coefficients : Vec<f32> 
}

impl SHFuncApproximation {
	pub fn new() -> SHFuncApproximation {
		SHFuncApproximation { coefficients: vec![0f32; 9]}
	}

	pub fn mul_in_place(&mut self, scalar : f32) {
		for i in 0..9 {
			self.coefficients[i] *= scalar;
		}
	}

	pub fn add_in_place(&mut self, other: &SHFuncApproximation) {
		for i in 0..9 {
			self.coefficients[i] += other.coefficients[i];
		}
	}

	pub fn convolution(&self, other : &SHFuncApproximation) -> f32 {
		let mut result = 0f32;
		for i in 0..9 {
			result += self.coefficients[i] * other.coefficients[i];
		}
		result
	}


	// Really fast spherical harmonics order 3 evaluation from
	// this paper: https://www.ppsloan.org/publications/SHJCGT.pdf
	// This is auto-generated code for first 9 SH functions
	// We overwrite the value passed by reference so we don't do allocations
	pub fn from_direction(&mut self, direction: Direction) {
		let sh = &mut self.coefficients;

		let f_x = direction.x;
		let f_y = direction.y;
		let f_z = direction.z;

		let f_z2 = f_z * f_z;
		sh[0] = 0.2820947917738781f32;
		sh[2] = 0.4886025119029199f32 * f_z;
		sh[6] = 0.9461746957575601f32 * f_z2 + -0.3153915652525201f32;
		let f_c0 = f_x;
		let f_s0 = f_y;
		let f_tmp_a = -0.48860251190292f32;
		sh[3] = f_tmp_a * f_c0;
		sh[1] = f_tmp_a * f_s0;
		let f_tmp_b = -1.092548430592079f32 * f_z;
		sh[7] = f_tmp_b * f_c0;
		sh[5] = f_tmp_b * f_s0;
		let f_c1 = f_x*f_c0 - f_y*f_s0;
		let f_s1 = f_x*f_s0 + f_y*f_c0;
		let f_tmp_c = 0.5462742152960395f32;
		sh[8] = f_tmp_c * f_c1;
		sh[4] = f_tmp_c * f_s1;
	}

	pub fn from_function<F, R>(func: F, mut rng: &mut R, count: u32) -> SHFuncApproximation
		where F : Fn(f32, f32, f32) -> f32, R : Rng {

		let mut approximation = SHFuncApproximation::new();
		let mut temporary = SHFuncApproximation::new();

		for _i in 0..count {
			let direction = Direction::generate_random_on_sphere(&mut rng);

			temporary.from_direction(direction);

			let func_value = func(direction.x, direction.y, direction.z);
			temporary.mul_in_place(func_value);

			approximation.add_in_place(&temporary);
		}


		approximation
	}

}

#[cfg(tests)]
mod tests {

}




