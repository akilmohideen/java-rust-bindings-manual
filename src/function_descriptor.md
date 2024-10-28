# Function Descriptor
FunctionDescriptor represents the signature of a function.  
`FunctionDescriptor.of(MemoryLayout, … )` takes a variadic[^variadic] input of
[`MemoryLayouts`](memory_layout.md). The first argument is the memory layout of the return
type, and the rest correspond to the memory layouts of the function
arguments. 

For example, `int foo(float, void*)` would be represented as
`FunctionDescriptor.of(`[`ValueLayout`](value_layout.md)`.JAVA_INT, ValueLayout.JAVA_FLOAT, ValueLayout.ADDRESS)`

For void functions,
`FunctionDescriptor.ofVoid(MemoryLayout, … )` is a static method that is
exactly the same as `FunctionDescriptor.of(MemoryLayout, … )` except that its
first argument corresponds to the first function argument rather than the
return value. 

For example, `void foo(float, void*)` would translate to
`FunctionDescriptor(ValueLayout.JAVA_FLOAT, ValueLayout.ADDRESS)`

For additional information on `FunctionDescriptor`, visit Oracle's [official documentation](https://cr.openjdk.org/~asotona/JDK-8308753-preview/api/java.base/java/lang/foreign/FunctionDescriptor.html).

[^variadic]:The function can take a variable amount of arguments
