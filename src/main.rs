// main.rs
mod camera_controller;
mod input_handler;

use raylib::prelude::*;
use crate::camera_controller::CameraController;
use crate::input_handler::InputHandler;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1280, 1080)
        .title("DRIVE")
        .build();

    rl.set_target_fps(1000);

    let mut camera_controller: CameraController = CameraController::new();
    let mut input_handler: InputHandler = InputHandler::new();

    while !rl.window_should_close() {
        let mouse_pos = rl.get_mouse_position();
        input_handler.update(&rl, &mut camera_controller);
        let mut d: RaylibDrawHandle = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);

        {
            let mut d_3d = d.begin_mode3D(camera_controller.camera);
            d_3d.draw_grid(10, 1.0);
            d_3d.draw_cube(Vector3::new(0.0, 0.0, 0.0), 2.0, 2.0, 2.0, Color::RED);
            d_3d.draw_sphere(Vector3::new(0.0, 3.5, 0.0), 1.0, Color::BLUE);
        }

        //d.draw_text("DRIVE", 10, 40, 20, Color::BLACK);
        d.draw_text(&format!("Mouse Position: ({:.1}, {:.1})", mouse_pos.x, mouse_pos.y), 10, 40, 20, Color::BLACK);
        d.draw_fps(10, 10);
    }
}
