use std::fs;

fn main() {
    cxx_build::bridge("src/geo/types.rs")
        .flag("-std=c++17")
        .flag("-IClipper2/CPP/Clipper2Lib/include")
        .file("src/geo/lart.cpp")
        .file("Clipper2/CPP/Clipper2Lib/src/clipper.engine.cpp")
        .file("Clipper2/CPP/Clipper2Lib/src/clipper.offset.cpp")
        .compile("lart");

    println!("cargo:rerun-if-changed=src/geo/types.rs");
    println!("cargo:rerun-if-changed=include/lart.h");
    println!("cargo:rerun-if-changed=src/geo/lart.cpp");
    for f in fs::read_dir("Clipper2/CPP/Clipper2Lib").unwrap() {
        let f = f.unwrap().path();
        println!("cargo:rerun-if-changed={}", f.as_os_str().to_str().unwrap());
    }
}
