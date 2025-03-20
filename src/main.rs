use minifb::{Key, MouseButton, MouseMode, Scale, ScaleMode, Window, WindowOptions};

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

    window.set_target_fps(60);

    let mut paint = Paint::Sand;

    let mut world = lfisiks::World::new(WIDTH, HEIGHT);

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

        window
            .get_keys_pressed(minifb::KeyRepeat::No)
            .iter()
            .for_each(|key| match key {
                Key::W => paint = Paint::Water,
                Key::T => paint = Paint::Stone,
                Key::S => paint = Paint::Sand,
                _ => (),
            });

        buffer = world.buffer();
        world.update();

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
