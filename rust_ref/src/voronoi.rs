use crate::*;


pub fn voronoi_test()
{	let mut img = image::ImageBuffer::new(512, 512);
	let grid_size = 10;
	let pixels_per_cell = Vec2u::new(img.width(), img.height());


	let mut tmp = image::ImageBuffer::new(512, 512);
	let mut rng = RNG::new();
	println!("{}", rng.0);
	for (x, y, pixel) in tmp.enumerate_pixels_mut()
	{	let r = (0.3 * x as f32) as u8;
		let g = (0.3 * y as f32) as u8;

		rng = rng.lehmer();
		let b = rng.clone().get_u8();
		*pixel = image::Rgb([r, g, b]); }


	for (x, y, pixel) in img.enumerate_pixels_mut()
	{	*pixel = tmp[(x, y)]; }

	let err_code = img.save("voronoi.png");
	println!("{:?}", err_code); }
