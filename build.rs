const PROTO_PATH: &str = "lib/ampoule-protos/protos";

fn main() {
    let proto_files: Vec<String> = vec!["command"]
        .into_iter()
        .map(|x| format!("{}/{}.proto", PROTO_PATH, x).to_string())
        .collect();

    prost_build::compile_protos(&proto_files, &[PROTO_PATH.to_string()]).unwrap();
}
