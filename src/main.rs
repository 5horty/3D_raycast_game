mod frames;
mod movement;
mod player;
mod ray;
mod sprite;
mod utils;
mod world;

use crate::frames::Screen;
use crate::player::Player;
use crate::ray::render;
use crate::sprite::Sprite;
use crate::world::World;
use minifb::{Window, WindowOptions};
use std::thread::sleep;
use std::time::{Duration, Instant};

fn main() {
    // screen and window
    let mut screen = Screen::defualt();
    let mut window = match Window::new(
        "doom",
        screen.width,
        screen.height,
        WindowOptions::default(),
    ) {
        Ok(win) => win,
        Err(_) => {
            println!("Failed to open window");
            return;
        }
    };

    // fps cap (30 fps)
    let target_frame_time = Duration::from_micros(33_333);

    // world and player
    let world = World::defualt();
    let mut player = Player::defualt();
    let sprite = Sprite::defualt();

    // main loop
    while window.is_open() {
        let frame_start = Instant::now();

        // handle input, move player
        let delta_time = target_frame_time.as_secs_f32();
        movement::get_key(
            &window,
            &mut player,
            &world,
            &Duration::from_secs_f32(delta_time),
        );

        // clear screen
        screen.clear();

        // render the scene from players pov
        render(&player, &mut screen, &world);

        //draw sprite
        sprite.sprite_projection(&player, &mut screen);

        // display buffer
        window
            .update_with_buffer(&screen.frame_buffer, screen.width, screen.height)
            .expect("Failed to update buffer");

        // frame limiter
        let frame_duration = frame_start.elapsed();
        if frame_duration < target_frame_time {
            sleep(target_frame_time - frame_duration);
        }
    }
}
