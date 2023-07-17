use crate::*;
use raylib::{prelude::*, ffi::DrawPixel};

pub fn voronoi_test()
{	let mut img = Vec2d::new(512, 512);
	let mut tmp = vec![vec![Color::BLACK; 512]; 512];

	let mut rng = RNG::new();
	println!("{}", rng.0);
	for (i, arr) in tmp.iter_mut().enumerate()
	{	
		// println!("{:?} {:?}", i, arr[i]);
	}
	// println!("{:?}", tmp);

	for i in 0..img.vec.len()
	{	print!("{:?} ", img.get_index(i % 512, i / 512)); }
	println!();

	let (mut rl, thread) = raylib::init()
		.size(512, 512)
		.title("Hello, World")
		.build();

	while !rl.window_should_close() {
		let mut d = rl.begin_drawing(&thread);

		d.clear_background(Color::WHITE);

		// draw the image
		unsafe {
		for (i, arr) in tmp.iter().enumerate()
		{	let pos = Vec2i::expand_1d_to_2d(i as i32, 512);
			DrawPixel(pos.x, pos.y, Color::BLACK.into());
			// println!("{:?}", pos);
		}
		}

		d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
	}
}
