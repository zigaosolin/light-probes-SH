mod spherical_harmonics;
mod spherical_integration;
mod spherical;

use spherical::Direction;
use spherical_harmonics::SHFuncApproximation;

fn main() {
	let direction = Direction::new(0f32, 1f32, 0f32);
	let mut sh = SHFuncApproximation::new();
	sh.from_direction(direction);
    println!("Result is {:?}", sh);
}
