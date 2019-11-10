use crate::spherical::Direction;
use std::f32::consts::PI;
use rand::Rng;

pub fn integrate_real_space<F, R>(func: F, mut rand: &mut R, count: u32) -> f32 
	where F: Fn(f32, f32, f32) -> f32, R: Rng {

	let mut sum = 0f32;
	for _i in 0..count {
		let direction = Direction::generate_random_on_sphere(&mut rand);

		sum += func(direction.x, direction.y, direction.z);
	}

	4f32 * PI * sum / (count as f32)
}

pub fn integrate_real_space_hemisphere<F, R>(normal: &Direction, func: F, mut rand: &mut R, count: u32) -> f32 
	where F: Fn(f32, f32, f32) -> f32, R: Rng {

	let mut sum = 0f32;
	for _i in 0..count {
		let direction = Direction::generate_random_on_hemisphere(normal, &mut rand);

		sum += func(direction.x, direction.y, direction.z);
	}

	4f32 * PI * sum / (count as f32)
}
