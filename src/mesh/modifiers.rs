use super::TriMesh;

impl TriMesh {
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
}
