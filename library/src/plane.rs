use std::f32::EPSILON;

use glam::Vec3;

use crate::{traits::Intersect, Ray, Segment};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    Coplanar,
    Below,
    Above,
}

/// An infinitly large plane
#[derive(Clone, Copy, Debug)]
pub struct Plane {
    pub point: Vec3,
    pub normal: Vec3,
}

impl Plane {
    pub fn new(point: Vec3, normal: Vec3) -> Self {
        Self { point, normal }
    }

    /// Returns which side of the plane, `point` is on
    pub fn side(&self, point: Vec3) -> Side {
        let dot = self.normal.dot(point - self.point);
        if dot < -EPSILON {
            Side::Below
        } else if dot > EPSILON {
            Side::Above
        } else {
            Side::Coplanar
        }
    }
}

impl Intersect<Segment, Option<Vec3>> for Plane {
    /// get the intersection point of a line segment  
    /// returns None if there's no intersection
    fn intersects(&self, segment: &Segment) -> Option<Vec3> {
        // https://stackoverflow.com/questions/7168484/3d-line-segment-and-plane-intersection
        let d = self.normal.dot(self.point);

        let ray = segment[1] - segment[0];
        if self.normal.dot(ray) == 0.0 {
            return None;
        }

        let t = (d - self.normal.dot(segment[0])) / self.normal.dot(ray);
        if t >= 0.0 && t <= 1.0 {
            return Some(segment[0] + ray * t);
        }

        None
    }
}

impl Intersect<Ray, Option<Vec3>> for Plane {
    /// get the intersection point of a ray  
    /// returns None if there's no intersection
    fn intersects(&self, ray: &Ray) -> Option<Vec3> {
        // https://stackoverflow.com/a/23976134
        let denom = self.normal.dot(ray.1);
        if denom.abs() > EPSILON {
            let t = (self.point - ray.0).dot(self.normal) / denom;
            if t < EPSILON {
                return None;
            }

            Some(ray.0 + t * ray.1)
        } else {
            None
        }
    }
}
