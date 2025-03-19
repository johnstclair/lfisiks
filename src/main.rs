use minifb::{Key, MouseButton, MouseMode, Scale, ScaleMode, Window, WindowOptions};
// use std::{thread, time::Duration};

use lfisiks::Id;

const WIDTH: usize = 40;
const HEIGHT: usize = 60;

pub enum Paint {
    Sand,
    Water,
    Stone,
}

fn main() {
    let mut buffer: Vec<u32>;

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: Scale::X16,
            scale_mode: ScaleMode::AspectRatioStretch,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.set_target_fps(30);

    let mut paint = Paint::Sand;

    let mut world = lfisiks::World::new(WIDTH, HEIGHT);
    world.change_pixel(0, Id::Sand);
    world.change_pixel(5, Id::Water);

    for x in 0..WIDTH + 32 {
        if let Some(p) = lfisiks::point_to_buffer((x, x + 5), WIDTH, HEIGHT) {
            world.change_pixel(p, Id::Stone);
        }
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if let Some((x, y)) = window.get_mouse_pos(MouseMode::Discard) {
            if let Some(p) = lfisiks::point_to_buffer((x as usize, y as usize), WIDTH, HEIGHT) {
                if window.get_mouse_down(MouseButton::Left) {
                    match paint {
                        Paint::Sand => world.change_pixel(p, Id::Sand),
                        Paint::Water => world.change_pixel(p, Id::Water),
                        Paint::Stone => world.change_pixel(p, Id::Stone),
                    }
                }

                if window.get_mouse_down(MouseButton::Right) {
                    world.change_pixel(p, Id::Empty);
                }
            }
        }

        // world.id_lize();
        window
            .get_keys_pressed(minifb::KeyRepeat::No)
            .iter()
            .for_each(|key| match key {
                Key::W => paint = Paint::Water,
                Key::T => paint = Paint::Stone,
                Key::S => paint = Paint::Sand,
                _ => (),
            });

        // if let Some(m) = window
        //     .get_keys_pressed(minifb::KeyRepeat::No)
        //     .iter()
        //     .for_each(|key| {
        //         match key {
        //             Key::W => return Some(Paint::Water),
        //             Key::T => return Some(Paint::Stone),
        //             Key::S => return Some(Paint::Sand),
        //             _ => (),
        //         };
        //         return None;
        //     })
        // {
        //     paint = m;
        // }

        buffer = world.buffer();
        world.update();

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
