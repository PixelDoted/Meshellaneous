fn main() {
    let data = std::fs::read_to_string("./mesh.obj").unwrap();
    let mesh = example::obj::decode(data).unwrap();

    for (i, mesh) in mesh.separate_by_loose_parts().into_iter().enumerate() {
        let mut file = std::fs::File::create(format!("output{}.obj", i)).unwrap();
        example::obj::encode(&mut file, mesh).unwrap();
    }
}
