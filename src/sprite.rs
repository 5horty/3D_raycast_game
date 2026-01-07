use crate::{frames::Screen, player::Player};
pub struct Sprite {
    pub x_cords: f32,
    pub y_cords: f32,
    pub texture: Vec<u32>,
    width: usize,
    height: usize,
    pub damage_timer: f32,
}

impl Sprite {
    pub fn defualt() -> Self {
        Self {
            x_cords: 5.0,
            y_cords: 5.0,
            texture: vec![0xFFFFFF; 100 * 100], //green white,
            width: 100,
            height: 100,
            damage_timer: 0.0,
        }
    }
    pub fn new(
        x_cords: f32,
        y_cords: f32,
        texture: Vec<u32>,
        width: usize,
        height: usize,
        damage_timer: f32,
    ) -> Self {
        Self {
            x_cords,
            y_cords,
            texture,
            width,
            height,
            damage_timer,
        }
    }
    pub fn sprite_projection(&self, player: &Player, screen: &mut Screen) {
        let dx = self.x_cords - player.x_cord;
        let dy = self.y_cords - player.y_cord;

        let angle_to_sprite = dy.atan2(dx);
        let mut relative_angle = angle_to_sprite - player.angle;

        // normalize to [-π, π]
        while relative_angle < -std::f32::consts::PI {
            relative_angle += 2.0 * std::f32::consts::PI;
        }
        while relative_angle > std::f32::consts::PI {
            relative_angle -= 2.0 * std::f32::consts::PI;
        }

        if relative_angle.abs() > player.fov / 2.0 {
            return; // outside FOV
        }

        let distance = (dx * dx + dy * dy).sqrt().max(0.1); // avoid divide by zero

        let screen_x = ((relative_angle / player.fov) + 0.5) * screen.width as f32;

        let sprite_height_on_screen = (screen.height as f32 / distance) as usize;
        let sprite_width_on_screen = sprite_height_on_screen;

        let colour = if self.damage_timer > 0.0 {
            0xFF0000
        } else {
            0x00000
        };
        self.draw_sprite(
            screen,
            screen_x,
            sprite_width_on_screen,
            sprite_height_on_screen,
            colour,
        );
    }
    pub fn draw_sprite(
        &self,
        screen: &mut Screen,
        screen_x: f32,
        width_on_screen: usize,
        height_on_screen: usize,
        colour: u32,
    ) {
        let start_x = screen_x as isize - (width_on_screen as isize / 2);
        let start_y = screen.height as isize / 2 - (height_on_screen as isize / 2);

        for y in 0..height_on_screen {
            let pixel_y = start_y + y as isize;
            if pixel_y < 0 || pixel_y >= screen.height as isize {
                continue;
            }

            for x in 0..width_on_screen {
                let pixel_x = start_x + x as isize;
                if pixel_x < 0 || pixel_x >= screen.width as isize {
                    continue;
                }

                let tex_x = x * self.width / width_on_screen;
                let tex_y = y * self.height / height_on_screen;
                let tex_index = tex_y * self.width + tex_x;

                let colour = self.texture[tex_index];
                screen.draw_pixel(pixel_x as usize, pixel_y as usize, colour);
            }
        }
    }
}
