# Method Handle
MethodHandle is where one of the most essential tools in the [FFM API](https://openjdk.org/jeps/454)
. The absolute most important method on MethodHandles returned from the `Linker` is `invokeExact(…)`.
`.invokeExact(…)` takes in the parameters of the function according to the
[`FunctionDescriptor`](function_descriptor.md) and returns a value with type also specified by the
`FunctionDescriptor`. Java will throw an exception at runtime if the arguments
passed to the method do not match up with the `FunctionDescriptor`. Because
of some Java Virtual Machine details, the return
value must also be explicitly cast to the expected return type. Otherwise, Java will once again throw an exception at
runtime, this time because the return type was wrong. A
function with signature `FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.JAVA_FLOAT)` would be called like so:  
 `int returnValue =
(int)handleName.invokeExact(myFloat)`.

For more information on `MethodHandle`, visit Oracle's [official documentation](https://docs.oracle.com/en/java/javase/22/docs/api/java.base/java/lang/invoke/MethodHandle.html).
