// input_handler.rs
use raylib::prelude::*;
use crate::camera::CameraController;

pub struct InputHandler {
    previous_mouse_position: Vector2, 
    current_mouse_position: Vector2,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            previous_mouse_position: Vector2::zero(),
            current_mouse_position: Vector2::zero(),
        }
    }

    pub fn update(&mut self, rl: &RaylibHandle, camera_controller: &mut CameraController) {
        self.previous_mouse_position = self.current_mouse_position;
        self.current_mouse_position = rl.get_mouse_position();
        let mouse_delta: Vector2 = self.current_mouse_position - self.previous_mouse_position;

        camera_controller.zoom(rl.get_mouse_wheel_move());

        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_MIDDLE) && rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT){
            camera_controller.orbit(mouse_delta);
        }
    }
}