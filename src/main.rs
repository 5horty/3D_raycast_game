mod crosshair;
mod enemies;
mod frames;
mod gun;
mod hitscan;
mod movement;
mod player;
mod ray;
mod sprite;
mod textures;
mod utils;
mod world;

use crate::crosshair::draw_crosshair;
use crate::enemies::Enemy;
use crate::frames::Screen;
use crate::gun::Gun;
use crate::hitscan::{remove_dead_enemies, shoot};
use crate::player::Player;
use crate::ray::render;
use crate::sprite::Sprite;
use crate::utils::update_enemies;
use crate::world::World;
use minifb::{Key, KeyRepeat, Window, WindowOptions};
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
    let mut world = World::defualt();
    let mut player = Player::defualt();
    let sprite = Sprite::defualt();
    //enemies
    let mut enemy = vec![Enemy::defualt()];

    //gun
    let mut gun = Gun::new();

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
        if window.is_key_pressed(Key::Space, KeyRepeat::No) {
            shoot(&player, &mut enemy, 10);
            gun.shoot();
        }

        if window.is_key_pressed(Key::E, KeyRepeat::No) {
            world.interact(player.x_cord, player.y_cord, player.angle);
        }

        world.update_doors(delta_time);

        // clear screen
        screen.clear();

        // render the scene from players pov
        render(&player, &mut screen, &world);

        //draw sprite
        sprite.sprite_projection(&player, &mut screen);
        for i in enemy.iter() {
            i.sprite.sprite_projection(&player, &mut screen);
        }
        update_enemies(&mut enemy, delta_time);
        remove_dead_enemies(&mut enemy);
        draw_crosshair(&mut screen, &player);
        gun.update(delta_time);
        gun.draw(&mut screen);

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
