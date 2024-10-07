# Unwinding
With the default panic handler (fittingly named “unwind”), when Rust code
calls `panic!()` Rust will begin walking local variables in the call stack to drop
them, then kill the thread. If the type is mutably shared across threads,
such as with a `Mutex<T>` does, then it may be in an inconsistent state,
though it should not be necessary to have a custom type doing that. However, What
is a concern is Rust calling drop on some types while they’re potentially in
inconsistent states. For example, say a `JavaRef<T>` type is used to represent a
reference held by Java. If it is busy updating its pointer for instance, and it
panics in that function, Rust’s unwinding will eventually call `drop()` on it, so
now the drop code is working with a `JavaRef<T>` with an invalid pointer.
Rust does have another panic handler called “abort” which just prints a stack
trace and aborts the process, which might be a better option if the types being used
are not believed to be unwind-safe.

## Example of Unwinding
Unwinding is implicit in Rust's error handling if a panic occurs. For explicit
handling:
```rust,ignore
match calculator.evaluate(tokens) {
    Ok(result) => println!("Result: {}", result),
    Err(e) => println!("Error: {}", e),
}
```

To learn more about unwinding, it is recommended to read the official Rust 
resource *The Rustonomicon* chapter [7](https://doc.rust-lang.org/nomicon/unwinding.html).
