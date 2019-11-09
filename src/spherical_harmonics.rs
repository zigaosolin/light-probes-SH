// A direction, represented as floats. The constraint it keeps is
// that the length is always 1
#[derive(Debug)]
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
}

// Spherical harmonic coefficients up to order 3. Represents coefficients only for
// one channel
#[derive(Debug, Clone)]
pub struct SH3 {
	coefficients : Vec<f32> 
}

impl SH3 {
	pub fn new() -> SH3 {
		SH3 { coefficients: vec![0f32; 9]}
	}

	pub fn mul_in_place(&mut self, scalar : f32) {
		for i in 0..9 {
			self.coefficients[i] *= scalar;
		}
	}

	pub fn add_in_place(&mut self, other: SH3) {
		for i in 0..9 {
			self.coefficients[i] += other.coefficients[i];
		}
	}

	pub fn convolution(&self, other : &SH3) -> f32 {
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
}




