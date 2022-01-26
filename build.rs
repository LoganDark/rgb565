use std::fs::File;
use std::io::{BufWriter, Write};

use with_std::{srgb_transfer, srgb_untransfer};

#[path = "src/std.rs"]
mod with_std;

#[path = "src/transforms.rs"]
mod transforms;

#[allow(unused)]
macro_rules! pick_empty {
	(($something:tt); $whatever:tt; $else:tt) => { $whatever };
	((); $whatever:tt; $else:tt) => { $else }
}

macro_rules! lut_gen {
	{$name:ident: $($feature:literal: $func:ident($size:literal $(* $mult:literal)?) => |$i:ident| $calc:expr),*} =>
{$(#[cfg(feature = $feature)]
fn $func(writer: &mut dyn Write) -> ::std::io::Result<()> {
	for $i in 0..=$size {
		writer.write_all(pick_empty!(($($mult)?); { $($calc as [u8; $mult])? }; { [$calc] }).as_slice())?;
	}

	Ok(())
}

)*const $name: &[(&str, fn(&mut dyn Write) -> ::std::io::Result<()>)] = &[$(
	#[cfg(feature = $feature)]
	(stringify!($func), $func)),*
];}
}

lut_gen! { LUTS:
	"swap_components_lut": swap_components_lut(65535u16 * 2) => |i| transforms::swap_components(i).to_le_bytes(),
	"l5_to_l8_lut": l5_to_l8_lut(31u8) => |i| transforms::l5_to_l8(i),
	"l6_to_l8_lut": l6_to_l8_lut(63u8) => |i| transforms::l6_to_l8(i),
	"l5_to_s8_lut": l5_to_s8_lut(31u8) => |i| transforms::l5_to_s8(i),
	"l6_to_s8_lut": l6_to_s8_lut(63u8) => |i| transforms::l6_to_s8(i),
	"l565_to_l888_lut": l565_to_l888_lut(65535u16 * 3) => |i| transforms::l565_to_l888(i),
	"l565_to_s888_lut": l565_to_s888_lut(65535u16 * 3) => |i| transforms::l565_to_s888(i),
	"l8_to_l5_lut": l8_to_l5_lut(255u8) => |i| transforms::l8_to_l5(i),
	"l8_to_l6_lut": l8_to_l6_lut(255u8) => |i| transforms::l8_to_l6(i),
	"s8_to_l5_lut": s8_to_l5_lut(255u8) => |i| transforms::s8_to_l5(i),
	"s8_to_l6_lut": s8_to_l6_lut(255u8) => |i| transforms::s8_to_l6(i),
	"l888_to_l565_lut": l888_to_l565_lut(16777215u32 * 2) => |i| transforms::l888_to_l565([(i >> 16) as u8, (i >> 8) as u8, i as u8]).to_le_bytes(),
	"s888_to_l565_lut": s888_to_l565_lut(16777215u32 * 2) => |i| transforms::s888_to_l565([(i >> 16) as u8, (i >> 8) as u8, i as u8]).to_le_bytes()
}

fn main() {
	let out_dir = std::env::var("OUT_DIR").unwrap();

	for (name, func) in LUTS.iter() {
		let file = File::create(format!("{}/{}.bin", out_dir, name)).unwrap();
		let mut writer = BufWriter::with_capacity(32 * 1024 * 1024, file);
		func(&mut writer).unwrap();
		writer.flush().unwrap();
	}
}
