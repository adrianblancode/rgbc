use crate::Memory;

pub const WIDTH: usize = 160;
pub const HEIGHT: usize = 144;
const TILE_COLORS: [u32; 4] = [0xFFFFFF, 0x000000, 0x000000, 0x000000];

#[derive(Debug)]
pub struct Gpu {
    pub buffer: [u32; WIDTH * HEIGHT],
    last_scanline: u8,
    pub dirty: bool,
}

struct Sprite {
    y: u8,
    x: u8,
    tile: Tile,
    flags: u8,
}

const TILE_SIDE: usize = 8;

type Tile = [[u8; TILE_SIDE]; TILE_SIDE];

impl Gpu {
    pub fn new() -> Gpu {
        Gpu { buffer: [0xFFFFFF; WIDTH * HEIGHT], last_scanline: 0, dirty: false }
    }

    fn read_tile(&self, mem: &Memory, tile_index: usize) -> Tile {
        let mut tile: Tile = [[0; TILE_SIDE]; TILE_SIDE];

        let addr: usize = 0x8000 + tile_index * 16 as usize;

        // For each tile row
        for i in 0..TILE_SIDE {
            // There are two bytes
            let byte1: u8 = mem.data[addr + i * 2];
            let byte2: u8 = mem.data[addr + i * 2 + 1];

            // For each pixel
            for p in 0..TILE_SIDE {
                // Consists of two bits from each byte
                let bit1 = (byte1 >> (7 - p)) & 1;
                let bit2 = (byte2 >> (7 - p)) & 1;

                // Final color is two bits color depth, as index to palette
                let c = bit2 << 1 | bit1;

                tile[i][p] = c
            }
        }
        tile
    }

    pub fn step(&mut self, mem: &Memory) {
        let scanline: u8 = mem.data[0xFF44];
        self.dirty = false;

        if self.last_scanline as usize == 0 && self.last_scanline != scanline{
            self.buffer.fill(0xFFFFFF);
        }

        if scanline as usize <= HEIGHT && self.last_scanline != scanline {
            self.draw_scanline2(mem, self.last_scanline);
            // self.draw_sprites(mem);
            // self.draw_tilemap(mem);
        }

        if scanline as usize == HEIGHT && self.last_scanline != scanline {
            self.dirty = true;
        }

        self.last_scanline = scanline;
    }

    fn draw_scanline(&mut self, mem: &Memory, scanline: u8) {
        // TODO lcd

        let screen_tile_cols: usize = WIDTH / TILE_SIDE;
        let screen_tile_rows: usize = HEIGHT / TILE_SIDE;
        let tilemap_side: usize = 256 / TILE_SIDE;

        let scroll_y: u8 = 0;// mem.data[0xFF42];

        let tilemap_scroll_y_offset: usize = (scroll_y as usize / TILE_SIDE) * tilemap_side;
        let tilemap_scroll_y_rem: usize = scroll_y as usize % TILE_SIDE;

        for tilemap_index in 0 .. (screen_tile_cols * screen_tile_rows) as usize {

            let tilemap_y_offset: usize = (tilemap_index / screen_tile_cols) * tilemap_side;
            let tilemap_x_offset: usize = tilemap_index % screen_tile_cols;
            let tile_index: usize = mem.data[0x9800 + tilemap_y_offset + tilemap_scroll_y_offset + tilemap_x_offset] as usize;

            let tile_pos_y: i16 = ((tilemap_index / screen_tile_cols as usize) * TILE_SIDE) as i16 - tilemap_scroll_y_rem as i16;
            let tile_pos_x: i16 = (tilemap_x_offset * TILE_SIDE) as i16;

            // if tile_pos_y > scanline as i16 || (tile_pos_y + TILE_SIDE as i16) < scanline as i16 { continue; }
            let tile: Tile = self.read_tile(mem, tile_index);

            for tile_pixel_y in 0..TILE_SIDE {
                for tile_pixel_x in 0..TILE_SIDE {
                    let y: i16 = tile_pos_y as i16 + tile_pixel_y as i16;
                    let x: i16 = tile_pos_x + tile_pixel_x as i16;

                    // if y != scanline as i16 || x < 0 || x as usize >= WIDTH { continue; }
                    if y < 0 || y >= HEIGHT as i16 || x < 0 || x as usize >= WIDTH { continue; }

                    let c = tile[tile_pixel_y][tile_pixel_x];
                    self.buffer[y as usize * WIDTH + x as usize] = TILE_COLORS[c as usize];
                }
            }
        }
    }

    fn draw_scanline2(&mut self, mem: &Memory, scanline: u8) {
        // TODO lcd

        let screen_tile_cols: usize = WIDTH / TILE_SIDE;
        let screen_tile_rows: usize = HEIGHT / TILE_SIDE;
        let tilemap_side: usize = 32;

        let scroll_y: u8 = mem.data[0xFF42];

        let tilemap_scroll_y_offset: usize = (scroll_y as usize / TILE_SIDE) * tilemap_side;
        let tilemap_scroll_y_rem: usize = scroll_y as usize % TILE_SIDE;

        println!("Scroll y: {scroll_y}");
        for tilemap_index in 0 .. (screen_tile_cols * screen_tile_rows) as usize {

            let tilemap_y_offset: usize = (tilemap_index / screen_tile_cols) * tilemap_side;
            let tilemap_x_offset: usize = tilemap_index % screen_tile_cols;
            let tile_index: usize = mem.data[0x9800 + tilemap_y_offset + tilemap_scroll_y_offset + tilemap_x_offset] as usize;

            let tile_pos_y: i16 = ((tilemap_index / screen_tile_cols as usize) * TILE_SIDE) as i16 - tilemap_scroll_y_rem as i16;
            let tile_pos_x: i16 = (tilemap_x_offset * TILE_SIDE) as i16;

            if tile_pos_y > scanline as i16 || (tile_pos_y + TILE_SIDE as i16) < scanline as i16 { continue; }
            let tile: Tile = self.read_tile(mem, tile_index);

            for tile_pixel_y in 0..TILE_SIDE {
                for tile_pixel_x in 0..TILE_SIDE {
                    let y: i16 = tile_pos_y as i16 + tile_pixel_y as i16;
                    let x: i16 = tile_pos_x + tile_pixel_x as i16;

                    if y != scanline as i16 || x < 0 || x as usize >= WIDTH { continue; }

                    let c = tile[tile_pixel_y][tile_pixel_x];
                    self.buffer[y as usize * WIDTH + x as usize] = TILE_COLORS[c as usize];
                }
            }
        }
    }

    fn draw_sprites(&mut self, mem: &Memory) {
        let mut addr: usize = 0xFE00;
        while addr <= 0xFE9F {
            let tile_y: i16 = mem.data[addr] as i16 - 16;
            let tile_x: i16 = mem.data[addr + 1] as i16 - 8;
            let tile_index: usize = mem.data[addr + 3] as usize;
            // let flags = mem.data[addr + 3];

            if tile_index == 0 {
                addr += 4;
                continue;
            }

            let tile = self.read_tile(mem, tile_index);

            for tile_pixel_y in 0..TILE_SIDE {
                for tile_pixel_x in 0..TILE_SIDE {
                    let y = tile_y + tile_pixel_y as i16;
                    let x = tile_x + tile_pixel_x as i16;
                    if y < 0 || x < 0 || y as usize >= HEIGHT || x as usize >= WIDTH { continue; };
                    let c = tile[tile_pixel_y][tile_pixel_x];
                    self.buffer[y as usize * WIDTH + x as usize] = TILE_COLORS[c as usize];
                }
            }

            addr += 4;
        }
    }

    fn draw_tilemap(&mut self, mem: &Memory) {
        // self.buffer.fill(0xFFFFFF);
        let bytes_per_tile = 16;

        let mut tile_index: usize = 0;
        while tile_index * bytes_per_tile < 0x800 {
            let buffer_y = ((tile_index * TILE_SIDE) / WIDTH) * TILE_SIDE;
            let buffer_x = (tile_index * TILE_SIDE) % WIDTH;

            let tile: Tile = self.read_tile(mem, tile_index);

            for pixel_y in 0..TILE_SIDE {
                for pixel_x in 0..TILE_SIDE {
                    let y = buffer_y + pixel_y;
                    let x = buffer_x + pixel_x;
                    let c = tile[pixel_y][pixel_x];
                    self.buffer[y * WIDTH + x] = TILE_COLORS[c as usize];
                }
            }

            tile_index += 1;
        }
    }
}