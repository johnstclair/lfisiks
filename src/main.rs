use minifb::{Key, MouseButton, MouseMode, Scale, ScaleMode, Window, WindowOptions};

use lfisiks::Id;

const WIDTH: usize = 200;
const HEIGHT: usize = 100;

fn main() {
    let mut buffer: Vec<u32>;

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: Scale::X8,
            scale_mode: ScaleMode::AspectRatioStretch,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.set_target_fps(120);

    let mut brush = lfisiks::Brush::new(2);

    let mut world = lfisiks::World::new(WIDTH, HEIGHT);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if let Some((x, y)) = window.get_mouse_pos(MouseMode::Discard) {
            if window.get_mouse_down(MouseButton::Left) {
                brush.draw((x as usize,y as usize), &mut world, false);
            }

            if window.get_mouse_down(MouseButton::Right) {
                brush.draw((x as usize,y as usize), &mut world, true);
            }
        }

        window
            .get_keys_pressed(minifb::KeyRepeat::No)
            .iter()
            .for_each(|key| match key {
                Key::W => brush.change_paint(lfisiks::Id::Water),
                Key::T => brush.change_paint(lfisiks::Id::Stone),
                Key::S => brush.change_paint(lfisiks::Id::Sand),
                _ => (),
            });

        buffer = world.buffer();
        world.update();

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
