use crate::sprite::Sprite;
use crate::textures::basic_enemy_texture;

pub struct Enemy {
    pub sprite: Sprite,
    pub health: i32,
    pub hit_radius: f32,
}
impl Enemy {
    pub fn defualt() -> Self {
        Self {
            sprite: Sprite::new(8.0, 8.0, basic_enemy_texture(), 100, 100, 0.0),
            health: 100,
            hit_radius: 0.5,
        }
    }
    pub fn is_alive(&self) -> bool {
        self.health > 0
    }
    pub fn take_damage(&mut self, damage: i32) {
        self.health -= damage;
        println!("takes damege");
        self.sprite.damage_timer = 0.5;
        if !self.is_alive() {
            self.health = 0;
        }
    }
}
