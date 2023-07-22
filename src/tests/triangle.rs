use glam::Vec3;

use crate::{traits::Intersect, triangle::Triangle, Line, Ray};

#[test]
pub fn intersect_ray() {
    let triangle = Triangle::new(
        [
            Vec3::new(0.0, 0.0, 0.5),
            Vec3::new(0.5, 0.0, -0.5),
            Vec3::new(-0.5, 0.0, -0.5),
        ],
        Vec3::Y,
    );
    let ray: Ray = (Vec3::new(0.0, 1.0, 0.1), Vec3::new(0.0, -1.0, 0.0));

    assert_eq!(triangle.intersects(&ray), Some(Vec3::new(0.0, 0.0, 0.1)));
}

#[test]
pub fn intersect_ray_parallel() {
    let triangle = Triangle::new(
        [
            Vec3::new(0.0, 0.0, 0.5),
            Vec3::new(0.5, 0.0, -0.5),
            Vec3::new(-0.5, 0.0, -0.5),
        ],
        Vec3::Y,
    );
    let ray: Ray = (Vec3::new(0.0, 1.0, 0.1), Vec3::new(1.0, 0.0, 0.0));

    assert_eq!(triangle.intersects(&ray), None);
}

#[test]
pub fn intersect_ray_away() {
    let triangle = Triangle::new(
        [
            Vec3::new(0.0, 0.0, 0.5),
            Vec3::new(0.5, 0.0, -0.5),
            Vec3::new(-0.5, 0.0, -0.5),
        ],
        Vec3::Y,
    );
    let ray: Ray = (Vec3::new(0.0, 1.0, 0.1), Vec3::new(0.0, 1.0, 0.0));

    assert_eq!(triangle.intersects(&ray), None);
}

#[test]
pub fn intersect_line() {
    let triangle = Triangle::new(
        [
            Vec3::new(0.0, 0.0, 0.5),
            Vec3::new(0.5, 0.0, -0.5),
            Vec3::new(-0.5, 0.0, -0.5),
        ],
        Vec3::Y,
    );
    let ray: Line = [Vec3::new(0.0, 1.0, 0.1), Vec3::new(0.0, -1.0, -0.1)];

    assert_eq!(triangle.intersects(&ray), Some(Vec3::ZERO));
}

#[test]
pub fn intersect_line_parallel() {
    let triangle = Triangle::new(
        [
            Vec3::new(0.0, 0.0, 0.5),
            Vec3::new(0.5, 0.0, -0.5),
            Vec3::new(-0.5, 0.0, -0.5),
        ],
        Vec3::Y,
    );
    let ray: Line = [Vec3::new(0.0, 1.0, 0.1), Vec3::new(1.0, 1.0, 0.1)];

    assert_eq!(triangle.intersects(&ray), None);
}

#[test]
pub fn intersect_line_away() {
    let triangle = Triangle::new(
        [
            Vec3::new(0.0, 0.0, 0.5),
            Vec3::new(0.5, 0.0, -0.5),
            Vec3::new(-0.5, 0.0, -0.5),
        ],
        Vec3::Y,
    );
    let ray: Line = [Vec3::new(0.0, 1.0, 0.1), Vec3::new(0.0, 2.0, 0.1)];

    assert_eq!(triangle.intersects(&ray), None);
}
