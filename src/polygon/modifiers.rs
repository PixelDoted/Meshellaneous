use crate::{
    plane::{Plane, Side},
    vertex::Vertex,
};

use super::Polygon;

impl Polygon {
    /// Slices this polygon in half  
    pub fn slice(&self, plane: &Plane, above: &mut Vec<Polygon>, below: &mut Vec<Polygon>) {
        let d = plane.normal.dot(plane.point);
        let mut vabove = Vec::with_capacity(3);
        let mut vbelow = Vec::with_capacity(3);

        let mut last = plane.side(self[0].point);
        for i in 1..self.len() {
            let vi = self[i];
            let si = plane.side(vi.point);

            if matches!(si, Side::Above | Side::Coplanar) {
                vabove.push(vi);
            }

            if matches!(si, Side::Below | Side::Coplanar) {
                vbelow.push(vi);
            }

            if matches!(
                (si, last),
                (Side::Above, Side::Below) | (Side::Below, Side::Above)
            ) {
                let vj = self[i - 1];
                let vector = vj.point - vi.point;
                let t = (d - plane.normal.dot(vi.point)) / plane.normal.dot(vector);

                let v = Vertex::new(
                    vi.point + vector * t,
                    vi.uv.lerp(vi.uv, t),
                    vi.normal.lerp(vj.normal, t),
                );

                vabove.push(v);
                vbelow.push(v);
            }

            last = si;
        }

        if vabove.len() > 2 {
            above.push(Polygon::new(vabove));
        }

        if vbelow.len() > 2 {
            below.push(Polygon::new(vbelow));
        }
    }
}
