extern crate cpp_new;
extern crate cpp_new_simple_cpp_lib;
extern crate libc;

use std::alloc::{GlobalAlloc, Layout};
use std::cell::RefCell;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

#[macro_use] extern crate lazy_static;

lazy_static! {
    static ref MUTEX: Mutex<()> = Mutex::new(());
}

thread_local! {
    pub static ALLOC_COUNT: RefCell<AtomicUsize> = RefCell::new(AtomicUsize::new(0));
    pub static FREE_COUNT: RefCell<AtomicUsize> = RefCell::new(AtomicUsize::new(0));
}

struct MyAllocator;

unsafe impl GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOC_COUNT.with(|x| {
            x.borrow_mut().fetch_add(1, Ordering::SeqCst);
        });
        libc::malloc(layout.size() as libc::size_t) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        FREE_COUNT.with(|x| {
            x.borrow_mut().fetch_add(1, Ordering::SeqCst);
        });
        libc::free(ptr as *mut libc::c_void)
    }
}

#[global_allocator]
static A: MyAllocator = MyAllocator;

use cpp_new_simple_cpp_lib::*;

#[test]
fn new() {
    let _guard = MUTEX.lock();

    let pre_count = ALLOC_COUNT.with(|x| {
            x.borrow_mut().load(Ordering::SeqCst)
        });
    let foo = safe_create_foo();
    let post_count = ALLOC_COUNT.with(|x| {
            x.borrow_mut().load(Ordering::SeqCst)
        });
    assert_eq!(pre_count + 1, post_count);

    assert_eq!(foo.i, 42);
    assert_eq!(foo.f, 42.0);
    assert_eq!(foo.c, 42);

    let pre_count = FREE_COUNT.with(|x| {
            x.borrow_mut().load(Ordering::SeqCst)
        });
    safe_destroy_foo(foo);
    let post_count = FREE_COUNT.with(|x| {
            x.borrow_mut().load(Ordering::SeqCst)
        });
    assert_eq!(pre_count + 1, post_count);
}

#[test]
fn array_new() {
    let _guard = MUTEX.lock();

    let pre_count = ALLOC_COUNT.with(|x| {
            x.borrow_mut().load(Ordering::SeqCst)
        });
    let foos = safe_create_foos(512);
    let post_count = ALLOC_COUNT.with(|x| {
            x.borrow_mut().load(Ordering::SeqCst)
        });
    assert_eq!(pre_count + 1, post_count);

    for foo in foos.iter() {
        assert_eq!(foo.i, 42);
        assert_eq!(foo.f, 42.0);
        assert_eq!(foo.c, 42);
    }

    let pre_count = FREE_COUNT.with(|x| {
            x.borrow_mut().load(Ordering::SeqCst)
        });
    safe_destroy_foos(foos);
    let post_count = FREE_COUNT.with(|x| {
            x.borrow_mut().load(Ordering::SeqCst)
        });
    assert_eq!(pre_count + 1, post_count);
}
