use minifb::{Key, Scale, Window, WindowOptions};
use crate::Memory;

const WIDTH: usize = 160;
const HEIGHT: usize = 144;

pub struct Frontend {
    window: Window,
    buffer: Vec<u32>,
}

impl Frontend {
    pub fn new() -> Frontend {
        let mut options = WindowOptions::default();
        options.scale = Scale::X4;

        let mut window = Window::new(
            "rgbc",
            WIDTH,
            HEIGHT,
            options,
        )
            .unwrap_or_else(|e| {
                panic!("{}", e);
            });

        // Limit to max ~60 fps update rate
        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
        buffer.fill(0xFFFFFF);

        window.update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();

        Frontend { window, buffer }
    }

    pub fn step(&mut self, mem: &Memory) {
        if self.window.is_open() {
            self.draw_tilebuffer(mem)
        }
    }

    fn draw_tilebuffer(&mut self, mem: &Memory) {

        let colors: [u32;4] = [0xFFFFFF, 0x000000, 0x444444, 0x888888];

        let bytes_per_tile = 16;
        let tile_side = 8;

        let mut t = 0;
        while 0x8000 + t * bytes_per_tile < 0x8800 {

            let tile_y = (t * tile_side) / WIDTH;
            let tile_x = (t * tile_side) % WIDTH;
            let tile_addr = tile_y * tile_side * WIDTH + tile_x;

            let source_addr = 0x8000 + t * bytes_per_tile;

            // For each tile row
            for i in 0..tile_side {
                // There are two bytes
                let byte1: u8 = mem.data[source_addr + i * 2];
                let byte2: u8 = mem.data[source_addr + i * 2 + 1];

                for p in 0..tile_side {
                    // Each tile pixel consists of two bits from each byte
                    let bit1 = (byte1 >> (7 - p)) & 1;
                    let bit2 = (byte2 >> (7 - p)) & 1;

                    // Final color is two bits color depth, as index to palette
                    let c = bit2 << 1 | bit1;

                    let tile_pixel_y = i;
                    let tile_pixel_x = p;
                    let tile_pixel = tile_pixel_y * WIDTH + tile_pixel_x;

                    self.buffer[tile_addr + tile_pixel] = colors[c as usize];
                }
            }

            t += 1;
        }

        self.window
            .update_with_buffer(&self.buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}