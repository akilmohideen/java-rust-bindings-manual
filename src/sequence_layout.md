# Sequence Layout
`SequenceLayout` represents the layout of arrays. To create a
`SequenceLayout`, call [`MemoryLayout`](memory_layout.md)`.sequenceLayout(numberOfElements,
MemoryLayout)`. There is no get method or any direct way to get the nth
element of an array. Instead, create a special [`VarHandle`](variable_handle.md) to the needed
data within the member, then call get on that with the index. For instance, to
get the x-coordinates of the structs in an array, use:
```java
SequenceLayout arrayOfStruct = MemoryLayout.sequenceLayout(10,
    MemoryLayout.structLayout(
        ValueLayout.JAVA_INT.withName(“x”),
        ValueLayout.JAVA_INT.withName(“y”)
    ).withName(“struct”)
);
VarHandle varHandle =
arrayOfStruct.arrayElementVarHandle(PathElement.groupElement(“x”));
for (int i=0; i<10; i++) {
    System.out.println(varHandle.get(memorySegment, 
        0L,
        (long)i)
    );
}
```
`SequenceLayout` provides some interesting **methods**.
`sequenceLayout.elementCount()` will, as the name suggests, give the
length of the array, which is useful for passing around slices as it is not necessary to store the length itself. 
`sequenceLayout.reshape(long dim1, long
dim2, …)` and `sequenceLayout.flatten()` are both related to reinterpreting
multidimensional arrays. Multidimensional arrays are just arrays of arrays,
but their layout means they can safely be reinterpreted as a single
dimension array of [size](size_and_alignment.md) `(dim 1 size)*(dim 2 size)*...`, which is exactly what
`sequenceLayout.flatten()` does. `sequenceLayout.reshape` does the inverse of
`sequenceLayout.flatten()`, but is also fallible. Obviously, if an attempt is made to reshape
an array to AxBxC but the array’s length isn’t divisible by A and B and C, this
method will throw an exception. Another nice property of
`sequenceLayout.reshape()` is that one argument may be set to -1, in which
case `sequenceLayout.reshape()` will do the math based on the array’s length
to determine what that dimension must be.

A Java type can be used to act as a wrapper around **Rust slices**, so
`SequenceLayout` would feature heavily in that kind of implementation. While
a slice object, composed of a pointer and a length, is not application binary
interface (ABI) stable, the underlying array is ABI stable.
Rust provides methods to get the pointer and length from a slice, as well as
functions to construct slices from a pointer and a length, so while it is not
ABI safe, it is easy enough to disassemble and
reassemble into safe forms as needed. While it is easier to just keep an
opaque blob of data and ask Rust any time it must be used, it is much
faster for Java to have direct access to the array.

The **Just-In-Time** (JIT) compiler knows how array accesses work, and can optimize
the corresponding Java code, possibly with automatic vectorization which is
a great boost to throughput. In contrast, every time a call is made out to a Rust
function, the JIT compiler has no idea what that function is doing.
This means that it can not optimize the memory accesses, and it must also assume
that the function breaks every optimization assumption it has. For instance, the
function could touch any value in memory, preventing the JIT
compiler from reordering any reads or writes from before the function call to
after the function call, and vise versa. 

The **Rust compiler** has the same issue: it
does not know what the Java code is doing, so there is no way it can optimize
around that such as automatic vectorization either. This does not matter so
much for one-off functions, functions that are only called a few thousand
times, or large functions where execution time is dominated by actually
running the function and not on function call overhead, but for simple code
in loops this can be brutal. And how are arrays typically
used? Usually small bits of code run many times in a loop. The performance
gains are too great to ignore. While doing the loop in Rust will beat Java
almost every time, it is not reasonable for every possible loop body to be put 
in Rust. However, developers have the option to write all of their
loops in Rust if they so choose. Still, `SequenceLayout`
provides a great opportunity to allow easy, direct access to arrays and
array elements for Java.

For more information on `SequenceLayout`, visit Oracle's [official documentation](https://docs.oracle.com/en/java/javase/22/docs/api/java.base/java/lang/foreign/SequenceLayout.html).
