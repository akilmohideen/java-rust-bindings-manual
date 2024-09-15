## Memory Management in Java


In Java, you need to manage memory explicitly when dealing with Rust structures that include lifetimes or references. Use `Arena.ofConfined()` to allocate memory that is tied to the scope of an operation, ensuring that it is freed when no longer needed.

Example:

```java
try (Arena arena = Arena.ofConfined()) {
    MemorySegment mem = arena.allocate(ValueLayout.JAVA_INT);
    mem.set(ValueLayout.JAVA_INT, 0, 42);
    
    MethodHandle fooUpdate = Global.linker.downcallHandle(
        Global.lib.find("foo_update").orElseThrow(),
        FunctionDescriptor.ofVoid(ValueLayout.ADDRESS, ValueLayout.ADDRESS)
    );
    
    fooUpdate.invokeExact(foo, mem);
}
```