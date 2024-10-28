# Symbols, Extern, Generics
By default, Rust functions have an undefined **application binary interface** (ABI), thus 
they are incompatible with what C expects. Rust functions also have mangled symbol
names[^mangle]. To
guarantee a C ABI (assuming the types
themselves are C ABI compatible, the next
section provides details on that), the function declaration must be prefixed with
`extern “C”`. So \
`extern “C” foo(number: i32) -> i32` would be equivalent to the
C function `int foo(int number)`. To guarantee the symbol name is that of the
function name, like in C, you must annotate the function with the
`#[no_mangle] attribute`.

However, this does not cover functions with **generic types**. Rust allows
creating functions that act on unknown types, so that a fucntion like \
`fn add<T: Add<Output=T>>(a: T, b: T) -> T { return a + b;
}` can be reused
 with any type as long as it implements `Add`. How does the same function
handle multiple types? On the machine code level it doesn’t, that’s why
functions are first monomorphized, creating a version of the function for
every used combination of generic types. Calling `add(1u32, 1u32)` would
generate a function equivalent to `fn add(a: u32, b: u32) -> u32`, whereas
calling `add(1u8, 1u8)` would generate `fn add(a: u8, b: u8) -> u8`. 

Java cannot see generic functions, it only sees monomorphized functions that
exist in the [shared object file](so.md). Rust only generates monomorphizations for
types that are used in that function, so if the Rust library code does not use `fn
add<T: Add<Output=T>>(a: T, b: T) -> T` at all, there are no used generic
types and thus the compiler does not generate anything related to that
function. Even if it did, it can not possibly support every type a programmer
might use, especially if a function had multiple type parameters. `fn foo<A,
B>()` would require the square of the number of possible types. The best thing to do without using `dyn pointers` is enforcing
wrapper functions without generic parameters: 
`fn add_u32(a: u32, b: u32) -> u32 { return add::<u32>(a, b); }`.

Specifying `dyn` references in a type instructs the Rust compiler to use **fat
pointers** - pointers that store the normal pointer as well as a pointer to a
vtable containing methods that can be called on the pointer. This works
almost exactly like in C++ with exactly the same tradeoff. There is only
one function in the final binary (no monomorphization needed) but it is not
specialized for a type (so no automatic vectorization on integers for
instance). Additionally, it requires dereferencing the pointer to the vtable, as well as
that function then needing to dereference the real pointer once it is called. 
This can lead to memory access / cache missing overhead. 

It also breaks a common
idiom: `Vec<T>`. `&dyn Vec<T>` can be done, but chances are `T` will need to be 
accessed. If `Vec<&dyn T>` is used, there will be 
[lifetime](lifetimes.md) issues and it will be necessary to
restructure everything that touches the vector to deal with `Vec<&dyn T>`,
even if they otherwise could have used the easier `Vec<T>`. The biggest issue
with using `dyn`, however, is that some trait methods simply do not work with
`dyn`. [*The Rust Reference*](https://doc.rust-lang.org/nightly/reference/items/traits.html#object-safety)
specifies the conditions that are required for a method to be object-safe: it
must not return `Self` directly[^self] (the compiler doesn’t know the ABI layout of a function with an unknown return type), it must
not take `Self` as an argument directly[^arg], and it must not use any generics beyond `Self`[^gen]. 

A final issue with `dyn` is that fat pointers do not
have a stable ABI. There is an experimental feature,
`ptr_metadata`, that allows splitting the pointer and its metadata as well as
creating a fat pointer from a raw pointer and metadata. Although, the Metadata
is not object safe. `DynMetadata<dyn T>` may have a stable representation
for different `T`[^t], but it requires lots of transmuting to make that work and it 
might technically be undefined behavior. Ultimately, `dyn` **saves some code size** at
the expense of **poor ergonomics**, using confusing experimental
Rust features, and performance. Therefore, a developer might just be
better off writing everything in Java instead of trying to interoperate with
Rust code.

[^mangle]:This means the symbol name for a function can not be known.
[^self]:This is because the compiler does not know the ABI layout of a function with 
an unknown return type.
[^arg]:This is because the compiler does not know the
ABI layout of a function with unknown argument types.
[^gen]:Ditto ABI of arguments.
[^t]:This is needed for passing it to Java through the C ABI.
