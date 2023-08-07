use glam::Vec3;
use meshellaneous::{meshes::TriMesh, plane::Plane};

fn main() {
    let data = std::fs::read_to_string("./mesh.obj").unwrap();
    let mesh = example::obj::decode(data).unwrap();

    let plane = Plane::new(Vec3::ZERO, Vec3::Y);
    let mut above_mesh = TriMesh::default();
    let mut below_mesh = TriMesh::default();
    for tri in mesh.triangles {
        tri.slice(&plane, &mut above_mesh.triangles, &mut below_mesh.triangles);
    }

    let mut file = std::fs::File::create("output0.obj").unwrap();
    example::obj::encode(&mut file, above_mesh).unwrap();

    let mut file = std::fs::File::create("output1.obj").unwrap();
    example::obj::encode(&mut file, below_mesh).unwrap();
}
