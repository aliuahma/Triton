// camera.rs
use raylib::prelude::*;

pub struct CameraController {
    pub camera: Camera3D,
    movement_speed: f32,
    rotation_speed: f32
}

