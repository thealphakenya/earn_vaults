use std::fs;
use std::ffi::OsStr;
use cc::Build;

fn main() {
    let cpp_dir = "src/cpp/";

    // Ensure Cargo re-runs the build script if any C++ file changes
    println!("cargo:rerun-if-changed={}", cpp_dir);

    let mut build = Build::new();
    
    // Enable C++ compilation
    build.cpp(true)
         .flag_if_supported("-std=c++17"); // Enable C++17 if supported

    // Find and add all `.cpp` files in the cpp directory to the build
    if let Ok(entries) = fs::read_dir(cpp_dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.extension() == Some(OsStr::new("cpp")) {
                // Print the path of each C++ file being compiled
                println!("Compiling: {:?}", path);
                build.file(path);
            }
        }
    }

    // Compile the static library
    build.compile("hello_library");

    // Link the compiled library to Rust
    println!("cargo:rustc-link-lib=static=hello_library");
}
