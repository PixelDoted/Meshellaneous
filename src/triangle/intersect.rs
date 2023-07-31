use std::f32::EPSILON;

use glam::Vec3;

use crate::{plane::Plane, traits::Intersect, Ray, Segment};

use super::Triangle;

impl Intersect<Ray, Option<Vec3>> for Triangle {
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

impl Intersect<Segment, Option<Vec3>> for Triangle {
    /// get the intersection point of a line segment  
    /// returns None if there's no intersection
    fn intersects(&self, segment: &Segment) -> Option<Vec3> {
        // https://stackoverflow.com/a/58694277
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

impl Intersect<Triangle, bool> for Triangle {
    fn intersects(&self, other: &Triangle) -> bool {
        // AABB Check (Cube-Cube intersection)
        let [amin, amax] = [self.min(), self.max()];
        let [bmin, bmax] = [other.min(), other.max()];

        if !(amin.x <= bmax.x
            && amax.x >= bmin.x
            && amin.y <= bmax.y
            && amax.y >= bmin.y
            && amin.z <= bmax.z
            && amax.x >= bmin.z)
        {
            return false;
        }

        // Edge Intersection (Segment-Triangle intersection)
        self.intersects(&[other[0], other[1]]).is_some()
            || self.intersects(&[other[1], other[2]]).is_some()
            || self.intersects(&[other[2], other[0]]).is_some()
    }
}
