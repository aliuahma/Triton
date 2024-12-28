// main.rs
mod camera;

use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("DRIVE")
        .build();

    rl.set_target_fps(1000);

    let mut camera_controller: camera::CameraController = camera::CameraController::new();

    while !rl.window_should_close() {
        let mut d: RaylibDrawHandle<'_> = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);

        camera_controller.update();

        {
            let mut d_3d = d.begin_mode3D(camera_controller.camera);
            d_3d.draw_grid(10, 1.0);
            d_3d.draw_cube(Vector3::new(0.0, 0.0, 0.0), 2.0, 2.0, 2.0, Color::RED);
            d_3d.draw_sphere(Vector3::new(0.0, 3.5, 0.0), 1.0, Color::BLUE);
        }

        d.draw_text("DRIVE", 10, 40, 20, Color::BLACK);
        d.draw_fps(10, 10);
    }
}
