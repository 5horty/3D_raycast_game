use crate::{frames::Screen, player::Player, world::World};

pub fn render(player: &Player, screen: &mut Screen, world: &World) {
    let screen_width = screen.width;
    let screen_height = screen.height;

    for column in 0..screen_width {
        let ray_angle =
            player.angle - player.fov / 2.0 + (column as f32 / screen_width as f32) * player.fov;
        let distance = cast_ray(player, world, ray_angle);

        let distance = distance.max(0.1);

        let corrected_distance = distance * (ray_angle - player.angle).cos();

        let wall_height = (screen_height as f32 / corrected_distance) as isize;

        let mut y_start = (screen_height as isize / 2) - (wall_height / 2);

        let mut y_end = (screen_height as isize / 2) + (wall_height / 2);

        if y_start < 0 {
            y_start = 0;
        }
        if y_end > screen_height as isize {
            y_end = screen_height as isize;
        }

        screen.draw_columns(column, y_start as usize, y_end as usize, 0x595858);
    }
}

fn cast_ray(player: &Player, world: &World, angle: f32) -> f32 {
    let step_size = 0.05;
    let mut distance = 0.0;
    loop {
        let ray_x = player.x_cord + distance * angle.cos();
        let ray_y = player.y_cord + distance * angle.sin();

        let tile_x = ray_x as isize;
        let tile_y = ray_y as isize;

        if tile_x < 0 || tile_y < 0 {
            return distance;
        }
        let tile_x = tile_x as usize;
        let tile_y = tile_y as usize;

        if world.is_wall(tile_x, tile_y) {
            return distance;
        }

        distance += step_size;
    }
}
