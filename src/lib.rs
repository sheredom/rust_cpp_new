use std::alloc::alloc;
use std::alloc::dealloc;
use std::alloc::Layout;
use std::cmp::max;
use std::isize;
use std::mem::align_of;
use std::mem::size_of;
use std::os::raw::c_void;
use std::ptr::null_mut;

const LAYOUT_ALIGNMENT: usize = align_of::<Layout>();
const LAYOUT_SIZE: usize = size_of::<Layout>();

#[no_mangle]
pub unsafe extern "C" fn rust_cpp_new(
    size: usize,
    alignment: usize,
) -> *mut c_void {
    let offset = max(LAYOUT_ALIGNMENT, LAYOUT_SIZE);
    let new_alignment = max(offset, alignment);
    let new_size = size + new_alignment;

    if new_alignment >= (isize::MAX as usize) {
        null_mut()
    } else {
        match Layout::from_size_align(new_size, new_alignment) {
            Ok(layout) => {
                let allocation = alloc(layout);
                let result_allocation =
                    allocation.offset(new_alignment as isize);
                let hidden_allocation =
                    result_allocation.offset(-(offset as isize)) as *mut Layout;
                hidden_allocation.write(layout);

                result_allocation as *mut c_void
            }
            Err(_) => null_mut(),
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn rust_cpp_delete(payload: *const c_void) {
    let offset = max(LAYOUT_ALIGNMENT, LAYOUT_SIZE);

    if !payload.is_null() {
        let result_allocation = payload as *mut u8;
        let hidden_allocation =
            result_allocation.offset(-(offset as isize)) as *const Layout;
        let layout = hidden_allocation.read();
        let allocation = result_allocation.offset(-(layout.align() as isize));
        dealloc(allocation, layout);
    }
}

#[link(name = "cpp_new")]
extern "C" {}

#[cfg(test)]
mod tests {
    extern crate cpp_new_simple_cpp_lib;

    use self::cpp_new_simple_cpp_lib::*;

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
}
