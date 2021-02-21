use pigpiod_if2::*;
use std::{thread, time::Duration};

struct NeoPixel {
    spi: Spi,
    buf: Vec<u8>,
    colors: Vec<u32>, // GRB
}

// Reference: https://tutorials-raspberrypi.com/connect-control-raspberry-pi-ws2812-rgb-led-strips/
impl NeoPixel {
    fn new(spi: Spi, num_pixels: usize) -> Self {
        Self {
            spi,
            buf: vec![0u8; num_pixels * 24],
            colors: vec![032; num_pixels],
        }
    }

    fn set_color(&mut self, i: usize, color: (u8, u8, u8)) {
        self.colors[i] = ((color.1 as u32) << 16) | ((color.0 as u32) << 8) | color.2 as u32
    }

    fn show(&mut self) {
        for (i, c) in self.colors.iter().enumerate() {
            for j in 0..24 {
                self.buf[24 * i + j] = if ((c >> (23 - j)) & 0b1) == 0b00 {
                    0xE0
                } else {
                    0xF8
                };
            }
        }
        self.spi
            .write(&self.buf)
            .expect("failed to write to the SPI device");
        thread::sleep(Duration::from_micros(1));
    }
}

fn main() {
    let pigpio = Pigpio::new().expect("failed to build pigpiod client");

    let spi_channel = 0;
    let spi_flags = 0;
    let spi = pigpio
        .spi(spi_channel, 6_400_000, spi_flags)
        .expect("failed to initiate an SPI device");

    let num_pixels = 2;
    let mut neo_pixel = NeoPixel::new(spi, num_pixels);

    loop {
        for c in 0..=255 {
            neo_pixel.set_color(0, (c, 0, 255 - c));
            neo_pixel.set_color(1, (255 - c, 0, c));
            neo_pixel.show();
            thread::sleep(Duration::from_millis(10));
        }
    }
}
