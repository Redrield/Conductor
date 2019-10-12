use std::process;
use std::path::Path;

fn main() {
    process::Command::new("/usr/local/bin/parcel")
        .current_dir(Path::new("./ui").canonicalize().unwrap())
        .arg("build")
        .arg("src/index.tsx")
        .spawn()
        .expect("Failed to spawn")
        .wait()
        .expect("Failed to wait");
}
