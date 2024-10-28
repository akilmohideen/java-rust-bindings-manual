# Union Layout
`UnionLayout` represents a C union. Much like a C union, it is used to specify and
access the different members like it was a struct. However, only one of
those members exists at any one time. You can create a `UnionLayout` with
[`MemoryLayout`](memory_layout.md)`.unionLayout(MemoryLayout…)`. Just like in C, a
[`MemorySegment`](memory_segment.md) referencing a `UnionLayout` can be treated as actually referencing the
layout of one of its members, such as by calling `.get()` with the associated
`MemoryLayout`. 

Alternatively, [Variable Handles](variable_handle.md) can be used to
reference members
in a process similar to that used in C.
Generally, union layouts will have a [size](size_and_alignment.md) equal to the maximum size of its
members and an [alignment](size_and_alignment.md) equal to the maximum alignment of its
members. Similarly to structs, unions can be overaligned, which can be
specified by adding `.withAlignment(alignment)` to the end of the method
chain to overwrite Java’s automatically-determined alignment for that type.

For more information on `UnionLayout`, visit Oracle's [official documentation](https://docs.oracle.com/en/java/javase/22/docs/api/java.base/java/lang/invoke/MethodHandle.html).
