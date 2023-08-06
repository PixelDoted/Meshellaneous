use std::f32::EPSILON;

use glam::Vec3;

use crate::{plane::Plane, traits::Intersect, Ray, Segment};

use super::Triangle;

impl Intersect<Ray, Option<Vec3>> for Triangle {
    /// get the intersection point of a ray
    /// returns None if there's no intersection
    fn intersects(&self, ray: &Ray) -> Option<Vec3> {
        // https://stackoverflow.com/a/42752998
        let e1 = self[1] - self[0];
        let e2 = self[2] - self[0];
        let n = e1.cross(e2);
        let det = -ray.1.dot(n);
        let invdet = 1.0 / det;

        let ao = ray.0 - self[0];
        let dao = ao.cross(ray.1);
        let u = e2.dot(dao) * invdet;
        let v = -e1.dot(dao) * invdet;
        let t = ao.dot(n) * invdet;

        if det >= EPSILON && t >= 0.0 && u >= 0.0 && v >= 0.0 && (u + v) <= 1.0 {
            Some(ray.0 + t * ray.1)
        } else {
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
    /// returns whether this Triangle intersect another Triangle
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
