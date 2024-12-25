use raylib::prelude::*;

fn main() {
    let (mut r1, thread) = raylib::init()
        .size(800, 600)
        .title("DRIVE")
        .build();

    while !r1.window_should_close() {
        let mut d = r1.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        d.draw_text("Hello, DRIVE!", 20, 20, 20, Color::BLACK);
    }
}
