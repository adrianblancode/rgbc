use minifb::{Key, Scale, Window, WindowOptions};
use crate::Memory;

const WIDTH: usize = 0xff;
const HEIGHT: usize = 0xff;

pub struct Frontend {
    window: Window,
    buffer: Vec<u32>
}

impl Frontend {

    pub fn new() -> Frontend {

        let mut options = WindowOptions::default();
        options.scale = Scale::X2;

        let mut window = Window::new(
            "rgbc",
            WIDTH,
            HEIGHT,
            options
        )
            .unwrap_or_else(|e| {
                panic!("{}", e);
            });

        // Limit to max ~60 fps update rate
        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        let buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

        window.update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();

        Frontend { window, buffer }
    }

    pub fn step(&mut self, mem: &Memory) {
        if self.window.is_open() {
            let mut iter = 0;
            for v in self.buffer.iter_mut() {
                if iter < mem.data.len() {
                    let data = mem.data[iter];
                    *v = ((data & 0b111 << 5) as u32) << 16 | ((((data & 0b111 << 3) << 2) as u32) << 8) | (((data & 0b11) << 6) as u32);
                    iter += 1;
                }
            }

            self.window
                .update_with_buffer(&self.buffer, WIDTH, HEIGHT)
                .unwrap();
        }
    }
}