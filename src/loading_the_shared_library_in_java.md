# Loading the Shared Library in Java

In Java, use SymbolLookup and Linker to load the shared library and access the Rust functions.

```java
import java.lang.foreign.ValueLayout;
import java.lang.foreign.SymbolLookup;
import java.lang.foreign.Linker;
import java.lang.invoke.MethodHandle;
import java.lang.foreign.Arena;
import java.lang.foreign.FunctionDescriptor;
import java.lang.foreign.MemorySegment;

public class RustBindings {
    static {
        SymbolLookup lib = SymbolLookup.libraryLookup("librustlib.so", Arena.global());
        Linker linker = Linker.nativeLinker();

        MethodHandle addHandle = linker.downcallHandle(
            lib.find("add").orElseThrow(),
            FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.JAVA_INT, ValueLayout.JAVA_INT)
        );
    }

    public static void main(String[] args) throws Throwable {
        // Example usage
        int result = (int) addHandle.invokeExact(3, 4);
        System.out.println("Result: " + result);
    }
}
```
