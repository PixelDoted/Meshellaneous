mod modifiers;

use std::ops::{Index, IndexMut};

use glam::Vec3;

use crate::triangle::Triangle;

use super::IndexMesh;

/// A Mesh made up of triangles
#[derive(Clone, Default, Debug)]
pub struct TriMesh {
    pub triangles: Vec<Triangle>,
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
    pub fn tricount(&self) -> usize {
        self.triangles.len()
    }

    /// returns two points with the minimum and maximum x, y, and z values
    pub fn min_max(&self) -> [Vec3; 2] {
        let mut out = [self[0].min(), self[0].max()];
        for i in 1..self.tricount() {
            out[0] = out[0].min(self[i].min());
            out[1] = out[1].max(self[i].max());
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

            out.add(Triangle::new(
                [
                    value.vertices[index[0][0]],
                    value.vertices[index[1][0]],
                    value.vertices[index[2][0]],
                ],
                [
                    value.uvs[index[0][1]],
                    value.uvs[index[1][1]],
                    value.uvs[index[2][1]],
                ],
                value.normals[index[0][2]],
            ));
        }

        out
    }
}
