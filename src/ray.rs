use crate::{frames::Screen, player::Player, world::World};

pub fn render(player: &Player, screen: &mut Screen, world: &World) {
    let screen_width = screen.width;
    let screen_height = screen.height;

    for column in 0..screen_width {
        let ray_angle =
            player.angle - player.fov / 2.0 + (column as f32 / screen_width as f32) * player.fov;
        let (distance, hit_door) = cast_ray(player, world, ray_angle);

        let distance = distance.max(0.1);
        let corrected_distance = distance * (ray_angle - player.angle).cos();
        let wall_height = (screen_height as f32 / corrected_distance) as isize;

        let y_start = ((screen_height as isize / 2) - (wall_height / 2)).max(0) as usize;
        let y_end =
            ((screen_height as isize / 2) + (wall_height / 2)).min(screen_height as isize) as usize;

        let colour = if hit_door { 0x8B4513 } else { 0x595858 };
        screen.draw_columns(column, y_start, y_end, colour);
    }
}

fn cast_ray(player: &Player, world: &World, angle: f32) -> (f32, bool) {
    let step_size = 0.005;
    let mut distance = 0.0;

    loop {
        let ray_x = player.x_cord + distance * angle.cos();
        let ray_y = player.y_cord + distance * angle.sin();

        let tile_x = ray_x as isize;
        let tile_y = ray_y as isize;

        if tile_x < 0 || tile_y < 0 {
            return (distance, false);
        }

        let tile_x = tile_x as usize;
        let tile_y = tile_y as usize;

        if tile_x >= world.width || tile_y >= world.height {
            return (distance, false);
        }

        let tile = world.tiles[tile_y * world.width + tile_x];

        if tile == 1 {
            return (distance, false);
        }

        if tile == 2 {
            if let Some(door) = world.doors.iter().find(|d| d.x == tile_x && d.y == tile_y) {
                // Slide: door opening means the ray passes through the top portion
                // We check if this ray hits the un-opened part of the door
                let frac_x = ray_x - ray_x.floor();
                let frac_y = ray_y - ray_y.floor();
                // Use whichever axis we're crossing
                let frac = if angle.cos().abs() > angle.sin().abs() {
                    frac_y
                } else {
                    frac_x
                };
                if frac > door.open_amount {
                    return (distance, true);
                }
            }
        }

        distance += step_size;
    }
}
