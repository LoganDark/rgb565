#![allow(unused)]

pub use transforms::{pack_565, unpack_565};
#[cfg(feature = "std")]
use with_std::{srgb_transfer, srgb_untransfer};

#[cfg(feature = "std")]
#[path = "./std.rs"]
mod with_std;

#[cfg(not(feature = "std"))]
fn srgb_transfer(v: f32) -> f32 { unimplemented!() }

#[cfg(not(feature = "std"))]
fn srgb_untransfer(v: f32) -> f32 { unimplemented!() }

#[path = "./transforms.rs"]
mod transforms;

/// `Lutable` represents a transformation that may or may not be backed by a
/// look-up table (LUT) depending on the features that were enabled for this
/// crate.
pub struct Lutable<A, R, const N: usize, const S: usize, const T: usize>(Result<(&'static [u8; T], fn(A) -> usize, fn([u8; S]) -> R), fn(A) -> R>);

impl<A, R, const N: usize, const S: usize, const T: usize> Lutable<A, R, N, S, T> {
	pub fn map(&self, value: A) -> R {
		match &self.0 {
			Ok((lut, map_in, map_out)) => {
				let index = map_in(value) * S;
				let slice: [u8; S] = lut[index..index + S].try_into().unwrap();
				map_out(slice)
			}
			Err(transform) => {
				transform(value)
			}
		}
	}
}

macro_rules! pick_empty {
	(($something:tt); $whatever:tt; $else:tt) => { $whatever };
	((); $whatever:tt; $else:tt) => { $else }
}

macro_rules! lutable {
	{$($name:literal: $ident:ident[$size:literal $(* $mult:literal)?] => $a:ty |$map_in_pat:pat_param| $map_in_expr:expr, $r:ty |$map_out_pat:pat_param| $map_out_expr:expr => $func:expr);+;} => {
$(
#[cfg(feature = $name)]
#[allow(non_snake_case)]
mod $ident {
	#[cfg(feature = $name)]
	pub fn map_in($map_in_pat: $a) -> usize {
		$map_in_expr
	}

	#[cfg(feature = $name)]
	pub fn map_out($map_out_pat: [u8; 1 $(- 1 + $mult)?]) -> $r {
		$map_out_expr
	}
}

#[cfg(feature = $name)]
pub const $ident: Lutable<$a, $r, { $size }, { 1 $(- 1 + $mult)? }, { $size $(* $mult)? }> = Lutable(Ok((include_bytes!(concat!(env!("OUT_DIR"), "/", $name, ".bin")), $ident::map_in, $ident::map_out)));
#[cfg(not(feature = $name))]
pub const $ident: Lutable<$a, $r, { $size }, { 1 $(- 1 + $mult)? }, { $size $(* $mult)? }> = Lutable(Err($func));
)+
	}
}

lutable! {
	"swap_components_lut": SWAP_COMPONENTS_LUT[65536 * 2] => u16 |rgb565| rgb565 as usize, u16 |bytes| u16::from_le_bytes(bytes) => transforms::swap_components;
	"l5_to_l8_lut": L5_TO_L8_LUT[32] => u8 |l5| l5 as usize, u8 |[l8]| l8 => transforms::l5_to_l8;
	"l6_to_l8_lut": L6_TO_L8_LUT[64] => u8 |l6| l6 as usize, u8 |[l8]| l8 => transforms::l6_to_l8;
	"l5_to_s8_lut": L5_TO_S8_LUT[32] => u8 |l5| l5 as usize, u8 |[s8]| s8 => transforms::l5_to_s8;
	"l6_to_s8_lut": L6_TO_S8_LUT[64] => u8 |l6| l6 as usize, u8 |[s8]| s8 => transforms::l6_to_s8;
	"l565_to_l888_lut": L565_TO_L888_LUT[65536 * 3] => u16 |l565| l565 as usize, [u8; 3] |l888| l888 => transforms::l565_to_l888;
	"l565_to_s888_lut": L565_TO_S888_LUT[65536 * 3] => u16 |l565| l565 as usize, [u8; 3] |s888| s888 => transforms::l565_to_s888;
	"l8_to_l5_lut": L8_TO_L5_LUT[256] => u8 |l8| l8 as usize, u8 |[l5]| l5 => transforms::l8_to_l5;
	"l8_to_l6_lut": L8_TO_L6_LUT[256] => u8 |l8| l8 as usize, u8 |[l6]| l6 => transforms::l8_to_l6;
	"s8_to_l5_lut": S8_TO_L5_LUT[256] => u8 |s8| s8 as usize, u8 |[l5]| l5 => transforms::s8_to_l5;
	"s8_to_l6_lut": S8_TO_L6_LUT[256] => u8 |s8| s8 as usize, u8 |[l6]| l6 => transforms::s8_to_l6;
	"l888_to_l565_lut": L888_TO_L565_LUT[16777216 * 2] => [u8; 3] |[r, g, b]| u32::from_be_bytes([0, r, g, b]) as usize, u16 |bytes| u16::from_le_bytes(bytes) => transforms::l888_to_l565;
	"s888_to_l565_lut": S888_TO_L565_LUT[16777216 * 2] => [u8; 3] |[r, g, b]| u32::from_be_bytes([0, r, g, b]) as usize, u16 |bytes| u16::from_le_bytes(bytes) => transforms::s888_to_l565;
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
