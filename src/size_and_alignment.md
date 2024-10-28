# Size and Alignment
Allocating a Rust object within Java to pass to Rust functions requires
respecting the type’s **size and alignment**. If the space allocated is
too small that leads to buffer overflows or overreading, but another property is alignment.

An alignment of 2 means that that type must be addressed at an address
that is a multiple of 2. For instance, a 16-bit integer on x86 has an
alignment of 2, and so if you try to load a 16-bit integer from say the address
0x7ffff01, the CPU will throw an exception because that number is not a multiple of 2.
x86 is a little less picky than most other architectures, with the highest
alignment being 4 bytes[^bytes], but ARM and most other RISCs align a type to its
size. This all means that Java needs to know the alignment of a type in order
to allocate space for it somewhere. 

Some Rust types have well-known
alignments due to matching one-to-one with types defined in the ISA, but
most Rust types have compile-time undefined layout. However, Rust does
provide the compile-time constant functions `core::mem::size_of::<T>()` and
`core::mem::align_of::<T>()` for querying the size and alignment of a type.
Unfortunately, types are not guaranteed to maintain their layout across
compilations, especially if the compiler version were to change. Therefore, calls to
these functions must be made in the same compiled library as all users of
them. 
[^bytes]:Technically, SIMD vectors have higher alignment
with certain instructions.
