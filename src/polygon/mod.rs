mod modifiers;

use std::{
    f32::EPSILON,
    ops::{Index, IndexMut},
};

use glam::Vec3;

use crate::{plane::Plane, traits::Intersect, triangle::Triangle, vertex::Vertex, Ray};

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

    pub fn center(&self) -> Vec3 {
        let mut out = self[0].point;
        for i in 1..self.len() {
            out += self[i].point;
        }

        out / self.len() as f32
    }

    pub fn flip(&self) -> Polygon {
        let mut output = Polygon::new(Vec::new());
        for i in 1..=self.len() {
            let i = self.len() - i;
            output.add(Vertex::new(self[i].point, self[i].uv, -self[i].normal));
        }

        output
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

impl Intersect<Ray, Option<Vec3>> for Polygon {
    fn intersects(&self, other: &Ray) -> Option<Vec3> {
        let plane = Plane::new(self[0].point, self[0].normal);
        let point = match plane.intersects(other) {
            Some(point) => point,
            None => return None,
        };

        for i in 0..self.len() {
            let j = (i + 1) % self.len();
            let n12 = (self[j].point - self[i].point).cross(self[0].normal);

            let da = (point - self[i].point).dot(n12) / n12.length();
            if da >= -EPSILON {
                return None;
            }
        }

        Some(point)
    }
}
