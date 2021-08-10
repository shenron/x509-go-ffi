use std::fs;
use std::path::Path;

fn main() {
    let mut config_content = "[build]";
    if cfg!(target_os = "macos") {
        config_content = "[build]
rustflags = [\"-C\", \"link-args=-framework CoreFoundation -framework Security\"]";
    }

    let out_dir = ".cargo";
    let dest_path = Path::new(&out_dir).join("config");
    fs::write(&dest_path, config_content).unwrap();

    let path = "./golib";
    let lib = "gophernize";

    println!("cargo:rustc-link-search=native={}", path);
    println!("cargo:rustc-link-lib=static={}", lib);
}
