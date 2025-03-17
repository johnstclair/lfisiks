use minifb::{Key, MouseButton, MouseMode, Scale, ScaleMode, Window, WindowOptions};
use rand::Rng;
use std::{thread, time::Duration};

use lfisiks;

const WIDTH: usize = 30;
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

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut black = true;
        for i in buffer.iter_mut() {
            if black {
                *i = 0x00000000;
            } else {
                *i = 0x00FF00FF;
            }
            black = !black;
        }

        if let Some((x, y)) = window.get_mouse_pos(MouseMode::Discard) {
            if let Some(p) = lfisiks::point_to_buffer(x, y, WIDTH, HEIGHT) {
                buffer[p] = 0x00FFFFFF;
            }
        }

        if let Some((scroll_x, scroll_y)) = window.get_scroll_wheel() {
            println!("Scrolling {} - {}", scroll_x, scroll_y);
        }

        buffer[500] = 0x0000FF00;

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
