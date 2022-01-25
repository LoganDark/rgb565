#![allow(unused)]

use super::{srgb_transfer, srgb_untransfer};

macro_rules! transforms {
	{$($func:ident($arg:ident: $ty:ty) -> $ret:ty $calc:block)+} => {
		$(pub fn $func($arg: $ty) -> $ret $calc)+
	}
}

#[inline]
pub fn unpack_565(packed: u16) -> (u8, u8, u8) {
	((packed >> 11 & 0b11111) as u8, (packed >> 5 & 0b111111) as u8, (packed & 0b11111) as u8)
}

#[inline]
pub fn pack_565((r5, g6, b5): (u8, u8, u8)) -> u16 {
	debug_assert!(r5 & 0b11111 == r5, "r5 channel too wide");
	debug_assert!(g6 & 0b111111 == g6, "g6 channel too wide");
	debug_assert!(b5 & 0b11111 == b5, "b5 channel too wide");

	(r5 as u16) << 11 | (g6 as u16) << 5 | b5 as u16
}

transforms! {
	swap_components(rgb565: u16) -> u16 { rgb565 & 0b11111100000 | rgb565 >> 11 | rgb565 << 11 }

	l5_to_l8(l5: u8) -> u8 { (l5 as u16 * 255 / 0b11111) as u8 }
	l6_to_l8(l6: u8) -> u8 { (l6 as u16 * 255 / 0b111111) as u8 }
	l5_to_s8(l5: u8) -> u8 { (srgb_transfer(l5 as f32 / 31.0) * 255.0) as u8 }
	l6_to_s8(l6: u8) -> u8 { (srgb_transfer(l6 as f32 / 63.0) * 255.0) as u8 }

	l565_to_l888(l565: u16) -> [u8; 3] {
		let (r, g, b) = unpack_565(l565);
		[l5_to_l8(r), l6_to_l8(g), l5_to_l8(b)]
	}

	l565_to_s888(l565: u16) -> [u8; 3] {
		let (r, g, b) = unpack_565(l565);
		[l5_to_s8(r), l6_to_s8(g), l5_to_s8(b)]
	}

	l8_to_l5(l8: u8) -> u8 { ((l8 as u16 + 1) * 0b11111 / 255) as u8 }
	l8_to_l6(l8: u8) -> u8 { ((l8 as u16 + 1) * 0b111111 / 255) as u8 }
	s8_to_l5(s8: u8) -> u8 { (srgb_untransfer(s8 as f32 / 255.0) * 31.999) as u8 }
	s8_to_l6(s8: u8) -> u8 { (srgb_untransfer(s8 as f32 / 255.0) * 63.999) as u8 }

	l888_to_l565(l888: [u8; 3]) -> u16 {
		let [r, g, b] = l888;
		pack_565((l8_to_l5(r), l8_to_l6(g), l8_to_l5(b)))
	}

	s888_to_l565(s888: [u8; 3]) -> u16 {
		let [r, g, b] = s888;
		pack_565((s8_to_l5(r), s8_to_l6(g), s8_to_l5(b)))
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn swap_components() {
		assert_eq!(super::swap_components(0b1111100000000000), 0b0000000000011111);
		assert_eq!(super::swap_components(0b0000000000011111), 0b1111100000000000);
		assert_eq!(super::swap_components(0b1111111111100000), 0b0000011111111111);
		assert_eq!(super::swap_components(0b0000011111111111), 0b1111111111100000);
		assert_eq!(super::swap_components(0b1111111111111111), 0b1111111111111111);
		assert_eq!(super::swap_components(0b0000000000000000), 0b0000000000000000);
	}
}
