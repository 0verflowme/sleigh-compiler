use std::path::PathBuf;

fn main() {
    // The compiler links libsla-sys's libsla, so it must see the same class layouts: its headers
    // first, this crate's own Ghidra copy only where libsla-sys does not state them.
    let source_path = std::env::var("DEP_SLA_INCLUDE")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("ghidra/Ghidra/Features/Decompiler/src/decompile/cpp"));
    cxx_build::bridge("src/ffi/sys.rs")
        .define("LOCAL_ZLIB", "1")
        .define("NO_GZIP", "1")
        .flag_if_supported("-std=c++14")
        .file("src/ffi/cpp/bridge.cc")
        // A fork of the slgh_compile.cc file is maintained locally to enable it
        // to be used as a library instead of as an executable
        .file("src/ffi/cpp/slgh_compile.cc")
        .include(&source_path) // Header files coexist with cpp files
        .warnings(false) // Not interested in the warnings for Ghidra code
        .compile("slacomp");

    println!("cargo:rustc-link-lib=sla");

    // Rerun if any of the bindings have changed
    println!("cargo:rerun-if-changed=src/ffi/sys.rs");

    // Rerun if any of the C++ bridge code has changed
    println!("cargo:rerun-if-changed=src/ffi/cpp");

    println!("cargo:rerun-if-env-changed=DEP_SLA_INCLUDE");
    println!("cargo:rerun-if-changed={}", source_path.display());
}
