// main.rs
mod camera;

use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("DRIVE")
        .build();

    rl.set_target_fps(120);

    while !rl.window_should_close() {
        let mut d: RaylibDrawHandle<'_> = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);

        d.draw_text("MMB to pan | Shift + MMB to rotate", 10, 40, 20, Color::BLACK);
        d.draw_fps(10, 10);
    }
}
