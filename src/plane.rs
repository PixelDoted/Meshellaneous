use std::f32::EPSILON;

use glam::Vec3;

use crate::traits::Intersect;

/// An infinitly large plane
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

impl Intersect<[Vec3; 2]> for Plane {
    /// get the intersection point of the line [0] to [1]  
    /// returns None if there's no intersection
    fn intersects(&self, other: &[Vec3; 2]) -> Option<Vec3> {
        let u = other[1] - other[0];
        let dot = self.normal.dot(u);

        if dot.abs() > EPSILON {
            let w = other[0] - self.point;
            let fac = -self.normal.dot(w) / dot;
            let u = u * fac;
            Some(other[0] + u)
        } else {
            None
        }
    }
}

pub enum Side {
    Coplanar,
    Below,
    Above,
}
