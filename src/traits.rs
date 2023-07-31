pub trait Intersect<T, R> {
    fn intersects(&self, other: &T) -> R;
}
