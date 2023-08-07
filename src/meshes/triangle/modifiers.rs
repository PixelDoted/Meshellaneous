use crate::triangle::Triangle;

use super::TriMesh;

impl TriMesh {
    /// Subdivides every triangle into 4 other triangles
    pub fn subdivide(&self) -> Self {
        let mut output = Self::new(Vec::new());
        for t in &self.triangles {
            let tris = t.subdivide();
            output.add(tris[0]);
            output.add(tris[1]);
            output.add(tris[2]);
            output.add(tris[3]);
        }

        output
    }

    /// Seperates any loose parts into there own mesh
    pub fn separate_by_loose_parts(self) -> Vec<Self> {
        let mut meshes: Vec<Self> = Vec::new();
        for t in self.triangles {
            let mut out = vec![t];

            let mut i = 0;
            while i < meshes.len() {
                if !is_connected(&out, &meshes[i].triangles) {
                    i += 1;
                    continue;
                }

                out.append(&mut meshes.remove(i).triangles);
            }

            meshes.push(TriMesh::new(out));
        }

        meshes
    }
}

fn is_connected(a: &[Triangle], b: &[Triangle]) -> bool {
    for a in a {
        for b in b {
            if !(a[0] == b[0]
                || a[0] == b[1]
                || a[0] == b[2]
                || a[1] == b[0]
                || a[1] == b[1]
                || a[1] == b[2]
                || a[2] == b[0]
                || a[2] == b[1]
                || a[2] == b[2])
            {
                continue;
            }

            return true;
        }
    }

    false
}
