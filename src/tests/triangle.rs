use glam::Vec3;

use crate::{traits::Intersect, triangle::Triangle, Ray, Segment};

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
    let ray: Segment = [Vec3::new(0.0, 1.0, 0.1), Vec3::new(0.0, -1.0, -0.1)];

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
    let ray: Segment = [Vec3::new(0.0, 1.0, 0.1), Vec3::new(1.0, 1.0, 0.1)];

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
    let ray: Segment = [Vec3::new(0.0, 1.0, 0.1), Vec3::new(0.0, 2.0, 0.1)];

    assert_eq!(triangle.intersects(&ray), None);
}

#[test]
pub fn subdivide() {
    let triangle = Triangle::from_points([
        Vec3::new(-1.0, 1.0, 0.0),
        Vec3::new(-1.0, -1.0, 0.0),
        Vec3::new(1.0, -1.0, 0.0),
    ]);

    let [a, b, c, d] = triangle.subdivide();

    // A
    assert_eq!(
        [a[0], a[1], a[2]],
        [
            Vec3::new(-1.0, 1.0, 0.0),
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 0.0)
        ]
    );

    // B
    assert_eq!(
        [b[0], b[1], b[2]],
        [
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(-1.0, -1.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0)
        ]
    );

    // C
    assert_eq!(
        [c[0], c[1], c[2]],
        [
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
            Vec3::new(1.0, -1.0, 0.0)
        ]
    );

    // D
    assert_eq!(
        [d[0], d[1], d[2]],
        [
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
            Vec3::new(0.0, 0.0, 0.0)
        ]
    );
}
