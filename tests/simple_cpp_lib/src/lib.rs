#[repr(C)]
pub struct Foo {
  pub i: i32,
  pub f: f32,
  pub c: i8
}

#[link(name = "cpp_new_simple_cpp_lib")]
extern "C" {
  fn create_foo() -> *mut Foo;
  fn destroy_foo(foos: *const Foo);
  fn create_foos(size: usize) -> *mut Foo;
  fn destroy_foos(foos: *const Foo);
}

pub fn safe_create_foo<'a>() -> &'a mut Foo {
  unsafe { &mut *create_foo() }
}

pub fn safe_destroy_foo<'a>(foo: &'a Foo) {
  unsafe { destroy_foo(foo) }
}

pub fn safe_create_foos<'a>(size: usize) -> &'a mut [Foo] {
  unsafe { ::std::slice::from_raw_parts_mut(create_foos(size), size) }
}

pub fn safe_destroy_foos<'a>(foos: &'a [Foo]) {
  unsafe { destroy_foos(foos.as_ptr()) }
}
