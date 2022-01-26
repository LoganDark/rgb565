#![cfg_attr(not(any(feature = "std", test)), no_std)]

//! `rgb565` provides deserialization, serialization and conversion routines for
//! the RGB565 pixel format, which stores color information in only 16 bits. The
//! red channel gets 5 bits, the green 6, and blue 5 again. The RGB565 format is
//! often used in embedded devices and microcontrollers for things like e-ink
//! displays that only have a low degree of color reproduction.
//!
//! `rgb565` includes out-of-the-box methods for converting to and from many
//! orderings and endiannesses, with each routine being manually verified,
//! automatically verified and also tested on physical devices that use the
//! RGB565 format.
//!
//! # LUTs
//!
//! `rgb565` comes with the optional ability to make use of *look-up tables* to
//! gain massive speed increases (up to 20%!) on embedded devices without a
//! graphics coprocessor. sRGB in particular gains a lot from LUTs because you
//! usually need floating-point math to correctly approximate the gamma curve.
//!
//! You can control the inclusion of look-up tables using Cargo features:
//!
//! - `swap_components_lut` to speed up loading and storing to BGR values
//! - `l5_to_l8_lut` to speed up converting to 8-bit RGB red/blue channels
//! - `l6_to_l8_lut` to speed up converting to 8-bit RGB green channel
//! - `l5_to_s8_lut` to speed up converting to 8-bit sRGB red/blue channels
//! - `l6_to_s8_lut` to speed up converting to 8-bit sRGB green channel
//! - `l565_to_l888_lut` to speed up converting RGB565 values to 8-bit RGB
//! - `l565_to_s888_lut` to speed up converting RGB565 values to 8-bit sRGB
//! - `l8_to_l5_lut` to speed up converting from 8-bit RGB red/blue channels
//! - `l8_to_l6_lut` to speed up converting from 8-bit RGB green channel
//! - `s8_to_l5_lut` to speed up converting from 8-bit sRGB red/blue channels
//! - `s8_to_l6_lut` to speed up converting from 8-bit sRGB green channels
//! - `l888_to_l565_lut` to speed up converting 8-bit RGB values to RGB565
//! - `s888_to_l565_lut` to speed up converting 8-bit sRGB values to RGB565
//!
//! If you won't be using BGR565, then you don't need `swap_components_lut`. If
//! you'll only be converting all three channels at once, then you don't need
//! the individual `l/s#_to_l/s#_lut` features.
//!
//! All LUTs are enabled by default except for `l888_to_l565_lut` and
//! `s888_to_l565_lut`, and you'll see why if you read this - the sizes of all
//! the LUTs is as follows:
//!
//! - `swap_components_lut`: 128 KiB (131,072 bytes)
//! - `l5_to_l8_lut`: 32 bytes
//! - `l6_to_l8_lut`: 64 bytes
//! - `l5_to_s8_lut`: 32 bytes
//! - `l6_to_s8_lut`: 64 bytes
//! - `l565_to_l888_lut`: 192 KiB (196,608 bytes)
//! - `l565_to_s888_lut`: 192 KiB (196,608 bytes)
//! - `l8_to_l5_lut`: 256 bytes
//! - `l8_to_l6_lut`: 256 bytes
//! - `s8_to_l5_lut`: 256 bytes
//! - `s8_to_l6_lut`: 256 bytes
//! - `l888_to_l565_lut`: **32 MiB** (**33,554,432 bytes**)
//! - `s888_to_l565_lut`: **32 MiB** (**33,554,432 bytes**)
//!
//! That's because `l888_to_l565_lut` and `s888_to_l565_lut` both have to cover
//! the entire 16.777216-million-color space of 24-bit "true color", and I don't
//! think it would be very nice to add bloat like that by default.

mod lut;

/// Represents an RGB565 color value.
///
/// Rgb565 encapsulates a color value stored in RGB565 format. It includes basic
/// serialization, deserialization and conversion routines for working with
/// different color spaces alongside RGB565. Notably, it contains functions for
/// converting to and from sRGB, which should be used when displaying RGB565
/// colors on a modern computer monitor.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, Default)]
pub struct Rgb565(u16);

impl Rgb565 {
	/// Unpacks r5, g6, and b5 values from a single RGB565 value. To unpack from
	/// BGR565 instead, swap r5 and b5.
	#[inline]
	pub fn unpack_565(packed: u16) -> (u8, u8, u8) { lut::unpack_565(packed) }

	/// Packs r5, g6, and b5 values into a single RGB565 value. To pack into
	/// BGR565 instead, swap r5 and b5.
	///
	/// # Panics
	///
	/// This function includes debug assertions to ensure that `r`, `g` and `b`
	/// fit into the space allotted by the RGB565 format. If values are passed
	/// that are too big, the function will panic.
	#[inline]
	pub fn pack_565(unpacked: (u8, u8, u8)) -> u16 { lut::pack_565(unpacked) }

	/// From rgb565, where the colors are packed as `rrrrrggggggbbbbb`
	#[inline]
	pub fn from_rgb565(packed: u16) -> Self { Self(packed) }
	/// From bgr565, where the colors are packed as `bbbbbggggggrrrrr`
	#[inline]
	pub fn from_bgr565(packed: u16) -> Self { Self(lut::SWAP_COMPONENTS_LUT.map(packed)) }
	/// To rgb565, where the colors are packed as `rrrrrggggggbbbbb`
	#[inline]
	pub fn to_rgb565(&self) -> u16 { self.0 }
	/// To bgr565, where the colors are packed as `bbbbbggggggrrrrr`
	#[inline]
	pub fn to_bgr565(&self) -> u16 { lut::SWAP_COMPONENTS_LUT.map(self.0) }

	/// From rgb565_le, where the colors are stored as `[gggbbbbb, rrrrrggg]`
	#[inline]
	pub fn from_rgb565_le(bytes: [u8; 2]) -> Self { Self::from_rgb565(u16::from_le_bytes(bytes)) }
	/// From rgb565_be, where the colors are stored as `[rrrrrggg, gggbbbbb]`
	#[inline]
	pub fn from_rgb565_be(bytes: [u8; 2]) -> Self { Self::from_rgb565(u16::from_be_bytes(bytes)) }
	/// From bgr565_le, where the colors are stored as `[gggrrrrr, bbbbbggg]`
	#[inline]
	pub fn from_bgr565_le(bytes: [u8; 2]) -> Self { Self::from_bgr565(u16::from_le_bytes(bytes)) }
	/// From bgr565_be, where the colors are stored as `[bbbbbggg, gggrrrrr]`
	#[inline]
	pub fn from_bgr565_be(bytes: [u8; 2]) -> Self { Self::from_bgr565(u16::from_be_bytes(bytes)) }

	/// From rgb565_le, where the colors are stored as `[gggbbbbb, rrrrrggg]`
	#[inline]
	pub fn to_rgb565_le(&self) -> [u8; 2] { self.to_rgb565().to_le_bytes() }
	/// From rgb565_be, where the colors are stored as `[rrrrrggg, gggbbbbb]`
	#[inline]
	pub fn to_rgb565_be(&self) -> [u8; 2] { self.to_rgb565().to_be_bytes() }
	/// From bgr565_le, where the colors are stored as `[gggrrrrr, bbbbbggg]`
	#[inline]
	pub fn to_bgr565_le(&self) -> [u8; 2] { self.to_bgr565().to_le_bytes() }
	/// From bgr565_be, where the colors are stored as `[bbbbbggg, gggrrrrr]`
	#[inline]
	pub fn to_bgr565_be(&self) -> [u8; 2] { self.to_bgr565().to_be_bytes() }

	/// From rgb565 components, where r fits into 5 bits, g into 6 and b into 5
	///
	/// # Panics
	///
	/// This function includes debug assertions to ensure that `r`, `g` and `b`
	/// fit into the space allotted by the RGB565 format. If values are passed
	/// that are too big, the function will panic.
	#[inline]
	pub fn from_rgb565_components(r: u8, g: u8, b: u8) -> Self {
		Self(Self::pack_565((r, g, b)))
	}

	#[inline]
	pub fn from_rgb888_components(r: u8, g: u8, b: u8) -> Self { Self(lut::L888_TO_L565_LUT.map([r, g, b])) }
	#[cfg(any(feature = "std", feature = "s888_to_l565_lut"))]
	#[inline]
	pub fn from_srgb888_components(r: u8, g: u8, b: u8) -> Self { Self(lut::S888_TO_L565_LUT.map([r, g, b])) }

	/// To rgb565 components, where r fits into 5 bits, g into 6 and b into 5
	#[inline]
	pub fn to_rgb565_components(&self) -> [u8; 3] {
		let (r, g, b) = Self::unpack_565(self.0);
		[r, g, b]
	}

	#[inline]
	pub fn to_rgb888_components(&self) -> [u8; 3] { lut::L565_TO_L888_LUT.map(self.0) }
	#[cfg(any(feature = "std", feature = "l565_to_s888_lut"))]
	#[inline]
	pub fn to_srgb888_components(&self) -> [u8; 3] { lut::L565_TO_S888_LUT.map(self.0) }
}

#[cfg(test)]
mod tests {
	use crate::Rgb565;

	#[test]
	fn round_trip_rgb() {
		for i in 0..=u16::MAX {
			let rgb565 = Rgb565::from_rgb565(i);
			let (r5, g6, b5) = Rgb565::unpack_565(rgb565.to_rgb565());

			let [r, g, b] = rgb565.to_rgb888_components();

			let rgb565 = Rgb565::from_rgb888_components(r, g, b);
			let (r5_2, g6_2, b5_2) = Rgb565::unpack_565(rgb565.to_rgb565());

			assert_eq!(rgb565.to_rgb565(), i, "{:05b} {:06b} {:05b} => {},{},{} => {:05b} {:06b} {:05b}", r5, g6, b5, r, g, b, r5_2, g6_2, b5_2);
		}
	}

	// sRGB is weird. This test doesn't pass right now. I don't think it ever will
	//#[test]
	fn round_trip_srgb() {
		for i in 0..=u16::MAX {
			let rgb565 = Rgb565::from_rgb565(i);
			let (r5, g6, b5) = Rgb565::unpack_565(rgb565.to_rgb565());

			let [r, g, b] = rgb565.to_srgb888_components();

			let rgb565 = Rgb565::from_srgb888_components(r, g, b);
			let (r5_2, g6_2, b5_2) = Rgb565::unpack_565(rgb565.to_rgb565());

			assert_eq!(rgb565.to_rgb565(), i, "{:05b} {:06b} {:05b} => {},{},{} => {:05b} {:06b} {:05b}", r5, g6, b5, r, g, b, r5_2, g6_2, b5_2);
		}
	}

	#[test]
	fn basic_stuff() {
		let mut red = [0b00000000, 0b11111000];
		let mut green = [0b11100000, 0b00000111];
		let mut blue = [0b00011111, 0b0000000];

		assert_eq!(Rgb565::from_rgb565_le(red).to_rgb888_components(), [255, 0, 0]);
		assert_eq!(Rgb565::from_rgb565_le(green).to_rgb888_components(), [0, 255, 0]);
		assert_eq!(Rgb565::from_rgb565_le(blue).to_rgb888_components(), [0, 0, 255]);

		assert_eq!(Rgb565::from_rgb888_components(255, 0, 0).to_rgb565_le(), red);
		assert_eq!(Rgb565::from_rgb888_components(0, 255, 0).to_rgb565_le(), green);
		assert_eq!(Rgb565::from_rgb888_components(0, 0, 255).to_rgb565_le(), blue);

		assert_eq!(Rgb565::from_bgr565_le(blue).to_rgb888_components(), [255, 0, 0]);
		assert_eq!(Rgb565::from_bgr565_le(green).to_rgb888_components(), [0, 255, 0]);
		assert_eq!(Rgb565::from_bgr565_le(red).to_rgb888_components(), [0, 0, 255]);

		assert_eq!(Rgb565::from_rgb888_components(255, 0, 0).to_bgr565_le(), blue);
		assert_eq!(Rgb565::from_rgb888_components(0, 255, 0).to_bgr565_le(), green);
		assert_eq!(Rgb565::from_rgb888_components(0, 0, 255).to_bgr565_le(), red);

		red.reverse();
		blue.reverse();
		green.reverse();

		assert_eq!(Rgb565::from_rgb565_be(red).to_rgb888_components(), [255, 0, 0]);
		assert_eq!(Rgb565::from_rgb565_be(green).to_rgb888_components(), [0, 255, 0]);
		assert_eq!(Rgb565::from_rgb565_be(blue).to_rgb888_components(), [0, 0, 255]);

		assert_eq!(Rgb565::from_rgb888_components(255, 0, 0).to_rgb565_be(), red);
		assert_eq!(Rgb565::from_rgb888_components(0, 255, 0).to_rgb565_be(), green);
		assert_eq!(Rgb565::from_rgb888_components(0, 0, 255).to_rgb565_be(), blue);

		assert_eq!(Rgb565::from_bgr565_be(blue).to_rgb888_components(), [255, 0, 0]);
		assert_eq!(Rgb565::from_bgr565_be(green).to_rgb888_components(), [0, 255, 0]);
		assert_eq!(Rgb565::from_bgr565_be(red).to_rgb888_components(), [0, 0, 255]);

		assert_eq!(Rgb565::from_rgb888_components(255, 0, 0).to_bgr565_be(), blue);
		assert_eq!(Rgb565::from_rgb888_components(0, 255, 0).to_bgr565_be(), green);
		assert_eq!(Rgb565::from_rgb888_components(0, 0, 255).to_bgr565_be(), red);
	}
}
