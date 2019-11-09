mod spherical_harmonics;

use spherical_harmonics::Direction;
use spherical_harmonics::SH3;

fn main() {
	let direction = Direction::new(0f32, 1f32, 0f32);
	let mut sh3 = SH3::new();
	sh3.from_direction(direction);
    println!("Result is {:?}", sh3);
}
