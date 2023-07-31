use crate::plane::{Plane, Side};

use super::Triangle;

impl Triangle {
    /// Subdivides this triangle into 4 other triangles
    pub fn subdivide(&self) -> [Triangle; 4] {
        let p01 = self[0] + (self[1] - self[0]) * 0.5;
        let p12 = self[1] + (self[2] - self[1]) * 0.5;
        let p20 = self[2] + (self[0] - self[2]) * 0.5;

        [
            Triangle::new([self[0], p01, p20], self.normal),
            Triangle::new([p01, self[1], p12], self.normal),
            Triangle::new([p20, p12, self[2]], self.normal),
            Triangle::new([p01, p12, p20], self.normal),
        ]
    }

    /// Slices this triangle in half  
    /// this will usually output 3 triangles  
    /// but in rare cases can output 2 triangles  
    /// note: returns 1 triangle if nothing changed
    pub fn slice(&self, plane: &Plane) -> Vec<Triangle> {
        let d = plane.normal.dot(plane.point);
        let sides = [
            plane.side(self[0]),
            plane.side(self[1]),
            plane.side(self[2]),
        ];

        let mut triangles = Vec::with_capacity(3);
        let mut above = Vec::with_capacity(3);
        let mut below = Vec::with_capacity(3);
        for i in 0..3 {
            let j = (i + 1) % 3;
            let si = sides[i];

            if si == Side::Coplanar {
                if plane.normal.dot(self.normal) > 0.0 {
                    above.push(self[i]);
                } else {
                    below.push(self[i]);
                }

                continue;
            }

            if si == Side::Above {
                above.push(self[i]);
            }

            if si == Side::Below {
                below.push(self[i]);
            }

            if matches!(
                (si, sides[j]),
                (Side::Above, Side::Below) | (Side::Below, Side::Above)
            ) {
                let vector = self[j] - self[i];
                let t = (d - plane.normal.dot(self[i])) / plane.normal.dot(vector);
                let v = self[i] + vector * t;
                above.push(v);
                below.push(v);
            }

            if above.len() == 3 {
                triangles.push(Triangle::new(
                    [above[0], above.remove(1), above[1]],
                    self.normal,
                ));
            }

            if below.len() == 3 {
                triangles.push(Triangle::new(
                    [below[0], below.remove(1), below[1]],
                    self.normal,
                ));
            }
        }

        triangles
    }
}
