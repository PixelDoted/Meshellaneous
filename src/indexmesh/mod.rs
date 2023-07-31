mod modifiers;

use glam::Vec3;

use crate::trimesh::TriMesh;

/// TODO: Documentation
#[derive(Clone, Default, Debug)]
pub struct IndexMesh {
    pub vertices: Vec<Vec3>,
    pub indices: Vec<usize>,
}

impl IndexMesh {
    pub fn new(vertices: Vec<Vec3>, indices: Vec<usize>) -> Self {
        Self { vertices, indices }
    }

    /// Adds a vertex to this mesh
    /// unless the vertex already exsists
    ///
    /// then returns it's index
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

    /// Adds three indices to this mesh
    pub fn add_triangle(&mut self, i0: usize, i1: usize, i2: usize) {
        self.indices.push(i0);
        self.indices.push(i1);
        self.indices.push(i2);
    }

    /// returns a point with the maximum x, y and z values
    pub fn max(&self) -> Vec3 {
        let mut out = self.vertices[0];
        for i in 1..self.vertices.len() {
            out = out.max(self.vertices[i]);
        }

        out
    }

    /// returns a point with the minimum x, y, and z values
    pub fn min(&self) -> Vec3 {
        let mut out = self.vertices[0];
        for i in 1..self.vertices.len() {
            out = out.min(self.vertices[i]);
        }

        out
    }
}

impl From<TriMesh> for IndexMesh {
    /// Takes this mesh and returns a `TriMesh`
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
