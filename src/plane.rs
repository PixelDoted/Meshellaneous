use std::f32::EPSILON;

use glam::Vec3;

use crate::{traits::Intersect, Line, Ray};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    Coplanar,
    Below,
    Above,
}

/// An infinitly large plane
#[derive(Clone, Copy, Debug)]
pub struct Plane {
    point: Vec3,
    normal: Vec3,
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

impl Intersect<Line> for Plane {
    /// get the intersection point of a line segment  
    /// returns None if there's no intersection
    fn intersects(&self, line: &Line) -> Option<Vec3> {
        // TODO: Find a faster algorithm

        let max_dist = line[0].distance_squared(line[1]);
        let ray = (line[0], line[1] - line[0]);
        if let Some(point) = self.intersects(&ray) {
            let dist = line[0].distance_squared(point);

            if dist <= max_dist {
                return Some(point);
            }
        }

        None
    }
}

impl Intersect<Ray> for Plane {
    /// get the intersection point of a ray  
    /// returns None if there's no intersection
    fn intersects(&self, ray: &Ray) -> Option<Vec3> {
        // https://stackoverflow.com/questions/23975555/how-to-do-ray-plane-intersection
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
