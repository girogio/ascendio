use std::fs::copy;
use std::path::Path;

const PATH_LIB: &str = "lib";
const LIBRARIES: [&str; 1] = ["SimConnect.dll"];

fn main() {
    for lib in LIBRARIES.iter() {
        let path = Path::new(PATH_LIB).join(lib);
        copy(path, format!("target/debug/{}", lib)).unwrap();
    }

    tauri_build::build();
}
