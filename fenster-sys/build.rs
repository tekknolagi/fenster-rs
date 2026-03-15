fn main() {
    cc::Build::new()
        .include("fenster")
        .file("src/wrapper.c")
        .compile("fenster");

    println!("cargo::rerun-if-changed=src/wrapper.c");
    println!("cargo::rustc-link-lib=user32");
    println!("cargo::rustc-link-lib=gdi32");
}
