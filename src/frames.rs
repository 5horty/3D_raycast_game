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
        for i in 0..self.frame_buffer.len() / 2 {
            self.frame_buffer[i] = from_u8_rgb(0, 127, 255);
        }
        for i in self.frame_buffer.len() / 2..self.frame_buffer.len() {
            self.frame_buffer[i] = from_u8_rgb(0, 255, 127);
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
