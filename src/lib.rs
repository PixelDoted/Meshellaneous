//! # Meshellaneous
//! Miscellaneous Mesh utilities

use glam::Vec3;

pub mod mesh;
pub mod plane;
pub mod traits;
pub mod triangle;

#[cfg(test)]
mod tests;

/// a line segment from [0] to [1]
pub type Line = [Vec3; 2];

/// a ray at .0 pointing towards .1
pub type Ray = (Vec3, Vec3);
