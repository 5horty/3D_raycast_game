use crate::enemies::Enemy;
use crate::player::Player;

pub fn ray_hits_enemy(player: &Player, enemies: &[Enemy], max_distance: f32) -> Option<usize> {
    let mut closet_enemy: Option<usize> = None;
    let mut closet_distance = f32::INFINITY;
    for (i, eneme) in enemies.iter().enumerate() {
        let dx = eneme.sprite.x_cords - player.x_cord;
        let dy = eneme.sprite.y_cords - player.y_cord;

        let distance = (dx * dx + dy * dy).sqrt().max(0.1); // avoid divide by zero

        if distance > max_distance {
            continue;
        }

        let angle_to_enemy = dy.atan2(dx);

        let mut relative_angle = angle_to_enemy - player.angle;

        while relative_angle < -std::f32::consts::PI {
            relative_angle += 2.0 * std::f32::consts::PI;
        }
        while relative_angle > std::f32::consts::PI {
            relative_angle -= 2.0 * std::f32::consts::PI;
        }
        if relative_angle.abs() > (eneme.hit_radius / distance) {
            continue;
        }
        if distance < closet_distance {
            closet_distance = distance;
            closet_enemy = Some(i);
        }
    }

    closet_enemy
}
pub fn shoot(player: &Player, enemies: &mut Vec<Enemy>, damage: i32) {
    if let Some(enemy_index) = ray_hits_enemy(player, enemies, 5.0) {
        let enemy = &mut enemies[enemy_index];
        println!("shoots");
        enemy.take_damage(damage);

        if !enemy.is_alive() {
            println!("enemies dead");
        }
    }
}
pub fn remove_dead_enemies(enemies: &mut Vec<Enemy>) {
    enemies.retain(|e| e.is_alive());
}
