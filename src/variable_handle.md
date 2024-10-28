# Variable Handle
A `VarHandle` represents a handle to a sub-layout given a layout. It helps
solve the problem of, say, accessing an `int` field of a struct, or accessing
an element of an array. Variable handles are used to construct a path to a value
that needs to be given a certain layout (basically a type). Say there is a pointer to
an array of struct `foo`, which has an integer member `x` that must be read.
This is how to construct a `VarHandle` to get `x` from any such
pointer:
```java
MemoryLayout layoutOfPointer =
ValueLayout.ADDRESS.withTargetLayout(
    MemoryLayout.sequenceLayout(arrayLen,
        MemoryLayout.structLayout(
            ValueLayout.JAVA_INT.withName(“x”),
            ValueLayout.JAVA_INT.withName(“y”),
        )
    )
);
VarHandle xHandle = layoutOfPointer.varHandle(
    PathElement.dereferenceElement(),
    PathElement.sequenceElement(),
    PathElement.groupElement(“x”)
);
```
Now whenever `x` is needed from this kind of pointer,  call
`(int)xHandle.get(`[`MemorySegment`](memory_segment.md)`, 0, index)`.

For more information on `VariableHandle`, visit Oracle's [official documentation](https://docs.oracle.com/en/java/javase/22/docs/api/java.base/java/lang/invoke/VarHandle.html).
