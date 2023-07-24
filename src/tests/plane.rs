use glam::Vec3;

use crate::{
    plane::{Plane, Side},
    traits::Intersect,
    Ray, Segment,
};

#[test]
pub fn side_above() {
    let plane = Plane::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
    let side = plane.side(Vec3::new(0.0, 1.0, 0.0));
    assert_eq!(side, Side::Above);
}

#[test]
pub fn side_below() {
    let plane = Plane::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
    let side = plane.side(Vec3::new(0.0, -1.0, 0.0));
    assert_eq!(side, Side::Below);
}

#[test]
pub fn side_coplanar() {
    let plane = Plane::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
    let side = plane.side(Vec3::new(1.0, 0.0, 0.0));
    assert_eq!(side, Side::Coplanar);
}

#[test]
pub fn intersect_ray() {
    let plane = Plane::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
    let ray: Ray = (Vec3::new(0.0, 1.0, 0.1), Vec3::new(0.0, -1.0, 0.0));

    assert_eq!(plane.intersects(&ray), Some(Vec3::new(0.0, 0.0, 0.1)));
}

#[test]
pub fn intersect_ray_parallel() {
    let plane = Plane::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
    let ray: Ray = (Vec3::new(0.0, 1.0, 0.1), Vec3::new(1.0, 0.0, 0.0));

    assert_eq!(plane.intersects(&ray), None);
}

#[test]
pub fn intersect_segment() {
    let plane = Plane::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
    let segment: Segment = [Vec3::new(0.0, 1.0, 0.1), Vec3::new(0.0, -1.0, -0.1)];

    assert_eq!(plane.intersects(&segment), Some(Vec3::ZERO));
}

#[test]
pub fn intersect_segment_parallel() {
    let plane = Plane::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
    let segment: Segment = [Vec3::new(0.0, 1.0, 0.1), Vec3::new(1.0, 1.0, 0.1)];

    assert_eq!(plane.intersects(&segment), None);
}

#[test]
pub fn intersect_segment_away() {
    let plane = Plane::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
    let segment: Segment = [Vec3::new(0.0, 1.0, 0.1), Vec3::new(0.0, 2.0, 0.1)];

    assert_eq!(plane.intersects(&segment), None);
}
