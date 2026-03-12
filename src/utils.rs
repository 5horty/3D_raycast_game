//conversts u8 to u8 for frame_buf in windows
pub fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}
use crate::enemies::Enemy;
pub fn update_enemies(enemies: &mut Vec<Enemy>, delta_time: f32) {
    for enemy in enemies {
        if enemy.sprite.damage_timer > 0.0 {
            enemy.sprite.damage_timer -= delta_time;
        }
    }
}
