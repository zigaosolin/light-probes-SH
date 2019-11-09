mod spherical_harmonics;

use spherical_harmonics::Direction;
use spherical_harmonics::spherical_harmonics_order3;


fn main() {
	let direction = Direction::new(0f32, 1f32, 0f32);
	let sh = spherical_harmonics_order3(direction);
    println!("Result is {:?}", sh);
}
