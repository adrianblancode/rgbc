use minifb::{Scale, Window, WindowOptions};
use crate::gpu::{WIDTH, HEIGHT, Gpu};

pub struct Frontend {
    window: Window,
    buffer: [u32;WIDTH*HEIGHT],
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

        window.set_background_color(255, 255, 255);
        // Limit to max ~60 fps update rate
        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        let buffer = [0xFFFFFF; WIDTH * HEIGHT];
        window.update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();

        Frontend { window, buffer }
    }

    pub fn step(&mut self, gpu: &Gpu) {
        if self.window.is_open() && gpu.dirty {
            self.draw_buffer(gpu);
        }
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }

    fn draw_buffer(&mut self, gpu: &Gpu) {
        self.buffer.copy_from_slice(&gpu.buffer);

        self.window
            .update_with_buffer(&self.buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}