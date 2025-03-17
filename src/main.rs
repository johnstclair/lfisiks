use minifb::{Key, MouseButton, MouseMode, Scale, ScaleMode, Window, WindowOptions};
// use std::{thread, time::Duration};

use lfisiks;

const WIDTH: usize = 10;
const HEIGHT: usize = 30;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: Scale::X32,
            scale_mode: ScaleMode::AspectRatioStretch,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.set_target_fps(60);

    let world = lfisiks::World::new(WIDTH, HEIGHT);
    world.id_lize();

    const SAND: u32 = 0x00ffc433;
    const EMPTY: u32 = 0x00000000;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if let Some((x, y)) = window.get_mouse_pos(MouseMode::Discard) {
            if let Some(p) = lfisiks::point_to_buffer(x, y, WIDTH, HEIGHT) {
                if window.get_mouse_down(MouseButton::Left) {
                    buffer[p] = SAND;
                }

                if window.get_mouse_down(MouseButton::Right) {
                    buffer[p] = EMPTY;
                }
            }
        }

        for x in 0..WIDTH + 32 {
            if let Some(p) = lfisiks::point_to_buffer(x as f32, x as f32, WIDTH, HEIGHT) {
                buffer[p] = 0x0000FF00;
            }
        }

        // let mut p: usize = WIDTH * HEIGHT;
        for i in buffer.iter_mut().rev() {
            // p -= 1;
            if *i == SAND {}
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
