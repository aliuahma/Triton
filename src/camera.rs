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
    pan_sensitivity: f32,
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
            pan_sensitivity: 0.01,
        }
    }

    // pub fn zoom(&mut self, delta: f32) {
    //     let d: Vector3 = self.camera.position - self.camera.target;   // displacement vector
    //     let r: Vector3 = d.normalized();                              // direction vector
    //     let mut l: f32 = d.length() + delta * self.zoom_sensitivity;  // new zoom distance
    //     l = l.clamp(self.zoom_min, self.zoom_max);                    // clamp zoom distance
    //     self.camera.position = self.camera.target + r * l;
    // }

    pub fn zoom(&mut self, delta: f32, mouse_pos: Vector2) {
        // Calculate the normalized device coordinates (NDC)
        let screen_width: f32 = 800.0;
        let screen_height: f32 = 600.0;

        let ndc_x: f32 = 2.0 * mouse_pos.x / screen_width - 1.0;
        let ndc_y: f32 = 1.0 - 2.0 * mouse_pos.y / screen_height;

        // Compute the ray direction in world space
        let fovy_radians: f32 = self.camera.fovy.to_radians();
        let aspect_ratio: f32 = screen_width / screen_height;
        let half_height: f32 = (fovy_radians / 2.0).tan();
        let half_width: f32 = aspect_ratio * half_height;

        let ray_dir_camera: Vector3 = Vector3::new(
            ndc_x * half_width,
            ndc_y * half_height,
            -1.0, // Points into the scene in camera space
        )
        .normalized();

        let camera_to_world: Matrix = Matrix::look_at(
            self.camera.position,
            self.camera.target,
            self.camera.up,
        )
        .inverted();

        // Transform the ray direction to world space
        let ray_dir_world: Vector3 = Vector3::new(
            ray_dir_camera.x * camera_to_world.m0 + ray_dir_camera.y * camera_to_world.m4 + ray_dir_camera.z * camera_to_world.m8,
            ray_dir_camera.x * camera_to_world.m1 + ray_dir_camera.y * camera_to_world.m5 + ray_dir_camera.z * camera_to_world.m9,
            ray_dir_camera.x * camera_to_world.m2 + ray_dir_camera.y * camera_to_world.m6 + ray_dir_camera.z * camera_to_world.m10,
        )
        .normalized();

        // Calculate the intersection point with the plane at target depth
        let plane_normal: Vector3 = (self.camera.target - self.camera.position).normalized();
        let plane_point: Vector3 = self.camera.target;

        let denom = ray_dir_world.dot(plane_normal);
        if denom.abs() < 1e-6 {
            // Ray is nearly parallel to the plane, no meaningful zoom
            return;
        }

        let t = (plane_point - self.camera.position).dot(plane_normal) / denom;
        if t < 0.0 {
            // Intersection is behind the camera, ignore
            return;
        }

        let hover_point = self.camera.position + ray_dir_world * t;

        // Move the camera toward or away from the hover point
        let to_hover = hover_point - self.camera.position;
        let zoom_distance = delta * self.zoom_sensitivity;

        let new_position = self.camera.position + to_hover * zoom_distance;
        let new_distance = (new_position - hover_point).length();

        if new_distance >= self.zoom_min && new_distance <= self.zoom_max {
            let to_target = self.camera.target - self.camera.position;
            self.camera.position = new_position;
            self.camera.target = new_position + to_target.normalized() * to_target.length();
        }
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

    pub fn pan(&mut self, delta: Vector2) {
        // right and up vectors of the camera, normalized
        let r_norm: Vector3 = (self.camera.target - self.camera.position).cross(self.camera.up).normalized();
        let u_norm: Vector3 = self.camera.up.normalized();

        let distance: f32 = (self.camera.target - self.camera.position).length();
        let viewport_height: f32 = 2.0 * distance * (self.camera.fovy.to_radians() / 2.0).tan();
        let scale_factor: f32 = distance / viewport_height;

        let delta_world: Vector3 = (r_norm * -delta.x + u_norm * delta.y) * scale_factor * self.pan_sensitivity;
        self.camera.position += delta_world;
        self.camera.target += delta_world;
    }
}
