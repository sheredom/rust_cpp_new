#include <cstdlib>
#include <new>

extern "C" {
    void* rust_cpp_new(size_t size, size_t alignment);
    void rust_cpp_delete(void* const payload);
}

void* operator new(size_t size) throw(std::bad_alloc) {
    return rust_cpp_new(size, 16);
}

void* operator new[](size_t size) throw(std::bad_alloc) {
    return rust_cpp_new(size, 16);
}

void operator delete(void* const payload) throw() {
    rust_cpp_delete(payload);
}

void operator delete[](void* const payload) throw() {
    rust_cpp_delete(payload);
}
