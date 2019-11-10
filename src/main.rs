mod spherical_harmonics;
mod spherical_integration;
mod spherical;

use spherical::Direction;
use spherical_harmonics::SHFuncApproximation;
use spherical_integration::integrate_real_space_hemisphere;

fn main() {
	let mut rng = rand::thread_rng();

	// Here, we would probably need to load a cubemap
	let cubemap = |x,y,z| (x*y + 0.5f32*z + 0.25f32 * x + 0.05f32 * x * y) * (1f32 - x*x + y);

	// We are using Lambertian cosine lightning that only depends on the
	// normal of the surface, not the out camera direction. We can therefore
	// precalculate the integral for normal direction
	let lightning_per_normal = |nx, ny, nz| {
		let normal = Direction::new(nx, ny, nz);
		let lightning_function = |x,y,z| {
			let direction = Direction::new(x,y,z);
			let cosine = normal.dot(&direction);

			assert!(cosine >= 0f32, "We are integrating over half-hemisphere, should never receive negative cosine");

			// lightning function is sum of cosine * direction value term
			cosine * cubemap(x,y,z)
		};

		let mut internal_rng = rand::thread_rng();

		// We sum all contributions
		integrate_real_space_hemisphere(&normal, lightning_function, &mut internal_rng, 5000)
	};

	// Doing only 1000 samples should be sufficient as we only calculate 9 coefficients from.
	// TODO: parallelize this call
	let sh = SHFuncApproximation::from_function(lightning_per_normal, &mut rng, 1000);
    println!("Resulting coefficients are {:?}\n", sh);

    let compare_values = |dir| {
    	let mut workspace = SHFuncApproximation::new();

    	// We can use this simple, cost effective eval (that is just a direction SH generation
    	// + DOT product) instead of evaluating the integral, or reading from big 'texture'
    	let sh_value = sh.eval(dir, &mut workspace);

    	// TODO: we are recalculating the integral here, possibly with different value. This
    	// may bring more difference
    	let direct_value = lightning_per_normal(dir.x, dir.y, dir.z);

    	print!("Compare values for '{0:?}': SH {1}, direct {2}\n", dir, sh_value, direct_value);
    };


    compare_values(Direction::new(1f32, 0f32, 0f32));
    compare_values(Direction::new(0f32, -1f32, 0f32));
    compare_values(Direction::new(0f32, 0f32, -1f32));
    compare_values(Direction::new(0f32, 1f32/2f32.sqrt(), 1f32/2f32.sqrt()));
}


