# pigpiod-if2

Rust wrapper of pigpiod_if2 C API.

## Dependencies

Please refer to the documentation of the depending `pigpiod-if2-sys` crate.

## Limitations

Only SPI routines are supported right now. The crate has only been tested with Raspberry Pi 4 and a
WS2812B strip. The author struggled to find a reliable method to control a WS2812B strip using the
userspace spidev API (e.g. flickers, the first LED mal-functioning), and ended up with the solution
involving pigpiod.
