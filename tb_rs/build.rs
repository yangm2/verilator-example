extern crate cbindgen;

fn main() {
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    let testbench_svdpi_h = std::path::Path::new(&std::env::var("OUT_DIR").unwrap())
        .with_file_name("testbench_svdpi.h");

    cbindgen::generate(crate_dir)
        .expect("Unable to generate C bindings.")
        .write_to_file(testbench_svdpi_h);
}
