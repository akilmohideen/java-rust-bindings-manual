# Memory Segment
`MemorySegment` represents a fat pointer, that is, a pointer with associated
bounds information, much like a mutable slice in Rust. The main method
associated with memory segments is `.get(`[`MemoryLayout`](memory_layout.md)`, offset)`, which
indexes offset amount into the pointer and reads whatever memory is there
as if it’s of the associated type. 

For instance,
`segment.get(`[`ValueLayout`](value_layut.md)`.JAVA_INT, 1)` is basically the same as C code doing
`((int*)segment)[1]`. The only difference from the C code is that Java will
throw an exception if the program attempts to access an index outside of the
bounds associated with the `MemorySegment`. The most common sources of
MemorySegments are functions returning pointers. `MemorySegments`
returned to Java through the foreign function interface will automatically be
assigned a length of zero, since Java does not have enough information to
determine the bounds. However, invoking the `.reinterpret(size)` method will
edit the bounds information. This is extremely unsafe and must
be used with caution. Assigning a logically incorrect bound
could allow normal Java code to cause a segmentation fault (or worse).

Finally, like Rust slices, `MemorySegments` can be subsliced using
`.asSlice(offset, size)`, which is also bounds-checked, returning a new slice
with the associated pointer and length values and the same [lifetime](lifetimes.md) as the
original.

For more information on `MemorySegment`, visit Oracle's [official documentation](https://docs.oracle.com/en/java/javase/22/docs/api/java.base/java/lang/foreign/MemorySegment.html).
