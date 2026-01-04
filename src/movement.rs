use std::time::Duration;

use crate::player::Player;
use crate::world;
use crate::world::World;
use minifb::Key;
use minifb::Window;

pub fn get_key(window: &Window, player: &mut Player, world: &World, delta_time: &Duration) {
    let move_speed = 5.0;
    let rotate_speed = 2.5;
    let movement = move_speed * delta_time.as_secs_f32();
    let rotate_angle = rotate_speed * delta_time.as_secs_f32();

    if window.is_key_down(Key::W) {
        player.move_forward(movement, &world);
    }
    if window.is_key_down(Key::S) {
        player.move_forward(-movement, &world);
    }
    if window.is_key_down(Key::D) {
        player.move_right(movement, &world);
    }
    if window.is_key_down(Key::A) {
        player.move_left(movement, &world);
    }
    if window.is_key_down(Key::H) {
        player.rotate(-rotate_angle);
    }
    if window.is_key_down(Key::L) {
        player.rotate(rotate_angle);
    }
}
