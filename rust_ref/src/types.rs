#![allow(dead_code)]
// #[inline] is forbidden. Just use LTO!

/// # Example
/// ```
/// let mut rng = RNG::new();
/// rng = rng.lehmer();
/// let num = rng.0;
///
/// let blue = rng.clone().get_u8(); // 255
///
/// println!("{}", rng.0);
/// println!("{}", blue);
///
/// rng = rng.lehmer();
/// let blue = rng.clone().get_u8();
/// println!("{}", blue); // 53
/// ```
#[derive(Clone)]
pub struct RNG(pub u64);
impl RNG
{	pub fn new() -> RNG { RNG(1023) }
	pub fn set(mut self, num: u64) -> Self { self.0 = num; self }
	pub fn lehmer(mut self) -> Self { self.0 = u64::wrapping_mul(self.0, 0xd1342543de82ef95); self }
	// LOL! Don't try to use bitwise AND "& 255" or modulo "% 256" on this RNG!
	pub fn get_u8(self) -> u8 { (self.0 >> (u64::BITS - 8)) as u8 }
	pub fn get_u16(self) -> u16 { (self.0 >> (u64::BITS - 16)) as u16 }
	pub fn get_u32(self) -> u32 { (self.0 >> (u64::BITS - 32)) as u32 }}

#[derive(Debug)]
pub struct Vec2i { pub x: i32, pub y: i32 }
impl Vec2i
{	pub fn new(x: i32, y: i32) -> Vec2i { Vec2i{ x, y } }
	pub fn expand_1d_to_2d(i: i32, row: i32) -> Vec2i { Vec2i{ x: i % row, y: i / row } }
	pub fn flat_2d_to_1d(vec: Vec2i, row: i32) -> i32 { vec.x + (vec.y * row) }}

pub struct Vec2u { pub x: u32, pub y: u32 }
impl Vec2u
{	pub fn new(x: u32, y: u32) -> Vec2u { Vec2u{ x, y } } }

pub struct Vec3i { pub x: i32, pub y: i32, pub z: i32 }
impl Vec3i
{	pub fn new(x: i32, y: i32, z: i32) -> Vec3i { Vec3i{ x, y, z } }
	pub fn expand_1d_to_3d(i: i32, row: i32, col: i32) -> Vec3i { Vec3i{ x: i % row, y: i / row, z: i / (row * col) } }
	pub fn flat_3d_to_1d(vec: Vec3i, row: i32, col: i32) -> i32 { vec.x + (vec.y * row) + (vec.z * row * col) }}


/// So, Vec is a pointer located in stack that points to the heap.
/// And Vec with Vec is a pointer to pointer to the heap.
/// It's all over the place. We don't want that.
/// So if you want to give each 1D Vec a different size. Then don't use this.
/// Else, use this.
pub struct Vec2d<T> { pub vec: Vec<T>, row: usize, col: usize }
impl Vec2d<u8>
{	pub fn new(row: usize, col: usize) -> Vec2d<u8> { Vec2d { vec: vec![0; row * col], row, col } }
	pub fn get_index(&self, x: usize, y: usize) -> u8 { self.vec[x + (y * self.row)] }}
