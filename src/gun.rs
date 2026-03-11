use crate::frames::Screen;

const T: u32 = 0x000000; // transparent
const D: u32 = 0x1A1A1A; // dark
const M: u32 = 0x3A3A3A; // mid
const L: u32 = 0x6A6A6A; // light
const H: u32 = 0x9A9A9A; // highlight
const F: u32 = 0xFFDD00; // muzzle flash
const R: u32 = 0xFF6600; // flash inner

pub struct Gun {
    pub shoot_timer: f32,
}

impl Gun {
    pub fn new() -> Self {
        Self { shoot_timer: 0.0 }
    }

    pub fn shoot(&mut self) {
        self.shoot_timer = 0.12;
    }

    pub fn update(&mut self, delta_time: f32) {
        if self.shoot_timer > 0.0 {
            self.shoot_timer -= delta_time;
        }
    }

    pub fn draw(&self, screen: &mut Screen) {
        // Scale gun to 60% of screen width 45% of screen height
        let gun_w = (screen.width as f32 * 0.6) as usize;
        let gun_h = (screen.height as f32 * 0.45) as usize;

        // Anchor bottom-right like Doom (gun sits to the right of center)
        let start_x = (screen.width as f32 * 0.38) as usize;
        let start_y = screen.height - gun_h;

        // 32x24 logical grid then scale up
        let grid_w = 32usize;
        let grid_h = 24usize;

        #[rustfmt::skip]
        let grid: [u32; 32 * 24] = [
            // Row 0-3: barrel tip top
            T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,
            T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,
            T,T,T,T,T,T,T,T,D,D,D,D,D,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,
            T,T,T,T,T,T,T,D,M,M,M,M,M,D,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,
            // Row 4-7: barrel body
            T,T,T,T,T,T,T,D,M,H,H,M,M,D,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,
            T,T,T,T,T,T,T,D,M,H,H,M,M,D,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,
            T,T,T,T,T,T,T,D,M,H,H,M,M,D,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,
            T,T,T,T,T,T,T,D,M,H,H,M,M,D,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,
            // Row 8-10: slide / top of gun body widens
            T,T,T,T,T,T,D,D,M,H,H,M,M,M,D,D,D,D,D,D,T,T,T,T,T,T,T,T,T,T,T,T,
            T,T,T,T,T,T,D,M,H,H,H,H,H,H,H,H,M,M,M,M,D,T,T,T,T,T,T,T,T,T,T,T,
            T,T,T,T,T,T,D,M,H,H,H,H,H,H,H,H,M,M,M,M,D,T,T,T,T,T,T,T,T,T,T,T,
            // Row 11-14: main gun body
            T,T,T,T,T,D,D,M,H,H,H,H,H,H,H,H,H,M,M,M,M,D,D,T,T,T,T,T,T,T,T,T,
            T,T,T,T,T,D,M,H,H,H,H,H,H,H,H,H,H,H,M,M,M,M,M,D,T,T,T,T,T,T,T,T,
            T,T,T,T,T,D,M,H,H,H,H,H,H,H,H,H,H,H,M,M,M,M,M,D,T,T,T,T,T,T,T,T,
            T,T,T,T,T,D,M,M,M,M,M,M,M,M,M,M,M,M,M,M,M,M,M,D,T,T,T,T,T,T,T,T,
            // Row 15-17: trigger guard
            T,T,T,T,D,D,M,M,M,M,M,M,M,M,M,M,M,M,M,M,M,M,D,D,T,T,T,T,T,T,T,T,
            T,T,T,T,D,M,H,M,M,M,M,M,M,M,M,M,M,M,M,M,M,M,H,M,D,T,T,T,T,T,T,T,
            T,T,T,T,D,M,M,D,L,D,M,M,M,M,M,M,M,M,M,D,L,D,M,M,D,T,T,T,T,T,T,T,
            // Row 18-20: grip starts
            T,T,T,D,M,M,M,L,L,L,D,M,M,M,M,M,M,M,D,L,L,L,M,M,M,D,T,T,T,T,T,T,
            T,T,T,D,M,H,M,L,L,L,L,D,M,M,M,M,M,D,L,L,L,L,M,H,M,D,T,T,T,T,T,T,
            T,T,T,D,M,M,D,L,L,L,L,L,D,M,M,M,D,L,L,L,L,L,D,M,M,D,T,T,T,T,T,T,
            // Row 21-23: bottom / hands
            T,T,D,M,M,M,D,L,L,L,L,L,L,D,D,D,L,L,L,L,L,L,D,M,M,M,D,T,T,T,T,T,
            T,D,M,H,M,M,D,L,L,L,L,L,L,L,L,L,L,L,L,L,L,L,D,M,M,H,M,D,T,T,T,T,
            D,M,M,M,M,M,D,L,L,L,L,L,L,L,L,L,L,L,L,L,L,L,D,M,M,M,M,M,D,T,T,T,
        ];

        for y in 0..gun_h {
            for x in 0..gun_w {
                // Map screen pixel back to grid cell
                let gx = x * grid_w / gun_w;
                let gy = y * grid_h / gun_h;
                let colour = grid[gy * grid_w + gx];
                if colour == T {
                    continue;
                }

                let px = start_x + x;
                let py = start_y + y;
                if px < screen.width && py < screen.height {
                    screen.draw_pixel(px, py, colour);
                }
            }
        }

        // Muzzle flash above barrel tip
        if self.shoot_timer > 0.0 {
            let flash_cx = start_x + (8 * gun_w / grid_w);
            let flash_cy = start_y + (1 * gun_h / grid_h);
            let flash_size = (gun_w / grid_w) * 3;

            for dy in 0..flash_size * 2 {
                for dx in 0..flash_size * 2 {
                    let px = flash_cx + dx;
                    let py = (flash_cy + dy).saturating_sub(flash_size);
                    let cx = flash_size as isize - dx as isize;
                    let cy = flash_size as isize - dy as isize;
                    let dist = ((cx * cx + cy * cy) as f32).sqrt();
                    let colour = if dist < flash_size as f32 * 0.4 {
                        R
                    } else if dist < flash_size as f32 * 0.9 {
                        F
                    } else {
                        continue;
                    };
                    if px < screen.width && py < screen.height {
                        screen.draw_pixel(px, py, colour);
                    }
                }
            }
        }
    }
}
