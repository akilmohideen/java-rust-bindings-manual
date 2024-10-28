# Orderings

## Sequentially Consistent
As its name suggests, operations that are sequentially consistent will be
executed sequentially. In other words, it guarantees that the execution of a
program with multiple threads behaves as if each thread’s operations
occurred in a certain order, without any [reordering](./reordering.md) or 
interleaving. This
means that if thread A is supposed to write to value x before thread B writes
to value x, B will only be able to write to value x once A has written to it. It
is implemented by using memory barriers: they are protecting x from B, and
they are only letting their guards down once A has written to it. Compiler and
hardware reordering makes a big difference in performance, so by restricting
the program in these fields, performance tends to suffer.

## Acquire and Release
Acquire and release work closely together, and they do so by acquiring locks
and releasing locks. This is similar to how locks are used in real life to
shut a door. On the outside anything can happen, but once a room is entered through a 
door, the space there is completely separated from the
outside. In ordering, this means that any operations that are written after a
lock is acquired can not be reordered, and the whole block of code will be
executed sequentially in relation to the “outside world”. Once the block of
code is executed and the lock is released, all operations that come after that
are free to be reordered.

## Relaxed
Relaxed [data accesses](./data_accesses.md) can be reordered freely, and doesn’t 
provide the
“happens before” relationship. Despite that, they are still [atomic](./atomics.md), 
and they
are used when a section needs to be executed without its order really
mattering. For example, using `fetch_add()` is a safe way of writing to a
counter (incrementing its value), assuming the counter isn’t used to
determine other accesses.

To learn more about orderings, it is recommended to read the official Rust 
resource *The Rustonomicon* chapter [8.3](https://doc.rust-lang.org/nomicon/atomics.html).
