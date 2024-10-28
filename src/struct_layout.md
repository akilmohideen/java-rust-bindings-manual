# Struct Layout
A `StructLayout` represents the layout of a C-style struct, including the layout
of all its members, all their members (if applicable), and so on. It does
exactly the same job as a struct definition in C. The class itself has no
interesting methods, but you can create a StructLayout using
`MemoryLayout.structLayout(MemoryLayout…)`. To translate the following
structs to the Java [FFM API](https://openjdk.org/jeps/454), we would use the
following Java code:

C:
```c
struct foo {
    int num;
    char* string;
    struct bar baz;
}
```
Java:
```java
StructLayout bar = …;
StructLayout foo = MemoryLayout.structLayout(
    ValueLayout.JAVA_INT.withName(“num”),
    ValueLayout.ADDRESS.withTargetLayout(0,
        ValueLayout.JAVA_BYTE).withName(“string”),
    bar.withName(“baz”)
);
```
The `.withName(String)` method allows you to later retrieve a [`VarHandle`](variable_handle.md)
using that name, covered in the `VarHandle` section.
Constructing a `StructLayout` like this will automatically generate the
appropriate total [size and alignment](size_and_alignment.md), as well as member offsets and padding
that C would add on this platform. Generally, the size is greater than or
equal to the sum of the sizes of the members (making room for padding as
necessary to keep all members aligned) and the alignment is the maximum
of the member alignments. Some exotic C programs may use overaligned
structs[^note], for which you can add a final
`.withAlignment(alignment)` to override the automatic alignment calculated by
Java.

This all still applies to Rust, but only on:
1. `#[repr(C)]` structs
1. `#[repr(C)]` tuple structs[^tuple]
1. `#[repr(integer type)]` enums with only valueless variants
1. enums with
exactly one nonnullable `#[repr(C)]` variant and up to one zero-sized variant[^note2]
1. `#[repr(transparent)]` structs and tuple structs with exactly one `#[repr(C)]` member and all other members being zero-sized

`#[repr(C)]` requires all members, and members of members, and members of those members, etc. to be `#[repr(C)]` as well, which is very
invasive to code. For the sake of performance, some may choose to do this,
but it also greatly limits what you can use in the standard library.
Common non `#[repr(C)]` types include: 
1. `Vec`
1. `String`
1. `&str`
1. slices
1. anonymous
1. tuples
1. `dyn` references
1. `Box<dyn T>`
1. most enums with a variant that holds a value (`Option<T>` for most `T`)
1. all enums with more than one variant that holds a value
1. every single container type[^container]

If a type uses any of these types (and most types from external libraries too) by
value, that type cannot be `#[repr(C)]`. The only way around this restriction
is through pointer indirection, like `Box<T>`[^box], because pointers are always
representable even if the thing they are pointing to is not. People wanting
every last ounce of performance can deal with this, but the average Rust
type cannot, and so it cannot be represented as a `StructLayout` or a
[`MemoryLayout`](memory_layout.md). The last class important specifically to `StructLayout` is `PaddingLayout`. This is the layout of padding in StructLayouts. It exists purely to pad
the struct.

For more information on `StructLayout`, visit Oracle's [official documentation](https://docs.oracle.com/en/java/javase/22/docs/api/java.base/java/lang/foreign/StructLayout.html).


[^note]:Many compilers accept `__attribute__((aligned(x)))` to align a struct
to `x`, or they keep its original alignment if `x` is less than or equal to that. Rust
has `#[align(x)]` to specify overalignment.
[^tuple]:Tuple structs are just structs with anonymous members.
[^note2]:This case exists pretty much purely to allow Option<reference> to be
exchanged as a nullable pointer
[^container]:`VecDeque`, `HashMap`, `HashSet`,
`BTreeMap`, `BTreeSet`, every iterator in the entire standard library, every IO
type, every FS type (including `File`), `Rc`, `Arc`, `RefCell`, `RwLock`, `Mutex`.
[^box]:Still doesn’t work for `dyn`, use
`ThinBox` for that. `Box<T>` is guaranteed to be represented by just a pointer,
semantically like one returned from malloc.
