use crate::enemies::{self, Enemy};
use crate::player::{self, Player};
use crate::world::{self, World};

pub fn ray_hits_enemy(
    player: &Player,
    world: &World,
    enemies: &Enemy,
    max_distance: f32,
) -> Option<f32> {
    let dx = enemies.sprite.x_cords - player.x_cord;
    let dy = enemies.sprite.y_cords - player.y_cord;

    let angle_to_enemy = dy.atan2(dx);

    let mut relative_angle = angle_to_enemy - player.angle;

    todo!()
}
pub fn shoot(player: &Player, enemies: &mut Vec<Enemy>, damage: i32) {}
