use crate::world::World;

pub struct Player {
    pub x_cord: f32,
    pub y_cord: f32,
    pub angle: f32,
    pub fov: f32,
}
impl Player {
    pub fn defualt() -> Self {
        Self {
            x_cord: 3.0,
            y_cord: 3.0,
            angle: 0.0,
            fov: std::f32::consts::FRAC_PI_3,
        }
    }
    pub fn move_forward(&mut self, distance: f32, world: &World) {
        let new_x_cord = self.x_cord + distance * self.angle.cos();
        let new_y_cord = self.y_cord + distance * self.angle.sin();

        if !world.is_wall(new_x_cord as usize, new_y_cord as usize) {
            self.x_cord = new_x_cord;
            self.y_cord = new_y_cord;
        }
    }
    pub fn rotate(&mut self, delta_angle: f32) {
        self.angle += delta_angle;
    }
}
