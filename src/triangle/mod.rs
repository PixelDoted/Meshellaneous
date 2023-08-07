mod intersect;
mod modifiers;

use std::ops::{Index, IndexMut};

use glam::{Vec2, Vec3};

use crate::polygon::Polygon;

#[derive(Clone, Copy, Debug)]
pub struct Triangle {
    pub points: [Vec3; 3],
    pub uvs: [Vec2; 3],
    pub normal: Vec3,
}

impl Triangle {
    pub fn new(points: [Vec3; 3], uvs: [Vec2; 3], normal: Vec3) -> Self {
        Self {
            points,
            uvs,
            normal,
        }
    }

    /// Gets the normal from points
    /// and create a Triangle
    pub fn from_points_uvs(points: [Vec3; 3], uvs: [Vec2; 3]) -> Self {
        let normal = (points[1] - points[0]).cross(points[2] - points[0]);
        Self::new(points, uvs, normal)
    }

    /// Gets the normal from points  
    /// uses ZERO for uvs
    /// and creates a Triangle
    pub fn from_points(points: [Vec3; 3]) -> Self {
        Self::from_points_uvs(points, [Vec2::ZERO; 3])
    }

    /// returns a point with the maximum x, y and z values
    pub fn max(&self) -> Vec3 {
        self[0].max(self[1]).max(self[2])
    }

    /// returns a point with the minimum x, y, and z values
    pub fn min(&self) -> Vec3 {
        self[0].min(self[1]).min(self[2])
    }

    /// Calculates the area of this triangle
    pub fn area(&self) -> f32 {
        // https://math.stackexchange.com/a/128995
        let ab = self[1] - self[0];
        let ac = self[2] - self[0];
        ab.dot(ac) * 0.5
    }
}

impl Index<usize> for Triangle {
    type Output = Vec3;

    /// returns a reference to the nth point
    ///
    /// ### Panic
    /// panics if the index is above 2
    fn index(&self, index: usize) -> &Self::Output {
        &self.points[index]
    }
}

impl IndexMut<usize> for Triangle {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.points[index]
    }
}

impl From<Polygon> for Vec<Triangle> {
    fn from(value: Polygon) -> Vec<Triangle> {
        let mut out = Vec::new();
        for i in 1..value.len() - 1 {
            let v0 = &value.vertices[0];
            let v1 = &value.vertices[i];
            let v2 = &value.vertices[i + 1];

            out.push(Triangle::new(
                [v0.point, v1.point, v2.point],
                [v0.uv, v1.uv, v2.uv],
                (v0.normal + v1.normal + v2.normal) / 3.0,
            ));
        }

        out
    }
}
