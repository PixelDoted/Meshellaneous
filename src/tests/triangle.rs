use glam::Vec3;

use crate::{plane::Plane, traits::Intersect, triangle::Triangle, Ray, Segment};

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

#[test]
pub fn slice() {
    let triangle = Triangle::from_points([
        Vec3::new(-1.0, 1.0, 0.0),
        Vec3::new(-1.0, -1.0, 0.0),
        Vec3::new(1.0, -1.0, 0.0),
    ]);
    let plane = Plane::new(Vec3::ZERO, Vec3::X);

    let tris = triangle.slice(&plane);
    assert_eq!(tris.len(), 3);
    assert_eq!(
        [tris[0][0], tris[0][1], tris[0][2]],
        [
            Vec3::new(-1.0, 1.0, 0.0),
            Vec3::new(-1.0, -1.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0)
        ]
    );

    assert_eq!(
        [tris[1][0], tris[1][1], tris[1][2]],
        [
            Vec3::new(0.0, -1.0, 0.0),
            Vec3::new(1.0, -1.0, 0.0),
            Vec3::ZERO
        ]
    );

    assert_eq!(
        [tris[2][0], tris[2][1], tris[2][2]],
        [
            Vec3::new(-1.0, 1.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
            Vec3::ZERO
        ]
    );
}
