use std::ops::{Index, IndexMut};

use glam::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Triangle {
    points: [Vec3; 3],
    normal: Vec3,
}

impl Triangle {
    pub fn new(points: [Vec3; 3], normal: Vec3) -> Self {
        Self { points, normal }
    }

    /// gets the normal
    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    /// sets the normal
    pub fn set_normal(&mut self, normal: Vec3) {
        self.normal = normal;
    }

    /// returns a point with the maximum x, y and z values
    pub fn max(&self) -> Vec3 {
        self[0].max(self[1]).max(self[2])
    }

    /// returns a point with the minimum x, y, and z values
    pub fn min(&self) -> Vec3 {
        self[0].min(self[1]).min(self[2])
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
