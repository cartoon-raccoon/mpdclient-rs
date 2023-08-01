use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-search=/usr/lib");

    println!("cargo:rustc-link-lib=mpdclient");

    //println!("cargo:rerun-if-changed=build/mpdclient.h");

    let bindings = bindgen::Builder::default()
        .header("build/mpdclient.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    //let src_path = PathBuf::from("mpdclient-sys/src");

    //println!("{}", src_path.display());

    println!("{}", out_path.display());

    bindings.write_to_file(out_path.join("bindings.rs"))
        .expect("Unable to write bindings to out_dir");

    // bindings.write_to_file(src_path.join("bindings.rs"))
    //     .expect("Unable to write bindings to source file");
}