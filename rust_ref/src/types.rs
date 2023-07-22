#![allow(dead_code)]
// #[inline] is forbidden. Just use LTO!
// #[derive(Clone)] allow you to copy from the globals. No opinion. Again, only use it if you need it.
// #[derive(Copy)] is discouraged on vecs.
// As it copy all of the contents to another variable. Which is trashing the memory bandwidth.
// Only use it when you have to.


/// # Example
/// ```
///let mut rng = RNG::new();
///
///let blue = rng.get_u8();
///
///println!("{}", rng.get_u64()); // 3861869672507332423
///println!("{}", rng.seed); // 3861869672507332423
///println!("{}", rng.set(1000)); // 1000
///println!("{}", rng.get_u64()); // 3724917921348574728
///println!("{}", blue); // 255
///
///let blue = rng.get_u8();
///println!("{}", blue); // 64
/// ```
pub struct RNG{ pub seed: u64 }
impl RNG
{	pub fn new() -> RNG { RNG{ seed: 1023 } }
	pub fn set(&mut self, num: u64) -> u64 { self.seed = num; self.seed }
	pub fn get_u64(&mut self) -> u64 { self.seed = u64::wrapping_mul(self.seed, 0xd1342543de82ef95); self.seed }
	// LOL! Don't try to use bitwise AND "& 255" or modulo "% 256" on this RNG!
	pub fn get_u8(&mut self) -> u8 { (self.get_u64() >> (u64::BITS - 8)) as u8 }
	pub fn get_u16(&mut self) -> u16 { (self.get_u64() >> (u64::BITS - 16)) as u16 }
	pub fn get_u32(&mut self) -> u32 { (self.get_u64() >> (u64::BITS - 32)) as u32 } }

#[derive(Debug)]
pub struct Vec2i { pub x: i32, pub y: i32 }
impl Vec2i
{	pub fn new(x: i32, y: i32) -> Vec2i { Vec2i{ x, y } }
	pub fn expand_1d_to_2d(i: i32, row: i32) -> Vec2i { Vec2i{ x: i % row, y: i / row } }
	pub fn flat_2d_to_1d(vec: Vec2i, row: i32) -> i32 { vec.x + (vec.y * row) } }

pub struct Vec2u { pub x: u32, pub y: u32 }
impl Vec2u
{	pub fn new(x: u32, y: u32) -> Vec2u { Vec2u{ x, y } } }

pub struct Vec3i { pub x: i32, pub y: i32, pub z: i32 }
impl Vec3i
{	pub fn new(x: i32, y: i32, z: i32) -> Vec3i { Vec3i{ x, y, z } }
	pub fn expand_1d_to_3d(i: i32, row: i32, col: i32) -> Vec3i { Vec3i{ x: i % row, y: i / row % col, z: i / (row * col) } }
	pub fn flat_3d_to_1d(vec: Vec3i, row: i32, col: i32) -> i32 { vec.x + (vec.y * row) + (vec.z * row * col) } }


#[derive(Debug, Clone, Copy)]
pub struct Color8 { pub r: u8, pub g: u8, pub b: u8, pub a: u8}
impl Color8
{	pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color8 { Color8{ r, g, b, a } } }
pub const BLACK: Color8 = Color8{r: 0, g: 0, b: 0, a: 0};
/// So, Vec is a pointer located in stack that points to the heap.
/// And Vec with Vec is a pointer to pointer to the heap.
/// It's all over the place. We don't want that.
/// So if you want to give each 1D Vec a different size. Then don't use this.
/// Else, use this.
pub struct Vec2d<T> { pub vec: Vec<T>, pub row: usize, pub col: usize }
impl Vec2d<u8>
{	pub fn new(row: usize, col: usize) -> Vec2d<u8> { Vec2d { vec: vec![0; row * col], row, col } }
	pub fn get_index(&self, x: usize, y: usize) -> u8 { self.vec[x + (y * self.row)] } }
impl Vec2d<Color8>
{	pub fn new(row: usize, col: usize) -> Vec2d<Color8> { Vec2d { vec: vec![BLACK; row * col], row, col } }
	pub fn get_index(&self, x: usize, y: usize) -> Color8 { self.vec[x + (y * self.row)] } }
