// particle.rs
use raylib::prelude::*;

pub struct Particle {
    pub position: Vector3,
    pub velocity: Vector3,
    pub density: f32,
    pub pressure: f32,
    pub color: Color,
    pub radius: f32,
}

impl Particle {
    pub fn new(position: Vector3, velocity: Vector3, color: Color, radius: f32) -> Self {
        Self {
            position,
            velocity,
            color,
            radius,
        }
    }

    pub fn update(&mut self) {
        self.position += self.velocity;
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_sphere(self.position, self.radius, self.color);
    }
}