fn main() {
    let out_path = cmake::Config::new("usrsctp")
        .build_target("usrsctp-static")
        .build();

    println!("cargo:rustc-link-lib=static=usrsctp");
    println!(
        "cargo:rustc-link-search=native={}/build/usrsctplib",
        out_path.display()
    );

    bindgen::builder()
        .header("usrsctp/usrsctplib/usrsctp.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
