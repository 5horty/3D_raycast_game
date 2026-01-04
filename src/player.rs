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
    pub fn check_coords(&mut self, world: &World, new_x_coords: f32, new_y_coords: f32) {
        if !world.is_wall(new_x_coords as usize, new_y_coords as usize) {
            self.x_cord = new_x_coords;
            self.y_cord = new_y_coords;
        }
    }

    pub fn move_forward(&mut self, distance: f32, world: &World) {
        let new_x_cord = self.x_cord + distance * self.angle.cos();
        let new_y_cord = self.y_cord + distance * self.angle.sin();

        self.check_coords(world, new_x_cord, new_y_cord);
    }
    pub fn move_right(&mut self, distance: f32, world: &World) {
        let new_x_cord = self.x_cord + distance * -self.angle.sin();
        let new_y_cord = self.y_cord + distance * self.angle.cos();

        self.check_coords(world, new_x_cord, new_y_cord);
    }
    pub fn move_left(&mut self, distance: f32, world: &World) {
        let new_x_cord = self.x_cord + distance * self.angle.sin();
        let new_y_cord = self.y_cord + distance * -self.angle.cos();

        self.check_coords(world, new_x_cord, new_y_cord);
    }
    pub fn rotate(&mut self, delta_angle: f32) {
        self.angle += delta_angle;
    }
}
