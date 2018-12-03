extern crate cpp_new;
extern crate cpp_new_simple_cpp_lib;
extern crate jemallocator;

#[global_allocator]
#[cfg(not(windows))]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

use cpp_new_simple_cpp_lib::*;

#[test]
fn new() {
    let foo = safe_create_foo();

    assert_eq!(foo.i, 42);
    assert_eq!(foo.f, 42.0);
    assert_eq!(foo.c, 42);

    safe_destroy_foo(foo);
}

#[test]
fn array_new() {
    let foos = safe_create_foos(512);

    for foo in foos.iter() {
        assert_eq!(foo.i, 42);
        assert_eq!(foo.f, 42.0);
        assert_eq!(foo.c, 42);
    }

    safe_destroy_foos(foos);
}
