mod modifiers;

use std::{
    f32::EPSILON,
    ops::{Index, IndexMut},
};

use glam::Vec3;

use crate::{plane::Plane, traits::Intersect, Ray, Segment};

#[derive(Clone, Copy, Debug)]
pub struct Triangle {
    points: [Vec3; 3],
    normal: Vec3,
}

impl Triangle {
    pub fn new(points: [Vec3; 3], normal: Vec3) -> Self {
        Self { points, normal }
    }

    /// Gets the normal from points  
    /// and creates a Triangle
    pub fn from_points(points: [Vec3; 3]) -> Self {
        let normal = (points[1] - points[0]).cross(points[2] - points[0]);
        Self::new(points, normal)
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

impl Intersect<Segment> for Triangle {
    /// get the intersection point of a line segment  
    /// returns None if there's no intersection
    fn intersects(&self, segment: &Segment) -> Option<Vec3> {
        // https://stackoverflow.com/questions/54143142/3d-intersection-between-segment-and-triangle
        let plane = Plane::new(self[0], self.normal);
        let point = match plane.intersects(segment) {
            Some(point) => point,
            None => return None,
        };

        let n12 = (self[1] - self[0]).cross(self.normal);
        let n23 = (self[2] - self[1]).cross(self.normal);
        let n31 = (self[0] - self[2]).cross(self.normal);

        let da = (point - self[0]).dot(n12) / n12.length();
        let db = (point - self[1]).dot(n23) / n23.length();
        let dc = (point - self[2]).dot(n31) / n31.length();
        if da < -EPSILON && db < -EPSILON && dc < -EPSILON {
            return Some(point);
        }

        None
    }
}
