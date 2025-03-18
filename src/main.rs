use minifb::{Key, MouseButton, MouseMode, Scale, ScaleMode, Window, WindowOptions};
// use std::{thread, time::Duration};

use lfisiks::Id;

const WIDTH: usize = 10;
const HEIGHT: usize = 8;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

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

    // Limit to max ~60 fps update rate
    window.set_target_fps(10);

    let mut world = lfisiks::World::new(WIDTH, HEIGHT);
    world.change_pixel(0, Id::Sand);
    world.change_pixel(5, Id::Sand);

    for x in 0..WIDTH + 32 {
        if let Some(p) = lfisiks::point_to_buffer((x, x + 5), WIDTH, HEIGHT) {
            world.change_pixel(p, Id::Stone);
        }
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if let Some((x, y)) = window.get_mouse_pos(MouseMode::Discard) {
            if let Some(p) = lfisiks::point_to_buffer((x as usize, y as usize), WIDTH, HEIGHT) {
                if window.get_mouse_down(MouseButton::Left) {
                    world.change_pixel(p, Id::Sand);
                    buffer[p] = 0x00ff00ff;
                }

                if window.get_mouse_down(MouseButton::Right) {
                    world.change_pixel(p, Id::Empty);
                }
            }
        }

        if let Some(p) = lfisiks::point_to_buffer((6, 3), WIDTH, HEIGHT) {
            buffer[p] = 0x00FFFFFF;
        }

        buffer = world.buffer();
        world.update();

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
