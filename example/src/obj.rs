use glam::{Vec2, Vec3};
use meshellaneous::{meshes::TriMesh, triangle::Triangle};

pub fn decode(s: String) -> std::io::Result<TriMesh> {
    let mut vertices: Vec<Vec3> = Vec::new();
    let mut normals: Vec<Vec3> = Vec::new();
    let mut uvs: Vec<Vec2> = Vec::new();
    let mut triangles: Vec<Triangle> = Vec::new();

    for line in s.lines() {
        let mut args = line.split(" ").skip(1);

        if line.starts_with("v ") {
            let x = args.next().unwrap();
            let y = args.next().unwrap();
            let z = args.next().unwrap();

            vertices.push(Vec3::new(
                x.parse().unwrap(),
                y.parse().unwrap(),
                z.parse().unwrap(),
            ));
        } else if line.starts_with("vt ") {
            let x = args.next().unwrap();
            let y = args.next().unwrap();

            uvs.push(Vec2::new(x.parse().unwrap(), y.parse().unwrap()));
        } else if line.starts_with("vn ") {
            let x = args.next().unwrap();
            let y = args.next().unwrap();
            let z = args.next().unwrap();

            normals.push(Vec3::new(
                x.parse().unwrap(),
                y.parse().unwrap(),
                z.parse().unwrap(),
            ));
        } else if line.starts_with("f ") {
            let mut a = args.next().unwrap().split("/");
            let mut b = args.next().unwrap().split("/");
            let mut c = args.next().unwrap().split("/");

            let v0 = a.next().unwrap();
            let uv0 = a.next().unwrap();
            let n0 = a.next().unwrap();
            let v1 = b.next().unwrap();
            let uv1 = b.next().unwrap();
            let v2 = c.next().unwrap();
            let uv2 = c.next().unwrap();

            triangles.push(Triangle::new(
                [
                    vertices[v0.parse::<usize>().unwrap() - 1],
                    vertices[v1.parse::<usize>().unwrap() - 1],
                    vertices[v2.parse::<usize>().unwrap() - 1],
                ],
                [
                    uvs[uv0.parse::<usize>().unwrap() - 1],
                    uvs[uv1.parse::<usize>().unwrap() - 1],
                    uvs[uv2.parse::<usize>().unwrap() - 1],
                ],
                normals[n0.parse::<usize>().unwrap() - 1],
            ))
        }
    }

    Ok(TriMesh::new(triangles))
}

pub fn encode<W: std::io::Write>(writer: &mut W, mesh: TriMesh) -> std::io::Result<()> {
    let mut vertices: VVec<Vec3> = VVec::new();
    let mut normals: VVec<Vec3> = VVec::new();
    let mut uvs: VVec<Vec2> = VVec::new();
    let mut indices: Vec<[Index; 3]> = Vec::new();

    for tri in mesh.triangles {
        let v0 = vertices.push(tri.points[0]);
        let v1 = vertices.push(tri.points[1]);
        let v2 = vertices.push(tri.points[2]);
        let uv0 = uvs.push(tri.uvs[0]);
        let uv1 = uvs.push(tri.uvs[1]);
        let uv2 = uvs.push(tri.uvs[2]);
        let n = normals.push(tri.normal);

        indices.push([
            Index::new(v0 + 1, uv0 + 1, n + 1),
            Index::new(v1 + 1, uv1 + 1, n + 1),
            Index::new(v2 + 1, uv2 + 1, n + 1),
        ]);
    }

    for v in vertices.inner {
        writer.write_all(format!("v {} {} {}\n", v.x, v.y, v.z).as_bytes())?;
    }

    for uv in uvs.inner {
        writer.write_all(format!("vt {} {}\n", uv.x, uv.y).as_bytes())?;
    }

    for n in normals.inner {
        writer.write_all(format!("vn {} {} {}\n", n.x, n.y, n.z).as_bytes())?;
    }

    for [a, b, c] in indices {
        writer.write_all(
            format!(
                "f {}/{}/{} {}/{}/{} {}/{}/{}\n",
                a.v, a.vt, a.vn, b.v, b.vt, b.vn, c.v, c.vt, c.vn
            )
            .as_bytes(),
        )?;
    }

    writer.flush()?;
    Ok(())
}

struct Index {
    v: usize,
    vt: usize,
    vn: usize,
}

impl Index {
    pub fn new(v: usize, vt: usize, vn: usize) -> Self {
        Self { v, vt, vn }
    }
}

struct VVec<T> {
    inner: Vec<T>,
}

impl<T: PartialEq> VVec<T> {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn push(&mut self, value: T) -> usize {
        for i in 0..self.inner.len() {
            if self.inner[i] != value {
                continue;
            }

            return i;
        }

        self.inner.push(value);
        self.inner.len() - 1
    }
}
