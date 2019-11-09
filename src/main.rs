use spherical_harmonics::project_fn;


fn sample_fun(theta: f32, phi: f32) -> f32 {
	theta.cos() * (phi*2.0).cos()
}

fn main() {
	let mut rng = rand::thread_rng();
	let result = project_fn(3, 100000, &mut rng, sample_fun);


    println!("Result is {:?}", result);
}
