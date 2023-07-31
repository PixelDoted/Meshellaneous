mod modifiers;

use std::ops::{Index, IndexMut};

use glam::Vec3;

use crate::{indexmesh::IndexMesh, triangle::Triangle};

/// A Mesh made up of triangles
#[derive(Clone, Default, Debug)]
pub struct TriMesh {
    triangles: Vec<Triangle>,
}

impl TriMesh {
    pub fn new(triangles: Vec<Triangle>) -> Self {
        Self { triangles }
    }

    /// adds a triangle to the mesh
    pub fn add(&mut self, triangle: Triangle) {
        self.triangles.push(triangle);
    }

    /// removes a triangle at index from the mesh
    pub fn remove(&mut self, index: usize) -> Triangle {
        self.triangles.remove(index)
    }

    /// returns the amount of triangles this mesh has
    pub fn triangle_count(&self) -> usize {
        self.triangles.len()
    }

    /// returns an iterator over the triangles
    pub fn iter(&self) -> std::slice::Iter<Triangle> {
        self.triangles.iter()
    }

    /// returns a mutable iterator over the triangles
    pub fn iter_mut(&mut self) -> std::slice::IterMut<Triangle> {
        self.triangles.iter_mut()
    }

    /// returns a point with the maximum x, y and z values
    pub fn max(&self) -> Vec3 {
        let mut out = self[0].max();
        for i in 1..self.triangle_count() {
            out = out.max(self[i].max());
        }

        out
    }

    /// returns a point with the minimum x, y, and z values
    pub fn min(&self) -> Vec3 {
        let mut out = self[0].min();
        for i in 1..self.triangle_count() {
            out = out.min(self[i].min());
        }

        out
    }

    /// Calculates the surface area of this mesh
    pub fn surface_area(&self) -> f32 {
        let mut sum = 0.0;
        for t in &self.triangles {
            let corner = t[0];
            let a = t[1] - corner;
            let b = t[2] - corner;

            sum += a.cross(b).length();
        }

        sum
    }
}

impl Index<usize> for TriMesh {
    type Output = Triangle;

    /// returns an index to the nth triangle
    fn index(&self, index: usize) -> &Self::Output {
        &self.triangles[index]
    }
}

impl IndexMut<usize> for TriMesh {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.triangles[index]
    }
}

impl From<IndexMesh> for TriMesh {
    fn from(value: IndexMesh) -> Self {
        let mut out = Self::default();
        for i in 0..value.indices.len() / 3 {
            let i = i * 3;
            let index = [value.indices[i], value.indices[i + 1], value.indices[i + 2]];

            out.add(Triangle::from_points([
                value.vertices[index[0]],
                value.vertices[index[1]],
                value.vertices[index[2]],
            ]));
        }

        out
    }
}
