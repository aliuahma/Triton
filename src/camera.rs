// camera.rs
use raylib::prelude::*;

pub struct CameraController {
    pub camera: Camera3D,
    zoom_min: f32,
    zoom_max: f32,
    zoom_sensitivity: f32,
    phi: f32,
    phi_min: f32,
    phi_max: f32,
    theta: f32, 
    orbit_sensitivity: f32,
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
            zoom_sensitivity: 0.5,
            phi: 0.0,
            phi_min: -std::f32::consts::FRAC_PI_4 + 0.00001,
            phi_max: std::f32::consts::FRAC_PI_4 - 0.00001,
            theta: 0.0,
            orbit_sensitivity: 0.005,
        }
    }

    pub fn zoom(&mut self, delta: f32) {
        let d: Vector3 = self.camera.position - self.camera.target;   // displacement vector
        let r: Vector3 = d.normalized();                              // direction vector
        let mut l: f32 = d.length() + delta * self.zoom_sensitivity;  // new zoom distance
        l = l.clamp(self.zoom_min, self.zoom_max);                    // clamp zoom distance
        self.camera.position = self.camera.target + r * l;
    }

    pub fn orbit(&mut self, delta: Vector2) {
        let delta_theta: f32 = -delta.x * self.orbit_sensitivity;
        let delta_phi: f32 = delta.y * self.orbit_sensitivity;

        self.theta += delta_theta % (std::f32::consts::PI * 2.0);
        self.phi = (self.phi + delta_phi).clamp(self.phi_min, self.phi_max);

        let distance: f32 = (self.camera.position - self.camera.target).length();
        let x: f32 = distance * self.phi.cos() * self.theta.sin();
        let y: f32 = distance * self.phi.sin();
        let z: f32 = distance * self.phi.cos() * self.theta.cos();

        self.camera.position = self.camera.target + Vector3::new(x, y, z);

        self.camera.up = Vector3::new(0.0, 1.0, 0.0);
    }
}
