pub fn srgb_transfer(v: f32) -> f32 {
	if v < 0.0031308 {
		v * 12.9232102
	} else {
		1.055 * v.powf(1.0 / 2.4) - 0.055
	}
}

pub fn srgb_untransfer(v: f32) -> f32 {
	if v < 0.0404599 {
		v / 12.9232102
	} else {
		((v + 0.055) / 1.055).powf(2.4)
	}
}
