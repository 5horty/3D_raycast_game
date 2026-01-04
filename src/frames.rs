use crate::utils::from_u8_rgb;

pub struct Screen {
    pub width: usize,
    pub height: usize,
    pub frame_buffer: Vec<u32>,
}

impl Screen {
    pub fn defualt() -> Self {
        Self {
            width: 1000,
            height: 1000,
            frame_buffer: vec![0; 1000 * 1000],
        }
    }
    pub fn clear(&mut self) {
        for y in 0..self.height {
            let colour = if y < self.height / 2 {
                from_u8_rgb(0, 127, 255) // sky
            } else {
                from_u8_rgb(0, 255, 127) // ground
            };
            for x in 0..self.width {
                self.frame_buffer[y * self.width + x] = colour;
            }
        }
    }

    pub fn draw_pixel(&mut self, x_cord: usize, y_cord: usize, colour: u32) {
        if x_cord >= self.width || y_cord >= self.height {
            println!("out of bounds:frames.rs");
            return;
        }
        let index = y_cord * self.width + x_cord;
        self.frame_buffer[index] = colour;
    }
    pub fn draw_columns(&mut self, x_cord: usize, y_start: usize, y_end: usize, colour: u32) {
        for y in y_start..y_end {
            self.draw_pixel(x_cord, y, colour);
        }
    }
}
