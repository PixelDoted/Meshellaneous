mod modifiers;

use glam::{Vec2, Vec3};

use super::TriMesh;

/// TODO: Documentation
/// Currently assumes indices are in triangles (0, 1, 2)
#[derive(Clone, Default, Debug)]
pub struct IndexMesh {
    pub vertices: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    pub uvs: Vec<Vec2>,
    pub indices: Vec<[usize; 3]>,
}

impl IndexMesh {
    pub fn new(
        vertices: Vec<Vec3>,
        normals: Vec<Vec3>,
        uvs: Vec<Vec2>,
        indices: Vec<[usize; 3]>,
    ) -> Self {
        Self {
            vertices,
            normals,
            uvs,
            indices,
        }
    }

    /// Adds a vertex to this mesh
    /// unless the vertex already exists
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

    /// Adds a normal to this mesh
    /// unless the normal already exists
    ///
    /// then returns it's index
    pub fn add_normal(&mut self, normal: Vec3) -> usize {
        for i in 0..self.normals.len() {
            let v = &self.normals[i];
            if v != &normal {
                continue;
            }

            return i;
        }

        self.normals.push(normal);
        self.normals.len() - 1
    }

    /// Adds a uv to this mesh
    /// unless the uv already exists
    ///
    /// ten returns it's index
    pub fn add_uv(&mut self, uv: Vec2) -> usize {
        for i in 0..self.uvs.len() {
            let v = &self.uvs[i];
            if v != &uv {
                continue;
            }

            return i;
        }

        self.uvs.push(uv);
        self.uvs.len() - 1
    }

    /// Adds three indices to this mesh
    /// [vertex, uv, normal]
    pub fn add_triangle(&mut self, i0: [usize; 3], i1: [usize; 3], i2: [usize; 3]) {
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
        for i in 0..value.tricount() {
            let tri = value[i];
            let v0 = out.add_vertex(tri[0]);
            let v1 = out.add_vertex(tri[1]);
            let v2 = out.add_vertex(tri[2]);
            let uv0 = out.add_uv(tri.uvs[0]);
            let uv1 = out.add_uv(tri.uvs[1]);
            let uv2 = out.add_uv(tri.uvs[2]);
            let n = out.add_normal(tri.normal);

            out.add_triangle([v0, uv0, n], [v1, uv1, n], [v2, uv2, n]);
        }

        out
    }
}
