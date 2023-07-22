use glam::Vec3;

pub trait Intersect<T> {
    fn intersects(&self, other: &T) -> Option<Vec3>;
}
