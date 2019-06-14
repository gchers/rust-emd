extern crate cc;

use std::fs;
use std::env;
use std::process::Command;

const REPO: &str = "https://github.com/garydoranjr/pyemd";
const COMMIT: &str = "57fb492e72813bd3d4d841a8b61d89c76ba137bf";


fn main() {
    // Define paths
    let zip_url = &format!("{}/archive/{}.zip", REPO, COMMIT);
    let zip_file = &format!("src/pyemd-{}.zip", COMMIT);
    let source_dir = &format!("src/pyemd-{}", COMMIT);
    let source_file = &format!("{}/c_emd/emd.c", source_dir);

    // Clone pyemd repo
    let _ = Command::new("wget")
                    .args(&[zip_url, "-O", zip_file])
                    .output()
                    .expect("Failed to run `wget`");
    
    let _ = Command::new("unzip")
                    .args(&[zip_file, "-d", "src/"])
                    .output()
                    .expect("Failed to extract source with `zip`");

    // Patch header import before compilation
    let source = fs::read_to_string(source_file)
                    .expect("Unable to read source code");
    let source_patched = source.replace("#include <emd.h>",
                                        "#include \"emd.h\"");
    fs::write(source_file, source_patched)
       .expect("Could not patch source code");

    // Compile
    cc::Build::new()
              .file(source_file)
              .compile("libemd.a");

    // Clean up
    fs::remove_dir_all(source_dir).expect("Clean up failed: source directory");
    fs::remove_file(zip_file).expect("Clean up failed: zip file");

    // Link library
    let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search={}/src/", project_dir);
    println!("proj dir: {}", project_dir);
    println!("cargo:rustc-link-lib=emd");
}
