use crate::sprite::Sprite;

pub struct Enemy {
    pub sprite: Sprite,
    pub health: i32,
}
impl Enemy {
    pub fn defualt() -> Self {
        Self {
            sprite: Sprite::defualt(),
            health: 100,
        }
    }
    pub fn is_alive(&self) -> bool {
        self.health > 0
    }
    pub fn take_damage(&mut self, damage: i32) {
        self.health -= damage;
        if !self.is_alive() {
            self.health = 0;
        }
    }
}
