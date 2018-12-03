# Rust C++ New

[![Build status](https://api.travis-ci.org/repositories/sheredom/rust_cpp_new.svg)](https://travis-ci.org/sheredom/rust_cpp_new)

This Rust crate maps the C++ new/delete/new[]/delete[] operators via extern "C"
functions into Rust's allocators.

## How to Use

Just include the crate like:

```
extern crate cpp_new;
```

And it'll remap the C++ new/delete to Rust's global allcators.

## Where to Use

Where you want to use this is if you have some pre-existing C++ library that you
do not control, and you want to ensure the allocations being done in the library
are being tracked by Rust's global allocator.

## License

This code is licensed under the
[CC0 1.0 Universal](https://creativecommons.org/publicdomain/zero/1.0/) license,
which is a permissible public domain license.
