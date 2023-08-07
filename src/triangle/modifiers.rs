use crate::plane::{Plane, Side};

use super::Triangle;

impl Triangle {
    /// Subdivides this triangle into 4 other triangles
    pub fn subdivide(&self) -> [Triangle; 4] {
        let p01 = self[0] + (self[1] - self[0]) * 0.5;
        let p12 = self[1] + (self[2] - self[1]) * 0.5;
        let p20 = self[2] + (self[0] - self[2]) * 0.5;
        let uv01 = self.uvs[0] + (self.uvs[1] - self.uvs[0]) * 0.5;
        let uv12 = self.uvs[1] + (self.uvs[2] - self.uvs[1]) * 0.5;
        let uv20 = self.uvs[2] + (self.uvs[0] - self.uvs[2]) * 0.5;

        [
            Triangle::new([self[0], p01, p20], [self.uvs[0], uv01, uv20], self.normal),
            Triangle::new([p01, self[1], p12], [uv01, self.uvs[1], uv12], self.normal),
            Triangle::new([p20, p12, self[2]], [uv20, uv12, self.uvs[2]], self.normal),
            Triangle::new([p01, p12, p20], [uv01, uv12, uv20], self.normal),
        ]
    }

    /// Slices this triangle in half  
    /// outputs 3-1 triangles total
    pub fn slice(&self, plane: &Plane, above: &mut Vec<Triangle>, below: &mut Vec<Triangle>) {
        let d = plane.normal.dot(plane.point);
        let sides = [
            plane.side(self[0]),
            plane.side(self[1]),
            plane.side(self[2]),
        ];

        // Skip for loop
        if sides[0] == sides[1] && sides[0] == sides[2] {
            if matches!(sides[0], Side::Coplanar | Side::Above) {
                above.push(*self);
            }

            if matches!(sides[0], Side::Coplanar | Side::Below) {
                below.push(*self);
            }

            return;
        }

        let mut vabove = Vec::with_capacity(3);
        let mut vbelow = Vec::with_capacity(3);
        for i in 0..3 {
            let j = (i + 1) % 3;
            let si = sides[i];

            if si == Side::Coplanar {
                if plane.normal.dot(self.normal) > 0.0 {
                    vabove.push((self[i], self.uvs[i]));
                } else {
                    vbelow.push((self[i], self.uvs[i]));
                }
            }

            if si == Side::Above {
                vabove.push((self[i], self.uvs[i]));
            }

            if si == Side::Below {
                vbelow.push((self[i], self.uvs[i]));
            }

            if matches!(
                (si, sides[j]),
                (Side::Above, Side::Below) | (Side::Below, Side::Above)
            ) {
                let vector = self[j] - self[i];
                let t = (d - plane.normal.dot(self[i])) / plane.normal.dot(vector);

                let v = self[i] + vector * t; // get vertex
                let uv = self.uvs[i].lerp(self.uvs[j], t); // get uv

                vabove.push((v, uv));
                vbelow.push((v, uv));
            }

            // Add above triangle
            while vabove.len() >= 3 {
                let v0 = vabove[0];
                let v1 = vabove.remove(1);
                let v2 = vabove[1];

                above.push(Triangle::new(
                    [v0.0, v1.0, v2.0],
                    [v0.1, v1.1, v2.1],
                    self.normal,
                ));
            }

            // Add below triangle
            while vbelow.len() >= 3 {
                let v0 = vbelow[0];
                let v1 = vbelow.remove(1);
                let v2 = vbelow[1];

                below.push(Triangle::new(
                    [v0.0, v1.0, v2.0],
                    [v0.1, v1.1, v2.1],
                    self.normal,
                ));
            }
        }
    }
}
