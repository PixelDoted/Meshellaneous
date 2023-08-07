mod modifiers;

use std::ops::{Index, IndexMut};

use glam::Vec3;

use crate::{triangle::Triangle, vertex::Vertex};

#[derive(Clone, Debug, Default)]
pub struct Polygon {
    pub vertices: Vec<Vertex>,
}

impl Polygon {
    pub fn new(vertices: Vec<Vertex>) -> Self {
        Self { vertices }
    }

    pub fn add(&mut self, vertex: Vertex) {
        self.vertices.push(vertex);
    }

    pub fn remove(&mut self, index: usize) -> Vertex {
        self.vertices.remove(index)
    }

    pub fn len(&self) -> usize {
        self.vertices.len()
    }

    pub fn is_empty(&self) -> bool {
        self.vertices.is_empty()
    }

    pub fn max(&self) -> Vec3 {
        let mut out = self.vertices[0].point;
        for i in 1..self.vertices.len() {
            out = out.max(self.vertices[i].point);
        }

        out
    }

    pub fn min(&self) -> Vec3 {
        let mut out = self.vertices[0].point;
        for i in 1..self.vertices.len() {
            out = out.min(self.vertices[i].point);
        }

        out
    }
}

impl Index<usize> for Polygon {
    type Output = Vertex;

    fn index(&self, index: usize) -> &Self::Output {
        &self.vertices[index]
    }
}

impl IndexMut<usize> for Polygon {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.vertices[index]
    }
}

impl From<Triangle> for Polygon {
    fn from(value: Triangle) -> Self {
        Polygon {
            vertices: vec![
                Vertex::new(value[0], value.uvs[0], value.normal),
                Vertex::new(value[1], value.uvs[1], value.normal),
                Vertex::new(value[2], value.uvs[2], value.normal),
            ],
        }
    }
}
