# Memory Layout

Memory Layouts can be used in
order to streamline the allocation of off-heap memory. Here is an overview
of how `MemoryLayout` differs from [`MemorySegment`](memory_segment.md).

Assume an array of structs needs to be declared for the following example. First an
[`Arena`](arenas.md) must be created, any arena type desired will do. Next a
`MemoryLayout.sequenceLayout()` can be used, with arguments n, that reflect the length
of the array, and `MemoryLayout.structLayout()`, that takes in the 
[value layouts](value_layout.md) and names of elements within the struct. After this, create
[`VarHandles`](variable_handle.md) for each element within the struct, which create a reference for
each respective element. Then create a `MemorySegment` that
corresponds to the entire memory layout of the array, and allocate it to the
appropriate arena, and finally the structs can be accessed.

For additional information on `MemoryLayout`, visit Oracle's [official documentation](https://cr.openjdk.org/~mcimadamore/jdk/FFM_22_PR/javadoc/java.base/java/lang/foreign/MemoryLayout.html).
