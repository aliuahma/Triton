use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("DRIVE")
        .build();

    rl.set_target_fps(120);

    // Define a 3D camera
    let mut camera = Camera3D {
        position: Vector3::new(4.0, 4.0, 4.0),  // Camera position in 3D space
        target: Vector3::new(0.0, 0.0, 0.0),    // Point the camera looks at
        up: Vector3::new(0.0, 1.0, 0.0),        // Camera's up [unit] vector
        fovy: 45.0,                             // Field of view in degrees
        projection: CameraProjection::CAMERA_PERSPECTIVE
    };

    let mut last_mouse_position = rl.get_mouse_position();

    // Main game loop
    while !rl.window_should_close() {
        // Handle camera movement
        let current_mouse_position = rl.get_mouse_position();
        let mouse_delta = current_mouse_position - last_mouse_position;
        last_mouse_position = current_mouse_position;

        // Pan with middle mouse button, rotate when shift is pressed as well
        if rl.is_mouse_button_down(MouseButton::MOUSE_MIDDLE_BUTTON) {
            if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) {
                let rotation_speed = 0.005;

                // Calculate horizontal rotation (yaw)
                let yaw = -mouse_delta.x * rotation_speed;
                let rotation_axis_y = Vector3::new(0.0, 1.0, 0.0);
                let direction = camera.target - camera.position;
                let rotated_direction_yaw = Vector3::rotate_by_axis_angle(direction, rotation_axis_y, yaw);

                // Calculate vertical rotation (pitch)
                let pitch = -mouse_delta.y * rotation_speed;
                let rotation_axis_pitch = Vector3::normalize(Vector3::cross(direction, camera.up));
                let rotated_direction_pitch = Vector3::rotate_by_axis_angle(rotated_direction_yaw, rotation_axis_pitch, pitch);

                // Update target based on the new direction
                camera.target = camera.position + rotated_direction_pitch;
            } else {
                // Calculate new camera center
                let pan_speed = 0.01;
                let right = Vector3::normalize(Vector3::cross(camera.target - camera.position, camera.up));
                let up = camera.up;

                camera.position += right * -mouse_delta.x * pan_speed;
                camera.target += right * -mouse_delta.x * pan_speed;
                camera.position += up * mouse_delta.y * pan_speed;
                camera.target += up * mouse_delta.y * pan_speed;
            }

            // Begin drawing
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::RAYWHITE);

            {
                let mut d_3d = d.begin_mode3d(&camera);

                // Draw a grid for reference
                d_3d.draw_grid(10, 1.0);

                // Draw a cube at the origin
                d_3d.draw_cube(Vector3::new(0.0, 1.0, 0.0), 2.0, 2.0, 2.0, Color::RED);

                // Draw a sphere above the cube
                d_3d.draw_sphere(Vector3::new(0.0, 3.5, 0.0), 1.0, Color::BLUE);
            }

            d.draw_text("MMB to pan | Shift + MMB to rotate", 10, 10, 20, Color::BLACK);
        }

    }
}
