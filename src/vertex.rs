use glam::{Vec2, Vec3};

#[derive(Clone, Copy, Debug, Default)]
pub struct Vertex {
    pub point: Vec3,
    pub uv: Vec2,
    pub normal: Vec3,
}

impl Vertex {
    pub fn new(point: Vec3, uv: Vec2, normal: Vec3) -> Self {
        Self { point, uv, normal }
    }
}
