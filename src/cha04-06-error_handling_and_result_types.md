## Error Handling and Result Types

Rust’s `Result<T, E>` types map naturally to Java’s exception handling. You can translate Rust errors into Java exceptions or use Java’s `Optional` for `Option` types.

Example:

```rust
#[no_mangle]
extern "C" fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 {
        Err("division by zero")
    } else {
        Ok(a / b)
    }
}
```

In Java:
```java
try {
    int result = (int) divideHandle.invokeExact(10, 2);
} catch (Throwable e) {
    System.out.println("Error: " + e.getMessage());
}
```
