use glam::Vec3;

use crate::trimesh::TriMesh;

/// TODO: Documentation
#[derive(Clone, Default, Debug)]
pub struct IndexMesh {
    vertices: Vec<Vec3>,
    indices: Vec<usize>,
}

impl IndexMesh {
    pub fn new(vertices: Vec<Vec3>, indices: Vec<usize>) -> Self {
        Self { vertices, indices }
    }

    /// TODO: Documentation
    pub fn add_vertex(&mut self, vertex: Vec3) -> usize {
        for i in 0..self.vertices.len() {
            let v = &self.vertices[i];
            if v != &vertex {
                continue;
            }

            return i;
        }

        self.vertices.push(vertex);
        self.vertices.len() - 1
    }

    /// TODO: Documentation
    pub fn add_triangle(&mut self, i0: usize, i1: usize, i2: usize) {
        self.indices.push(i0);
        self.indices.push(i1);
        self.indices.push(i2);
    }

    /// TODO: Documentation
    pub fn index_count(&self) -> usize {
        self.indices.len()
    }

    /// TODO: Documentation
    pub fn index(&self, index: usize) -> usize {
        self.indices[index]
    }

    /// TODO: Documentation
    pub fn vertex(&self, index: usize) -> Vec3 {
        self.vertices[index]
    }

    /// returns a point with the maximum x, y and z values
    pub fn max(&self) -> Vec3 {
        let mut out = self.vertex(0);
        for i in 1..self.vertices.len() {
            out = out.max(self.vertex(i));
        }

        out
    }

    /// returns a point with the minimum x, y, and z values
    pub fn min(&self) -> Vec3 {
        let mut out = self.vertex(0);
        for i in 1..self.vertices.len() {
            out = out.min(self.vertex(i));
        }

        out
    }
}

impl From<TriMesh> for IndexMesh {
    fn from(value: TriMesh) -> Self {
        let mut out = Self::default();
        for i in 0..value.triangle_count() {
            let tri = value[i];
            let i0 = out.add_vertex(tri[0]);
            let i1 = out.add_vertex(tri[1]);
            let i2 = out.add_vertex(tri[2]);
            out.add_triangle(i0, i1, i2);
        }

        out
    }
}
