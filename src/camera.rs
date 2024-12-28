// camera.rs
use raylib::prelude::*;

pub struct CameraController {
    pub camera: Camera3D,
    zoom_min: f32,
    zoom_max: f32,
    zoom_speed: f32,
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
            camera,
            zoom_min: 2.0,
            zoom_max: 20.0,
            zoom_speed: 0.5,
        }
    }

    pub fn zoom(&mut self, delta: f32) {
        let d: Vector3 = self.camera.position - self.camera.target;  // displacement vector
        let r: Vector3 = d.normalized();                             // direction vector
        let mut l: f32 = d.length() + delta * self.zoom_speed;       // new zoom distance
        l = l.min(self.zoom_max).max(self.zoom_min);                 // clamp zoom distance
        self.camera.position = self.camera.target + r * l;
    }
}
