use super::Triangle;

/// A sliced triangle
pub struct Slice {
    pub above: Vec<Triangle>,
    pub below: Vec<Triangle>,
}

impl Slice {
    pub(crate) fn empty() -> Self {
        Self {
            above: Vec::with_capacity(2),
            below: Vec::with_capacity(2),
        }
    }
}
