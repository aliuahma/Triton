// input_handler.rs
use raylib::prelude::*;

pub struct InputHandler {
    previous_mouse_position: Vector2, 
    current_mouse_position: Vector2,
    mouse_wheel_move: f32,
    accumulated_scroll: f32,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            previous_mouse_position: Vector2::zero(),
            current_mouse_position: Vector2::zero(),
            mouse_wheel_move: 0.0,
            accumulated_scroll: 0.0,
        }
    }

    pub fn update(&mut self, rl: &RaylibHandle) {
        self.previous_mouse_position = self.current_mouse_position;
        self.current_mouse_position = rl.get_mouse_position();
        self.mouse_wheel_move = rl.get_mouse_wheel_move();

        self.accumulated_scroll += self.mouse_wheel_move;

        if self.mouse_wheel_move != 0.0 {

        }
    }

    pub fn scroll(&mut self) -> f32 {
        self.accumulated_scroll
    }
}