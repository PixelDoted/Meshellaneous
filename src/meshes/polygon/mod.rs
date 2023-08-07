use std::ops::{Index, IndexMut};

use glam::Vec3;

use crate::polygon::Polygon;

/// A Mesh made up of triangles
#[derive(Clone, Default, Debug)]
pub struct PolyMesh {
    pub polygons: Vec<Polygon>,
}

impl PolyMesh {
    pub fn new(polygons: Vec<Polygon>) -> Self {
        Self { polygons }
    }

    /// adds a triangle to the mesh
    pub fn add(&mut self, polygon: Polygon) {
        self.polygons.push(polygon);
    }

    /// removes a triangle at index from the mesh
    pub fn remove(&mut self, index: usize) -> Polygon {
        self.polygons.remove(index)
    }

    /// returns the amount of triangles this mesh has
    pub fn polycount(&self) -> usize {
        self.polygons.len()
    }

    /// returns a point with the maximum x, y and z values
    pub fn max(&self) -> Vec3 {
        let mut out = self[0].max();
        for i in 1..self.polycount() {
            out = out.max(self[i].max());
        }

        out
    }

    /// returns a point with the minimum x, y, and z values
    pub fn min(&self) -> Vec3 {
        let mut out = self[0].min();
        for i in 1..self.polycount() {
            out = out.min(self[i].min());
        }

        out
    }
}

impl Index<usize> for PolyMesh {
    type Output = Polygon;

    /// returns an index to the nth triangle
    fn index(&self, index: usize) -> &Self::Output {
        &self.polygons[index]
    }
}

impl IndexMut<usize> for PolyMesh {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.polygons[index]
    }
}
