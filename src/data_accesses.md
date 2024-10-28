# Data Accesses
Another way the [atomicity model](./atomics.md) Rust employs deals with providing 
strong guarantees is by introducing the concept of causality and providing tools to
establish relationships between different parts of a program and the threads
executing them. One of these, and potentially the most important, is the “happens
before” relationship. It defines the order of a program: if there is a statement
1 and statement 2, and there is a relationship of “statement 1 happens
before statement 2”, then statement 1 will be run before statement 2. This
provides extra information to the compiler and hardware about the ordering
of the operations, and allows for bigger optimizations on
operations that are not affected by the order they are executed in. Data accesses are
unsynchronized, which allows compilers to move them around as much as
they want to optimize performance, especially if the program is
single-threaded. The downside is that it can cause [data races](./data_races.md), 
which results in undefined behavior. Atomic accesses tell the compiler and hardware 
that the program is multi-threaded. They are marked with an ordering, which
limits how the [compiler and hardware can reorder](./reordering.md) these statements.
In Rust,there are four types of [orderings](./orderings.md): sequentially consistent, 
release, acquire,
relaxed.

To learn more about data accesses, it is recommended to read the official Rust 
resource *The Rustonomicon* chapter [8.3](https://doc.rust-lang.org/nomicon/atomics.html).
