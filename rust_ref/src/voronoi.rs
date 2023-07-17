use crate::*;
use raylib::{prelude::*, ffi::DrawPixel};

pub fn voronoi_test()
{	let mut img = Vec2d::<Color8>::new(512, 512);

	let mut rng = RNG::new();
	println!("{}", rng.seed);

	for (i, col) in img.vec.iter_mut().enumerate()
	{	let pos = Vec2i::expand_1d_to_2d(i as i32, img.row as i32);
		*col = Color8{
			r: 50,
			g: (pos.x as f32 * 0.3) as u8,
			b: rng.get_u8(),
			a: 255
		}; }

	let (mut rl, thread) = raylib::init()
		.size(512, 512)
		.title("Hello, World")
		.build();

	while !rl.window_should_close()
	{	let mut d = rl.begin_drawing(&thread);

		d.clear_background(Color::WHITE);

		// draw the image
		for (i, col) in img.vec.iter().enumerate()
		{	let pos = Vec2i::expand_1d_to_2d(i as i32, img.row as i32);
			let pen = Color{r: col.r, g: col.g, b: col.b, a: col.a};
			unsafe { DrawPixel(pos.x, pos.y, pen.into()); } }

		d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK); } }
