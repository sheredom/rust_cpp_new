#include <cstdlib>

struct Foo {
    int i;
    float f;
    char c;

    Foo() : i(42), f(42.0f), c(42) {}
};

extern "C" Foo* create_foo() {
    return new Foo;
}

extern "C" void destroy_foo(Foo* foo) {
    delete foo;
}


extern "C" Foo* create_foos(size_t size) {
    return new Foo[size];
}

extern "C" void destroy_foos(Foo* foos) {
    delete[] foos;
}
