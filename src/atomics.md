# Atomics
Atomics are types that support operations in a thread-safe manner without
external synchronization. For example, consider wanting to use a counter,
`foo`, you want to use across different threads. It would not be safe to
increment the counter using `foo++`, because that could result in a race
condition: different threads trying to increment `foo` by one will cause
undefined behavior. Locking can be used to make sure one thread
increments the value of `foo` by one, and then the other one, but it has
severe performance costs. Let’s say at first, `foo = 0`. Then, after both
threads write to it, `foo = 2` should be true. The way atomics would handle
this is: both threads would check if the value of `foo` is 0, and if it is,
increment to 1, otherwise, reevaluate. This would ensure that, no matter the
order the operating system decides to call these operations, at the end, `foo`
will be 2. Rust makes it very easy to work with atomics, for `foo`, just
write: 

`let foo = Arc::new(AtomicUsize::new(0));`

To learn more about atomics, it is recommended to read the official Rust 
resource *The Rustonomicon* chapter [8.3](https://doc.rust-lang.org/nomicon/atomics.html).
