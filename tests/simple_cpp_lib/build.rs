extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/lib.cpp")
        .cpp(true)
        .compile("cpp_new_simple_cpp_lib");
}
