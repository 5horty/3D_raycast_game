pub struct World {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<u8>,
}
impl World {
    /// just a map impl for testing
    pub fn defualt() -> Self {
        Self {
            width: 10,
            height: 10,
            tiles: vec![
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0,
                0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0,
                0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0,
                0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            ],
        }
    }
    fn get_tile(&self, x_cord: usize, y_cord: usize) -> u8 {
        self.tiles[y_cord * self.width + x_cord]
    }
    pub fn is_wall(&self, x_cord: usize, y_cord: usize) -> bool {
        self.get_tile(x_cord, y_cord) == 1
    }
}
