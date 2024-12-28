// camera.rs
use raylib::prelude::*;

pub struct CameraController {
    pub camera: Camera3D,
}

impl CameraController {
    pub fn new() -> Self {
        let camera = Camera3D::perspective (
            Vector3::new(0.0, 0.0, 5.0),
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 5.0),
            45.0
        );

        Self {
            camera
        }
    }

    pub fn update(&mut self) {
        
    }
}
