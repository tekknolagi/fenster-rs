fn main() {
    cc::Build::new()
        .include("fenster")
        .file("src/wrapper.c")
        .compile("fenster");

    println!("cargo::rerun-if-changed=src/wrapper.c");
}
