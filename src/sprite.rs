use crate::{frames::Screen, player::Player};
pub struct Sprite {
    x_cords: f32,
    y_cords: f32,
    texture: Vec<u32>,
    width: usize,
    height: usize,
}

impl Sprite {
    pub fn defualt() -> Self {
        Self {
            x_cords: 5.0,
            y_cords: 5.0,
            texture: vec![0x00FF00; 50 * 50], //green blob,
            width: 50,
            height: 50,
        }
    }
    pub fn sprite_projection(&mut self, player: &Player, screen: &mut Screen) {
        let dx = self.x_cords - player.x_cord;
        let dy = self.y_cords - player.y_cord;
        //angle to sprte
        let angle_to_sprite = dy.atan2(dx);

        //distance to player
        let distance = (dx * dx + dy * dy).sqrt();

        let relative_angle = angle_to_sprite - player.angle;

        let screen_x = ((relative_angle / player.fov) + 0.5) * screen.width as f32;

        //distance scalling
        let sprite_height_on_screen = (screen.height as f32 / distance) as usize;
        let sprite_width_on_screen = sprite_height_on_screen; // its a square
        self.draw_sprite(screen, screen_x);
    }

    pub fn draw_sprite(&self, screen: &mut Screen, screen_x: f32) {
        let start_x = screen_x as isize - (self.width as isize / 2);
        let start_y = screen.height as isize / 2 - (self.height as isize / 2);

        for y in 0..self.height {
            for x in 0..self.width {
                let tex_index = y * self.width + x;
                let colour = self.texture[tex_index];
                screen.draw_pixel(
                    (start_x + x as isize) as usize,
                    (start_y + y as isize) as usize,
                    colour,
                );
            }
        }
    }
}
