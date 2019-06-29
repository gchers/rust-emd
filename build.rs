extern crate cc;

use std::fs;
use std::env;

const SRC_FILE: &str = "pyemd/c_emd/emd.c";
const SRC_PATCHED: &str = "pyemd/c_emd/emd.patched.c";


fn main() {
    // Patch before compilation
    let source = fs::read_to_string(SRC_FILE)
                    .expect("Unable to read \"emd.c\"");
    let source_patched = source.replace("#include <emd.h>",
                                        "#include \"emd.h\"");
    fs::write(SRC_PATCHED, source_patched)
       .expect("Could not patch source code");

    // Compile
    cc::Build::new()
              .file(SRC_PATCHED)
              .compile("libemd.a");

    // Clean up
    fs::remove_file(SRC_PATCHED)
       .expect("Failed to remove patched file");

    // Link library
    let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search={}/src/", project_dir);
    println!("proj dir: {}", project_dir);
    println!("cargo:rustc-link-lib=emd");
}
