#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    Coplanar,
    Below,
    Above,
}

impl Side {
    pub fn aprox_equals(&self, rhs: &Side) -> bool {
        self == rhs || rhs == &Side::Coplanar || self == &Side::Coplanar
    }

    pub fn aprox_above(&self) -> bool {
        matches!(self, Side::Coplanar | Side::Above)
    }

    pub fn aprox_below(&self) -> bool {
        matches!(self, Side::Coplanar | Side::Below)
    }
}
