#![allow(dead_code)]

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
	pub fn get_u8(self) -> u8 { (self.0 >> (u64::BITS - 8)) as u8 }
	pub fn get_u16(self) -> u16 { (self.0 >> (u16::BITS - 16)) as u16 }
	pub fn get_u32(self) -> u32 { (self.0 >> (u32::BITS - 32)) as u32 } }

pub struct Vec2i { w: i32, h: i32 }
impl Vec2i
{	pub fn new(w: i32, h: i32) -> Vec2i { Vec2i { w: w, h: h }} }

pub struct Vec2u { w: u32, h: u32 }
impl Vec2u
{	pub fn new(w: u32, h: u32) -> Vec2u { Vec2u { w: w, h: h }} }
