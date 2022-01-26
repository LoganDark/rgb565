#![allow(unused)]

pub use transforms::{pack_565, unpack_565};
use with_std::{srgb_transfer, srgb_untransfer};

#[cfg(feature = "std")]
#[path = "./std.rs"]
mod with_std;

#[cfg(not(feature = "std"))]
mod with_std {
	fn srgb_transfer(v: f32) -> f32 { unimplemented!() }

	fn srgb_untransfer(v: f32) -> f32 { unimplemented!() }
}

#[path = "./transforms.rs"]
mod transforms;

/// `Lutable` represents a transformation that may or may not be backed by a
/// look-up table (LUT) depending on the features that were enabled for this
/// crate.
pub struct Lutable<I: MapIn, O: MapOut<S>, const N: usize, const S: usize>(Result<&'static [u8; N], fn(I) -> O>);

/// Trait for values that can be looked up in a LUT.
pub trait MapIn {
	#[inline]
	fn map_in(self) -> usize;
}

/// Trait for values that can be retrieved from a LUT.
pub trait MapOut<const N: usize> {
	#[inline]
	fn map_out(bytes: [u8; N]) -> Self;
}

impl MapIn for u8 {
	fn map_in(self) -> usize { self as usize }
}

impl MapIn for u16 {
	fn map_in(self) -> usize { self as usize }
}

impl MapIn for [u8; 3] {
	fn map_in(self) -> usize { u32::from_be_bytes([0, self[0], self[1], self[2]]) as usize }
}

impl MapOut<1> for u8 {
	fn map_out(bytes: [u8; 1]) -> Self { bytes[0] }
}

impl MapOut<2> for u16 {
	fn map_out(bytes: [u8; 2]) -> Self { u16::from_le_bytes(bytes) }
}

impl MapOut<3> for [u8; 3] {
	fn map_out(bytes: [u8; 3]) -> Self { bytes }
}

impl<I: MapIn, O: MapOut<S>, const N: usize, const S: usize> Lutable<I, O, N, S> {
	#[inline]
	pub fn map(&self, value: I) -> O {
		match &self.0 {
			Ok(lut) => {
				let index = value.map_in() * S;
				let bytes: [u8; S] = unsafe { lut[index..index + S].try_into().unwrap_unchecked() };
				O::map_out(bytes)
			}

			Err(transform) => transform(value)
		}
	}
}

macro_rules! pick_empty {
	(($something:tt); $whatever:tt; $else:tt) => { $whatever };
	((); $whatever:tt; $else:tt) => { $else }
}

macro_rules! lutable {
	{$($name:literal: $ident:ident[$size:literal $(* $mult:literal)?] => $i:ty, $o:ty => $func:expr);+;} => {
$(
#[cfg(feature = $name)]
pub const $ident: Lutable<$i, $o, { $size $(* $mult)? }, { 1 $(- 1 + $mult)? }> = Lutable(Ok(include_bytes!(concat!(env!("OUT_DIR"), "/", $name, ".bin"))));
#[cfg(not(feature = $name))]
pub const $ident: Lutable<$i, $o, { $size $(* $mult)? }, { 1 $(- 1 + $mult)? }> = Lutable(Err($func));
)+
	}
}

lutable! {
	"swap_components_lut": SWAP_COMPONENTS_LUT[65536 * 2] => u16, u16 => transforms::swap_components;
	"l5_to_l8_lut": L5_TO_L8_LUT[32] => u8, u8 => transforms::l5_to_l8;
	"l6_to_l8_lut": L6_TO_L8_LUT[64] => u8, u8 => transforms::l6_to_l8;
	"l5_to_s8_lut": L5_TO_S8_LUT[32] => u8, u8 => transforms::l5_to_s8;
	"l6_to_s8_lut": L6_TO_S8_LUT[64] => u8, u8 => transforms::l6_to_s8;
	"l565_to_l888_lut": L565_TO_L888_LUT[65536 * 3] => u16, [u8; 3] => transforms::l565_to_l888;
	"l565_to_s888_lut": L565_TO_S888_LUT[65536 * 3] => u16, [u8; 3] => transforms::l565_to_s888;
	"l8_to_l5_lut": L8_TO_L5_LUT[256] => u8, u8 => transforms::l8_to_l5;
	"l8_to_l6_lut": L8_TO_L6_LUT[256] => u8, u8 => transforms::l8_to_l6;
	"s8_to_l5_lut": S8_TO_L5_LUT[256] => u8, u8 => transforms::s8_to_l5;
	"s8_to_l6_lut": S8_TO_L6_LUT[256] => u8, u8 => transforms::s8_to_l6;
	"l888_to_l565_lut": L888_TO_L565_LUT[16777216 * 2] => [u8; 3], u16 => transforms::l888_to_l565;
	"s888_to_l565_lut": S888_TO_L565_LUT[16777216 * 2] => [u8; 3], u16 => transforms::s888_to_l565;
}

#[cfg(test)]
mod tests {
	use super::SWAP_COMPONENTS_LUT;

	#[test]
	fn swap_components() {
		assert_eq!(SWAP_COMPONENTS_LUT.map(0b1111100000000000), 0b0000000000011111);
		assert_eq!(SWAP_COMPONENTS_LUT.map(0b0000000000011111), 0b1111100000000000);
		assert_eq!(SWAP_COMPONENTS_LUT.map(0b1111111111100000), 0b0000011111111111);
		assert_eq!(SWAP_COMPONENTS_LUT.map(0b0000011111111111), 0b1111111111100000);
		assert_eq!(SWAP_COMPONENTS_LUT.map(0b1111111111111111), 0b1111111111111111);
		assert_eq!(SWAP_COMPONENTS_LUT.map(0b0000000000000000), 0b0000000000000000);
	}
}
