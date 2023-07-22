use std::{
    f32::EPSILON,
    ops::{Index, IndexMut},
};

use glam::Vec3;

use crate::{traits::Intersect, Line, Ray};

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

impl Intersect<Ray> for Triangle {
    /// get the intersection point of a ray  
    /// returns None if there's no intersection
    fn intersects(&self, ray: &Ray) -> Option<Vec3> {
        // TODO: Find a faster algorithm

        //https://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm
        let edge1 = self[1] - self[0];
        let edge2 = self[2] - self[0];
        let h = ray.1.cross(edge2);
        let a = edge1.dot(h);

        if a > -EPSILON && a < EPSILON {
            return None; // This ray is parallel to this triangle.
        }

        let f = 1.0 / a;
        let s = ray.0 - self[0];
        let u = f * s.dot(h);

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = s.cross(edge1);
        let v = f * ray.1.dot(q);

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        // At this stage we can compute t to find out where the intersection point is on the line.
        let t = f * edge2.dot(q);

        if t > EPSILON {
            // ray intersection
            Some(ray.0 + ray.1 * t)
        } else {
            // This means that there is a line intersection but not a ray intersection.
            None
        }
    }
}

impl Intersect<Line> for Triangle {
    /// get the intersection point of a line segment  
    /// returns None if there's no intersection
    fn intersects(&self, line: &Line) -> Option<Vec3> {
        // TODO: Find a faster algorithm

        let max_dist = line[0].distance_squared(line[1]);
        let ray = (line[0], line[1] - line[0]);
        if let Some(point) = self.intersects(&ray) {
            let dist = point.distance_squared(ray.0);
            println!("{}", dist);

            if dist <= max_dist {
                return Some(point);
            }
        }

        None
    }
}
