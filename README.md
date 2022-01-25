# `rgb565`

`rgb565` provides deserialization, serialization and conversion routines for the
RGB565 pixel format, which stores color information in only 16 bits. The red
channel gets 5 bits, the green 6, and blue 5 again. The RGB565 format is often
used in embedded devices and microcontrollers for things like e-ink displays
that only have a low degree of color reproduction.

`rgb565` includes out-of-the-box methods for converting to and from many
orderings and endiannesses, with each routine being manually verified,
automatically verified and also tested on physical devices that use the RGB565
format.

## LUTs

`rgb565` comes with the optional ability to make use of *look-up tables* to gain
massive speed increases (up to 20%!) on embedded devices without a graphics
coprocessor. sRGB in particular gains a lot from LUTs because you usually need
floating-point math to correctly approximate the gamma curve.

You can control the inclusion of look-up tables using Cargo features:

- `swap_components_lut` to speed up loading and storing to BGR values
- `l5_to_l8_lut` to speed up converting to 8-bit RGB red/blue channels
- `l6_to_l8_lut` to speed up converting to 8-bit RGB green channel
- `l5_to_s8_lut` to speed up converting to 8-bit sRGB red/blue channels
- `l6_to_s8_lut` to speed up converting to 8-bit sRGB green channel
- `l565_to_l888_lut` to speed up converting RGB565 values to 8-bit RGB
- `l565_to_s888_lut` to speed up converting RGB565 values to 8-bit sRGB
- `l8_to_l5_lut` to speed up converting from 8-bit RGB red/blue channels
- `l8_to_l6_lut` to speed up converting from 8-bit RGB green channel
- `s8_to_l5_lut` to speed up converting from 8-bit sRGB red/blue channels
- `s8_to_l6_lut` to speed up converting from 8-bit sRGB green channels
- `l888_to_l565_lut` to speed up converting 8-bit RGB values to RGB565
- `s888_to_l565_lut` to speed up converting 8-bit sRGB values to RGB565

If you won't be using BGR565, then you don't need `swap_components_lut`. If
you'll only be converting all three channels at once, then you don't need the
individual `l/s#_to_l/s#_lut` features.

All LUTs are enabled by default except for `l888_to_l565_lut` and
`s888_to_l565_lut`, and you'll see why if you read this - the sizes of all the
LUTs is as follows:

- `swap_components_lut`: 128 KiB (131,072 bytes)
- `l5_to_l8_lut`: 32 bytes
- `l6_to_l8_lut`: 64 bytes
- `l5_to_s8_lut`: 32 bytes
- `l6_to_s8_lut`: 64 bytes
- `l565_to_l888_lut`: 192 KiB (196,608 bytes)
- `l565_to_s888_lut`: 192 KiB (196,608 bytes)
- `l8_to_l5_lut`: 256 bytes
- `l8_to_l6_lut`: 256 bytes
- `s8_to_l5_lut`: 256 bytes
- `s8_to_l6_lut`: 256 bytes
- `l888_to_l565_lut`: **32 MiB** (**33,554,432 bytes**)
- `s888_to_l565_lut`: **32 MiB** (**33,554,432 bytes**)

That's because `l888_to_l565_lut` and `s888_to_l565_lut` both have to cover the
entire 16.777216-million-color space of 24-bit "true color", and I don't think
it would be very nice to add bloat like that by default.

## Building

```
$ cargo build
```

## Testing

```
cargo test
```

## License

This repository and all source code it contains is licensed under the MIT
License, mainly because I want to include it in [`libremarkable`](
https://github.com/canselcik/libremarkable/), which is also licensed under MIT.
