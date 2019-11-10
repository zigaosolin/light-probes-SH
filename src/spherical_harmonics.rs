use rand::Rng;
use std::f32::consts::PI;

// A direction, represented as floats. Invariant is length == 1
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

// Spherical harmonic coefficients. You can represent any function
// on sphere using these structure (to certain degree). Smooth
// functions of angle are represented better
#[derive(Debug, Clone)]
pub struct SHFuncApproximation {
	coefficients : Vec<f32> 
}

impl SHFuncApproximation {
	pub fn new() -> SHFuncApproximation {
		SHFuncApproximation { coefficients: vec![0f32; 9]}
	}

	// Multiplies with self, and stores value in self (to avoid allocations)
	pub fn mul_in_place(&mut self, scalar : f32) {
		for i in 0..9 {
			self.coefficients[i] *= scalar;
		}
	}

	// Adds other coefficients to self
	pub fn add_in_place(&mut self, other: &SHFuncApproximation) {
		for i in 0..9 {
			self.coefficients[i] += other.coefficients[i];
		}
	}

	// Evaluates the SH in certain direction. We use convolution
	// to evalute integral with delta function, as it is faster to do like this
	pub fn eval(&self, direction: Direction, workspace: &mut SHFuncApproximation) -> f32 {
		workspace.from_direction(direction);
		self.convolution(workspace) / (4f32 * PI)
	}

	// Computes the integral of multiply of two SH representations,
	// matches the real-case integral as closely as it can
	pub fn convolution(&self, other : &SHFuncApproximation) -> f32 {
		let mut result = 0f32;
		for i in 0..9 {
			result += self.coefficients[i] * other.coefficients[i];
		}

		// In SH space, normalization is 1, in realspace, normalization
		// is 4 PI (this is the result of integrating over sphere). We match
		// realspace here
		16f32 * PI * PI * result
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

		// Normalize by the amount of samples
		approximation.mul_in_place(1f32 / (count as f32));
		approximation
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

	#[test]
	fn uniform_distribution_sh() {
		let mut rng = rand::thread_rng();
		let func = |_x,_y,_z| 1f32;

		let sh = SHFuncApproximation::from_function(func, &mut rng, 10000);
		
		println!("{:?}", sh);
		for i in 1..9 {
			assert!(sh.coefficients[i].abs() < 0.01, "All but first coefficient should converge to zero, got {0}", sh.coefficients[i]);
		}
	}

	fn integrate_real_space<F, R>(func: F, mut rand: &mut R, count: u32) -> f32 
		where F: Fn(f32, f32, f32) -> f32, R: Rng {

		let mut sum = 0f32;
		for _i in 0..count {
			let direction = Direction::generate_random_on_sphere(&mut rand);

			sum += func(direction.x, direction.y, direction.z);
		}

		4f32 * PI * sum / (count as f32)
	}

	#[test]
	fn convolution_sh_constant() {
		let mut rng = rand::thread_rng();
		let func = |_x,_y,_z| 1f32;
		let sh = SHFuncApproximation::from_function(func, &mut rng, 10000);
		
		// Convoluting constant function with constant is the same
		
		let result = sh.convolution(&sh);

		let normalized = integrate_real_space(|x,y,z| { let value = func(x,y,z); value*value }, &mut rng, 10000);
		assert!( (normalized - 4f32 * PI).abs() < 0.0001, "We expect the real space integration to yield 4PI for constant");

		// Expected error is small as we can do a really perfect approximation of constant function
		let expected = normalized;
		assert!( (result - expected).abs() < 0.3, "Result is {0}, expected {1}", result, expected);
	}

	#[test]
	fn convolution_sh_nontrivial_odd() {
		let mut rng = rand::thread_rng();
		let func = |x:f32,_y:f32,_z:f32|  x;

		let sh = SHFuncApproximation::from_function(func, &mut rng, 10000);
		
		// Convoluting constant function with constant is the same
		let result = sh.convolution(&sh);

		let normalized = integrate_real_space(|x,y,z| { let value = func(x,y,z); value*value }, &mut rng, 10000);

		let expected = normalized;
		assert!( (result - expected).abs() < 0.3, "Result is {0}, expected {1}", result, expected);
	}

	#[test]
	fn convolution_sh_nontrivial() {
		let mut rng = rand::thread_rng();
		let func = |x:f32,y:f32,z:f32| x*x + y*z;

		let sh = SHFuncApproximation::from_function(func, &mut rng, 10000);
		
		// Convoluting constant function with constant is the same
		let result = sh.convolution(&sh);

		// We compute convolution in real space
		let normalized = integrate_real_space(|x,y,z| { let value = func(x,y,z); value*value }, &mut rng, 10000);
		let expected = normalized;
		assert!( (result - expected).abs() < 0.3, "Result is {0}, expected {1}", result, expected);
	}

	#[test]
	fn eval_trivial() {
		let mut rng = rand::thread_rng();
		let func = |x:f32,_y:f32,_z:f32| x*x;

		let sh = SHFuncApproximation::from_function(func, &mut rng, 10000);
		let mut workspace = SHFuncApproximation::new();

		// Convoluting constant function with constant is the same
		let result = sh.eval(Direction::new(1f32,0f32,0f32), &mut workspace);

		let expected = 1f32;
		assert!( (result - expected).abs() < 0.1, "Result is {0}, expected {1}", result, expected);
	}

}




