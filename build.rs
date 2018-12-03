extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/new.cpp")
        .cpp(true)
        .compile("cpp_new");
}
