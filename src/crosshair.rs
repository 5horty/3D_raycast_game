use crate::frames::Screen;
use crate::player::Player;

pub fn draw_crosshair(screen: &mut Screen, player: &Player) {
    let half_width = (0.03 * screen.width as f32 / player.fov) as usize;
    let center_x = screen.width / 2;
    screen.draw_pixel(center_x + half_width, screen.height / 2, 0xFFFFFF);
    screen.draw_pixel(center_x - half_width, screen.height / 2, 0xFFFFFF);
}
