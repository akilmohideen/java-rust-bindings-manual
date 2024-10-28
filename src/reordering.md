# Compiler and Hardware Reordering

## Compiler Reordering
Rust’s compiler makes many optimizations to reduce the number of
operations the CPU will actually have to process. Sometimes it may as well
just remove operations. For example:
```rust,ignore
let x: i32 = 1;
x = 2;
x = 3;
```
The compiler would remove the second line, `x = 2`, because it does not
change the result. The code will still define `x`, initialize it as an i32 variable
with value 1, and end with x having the value 3. However, if the result is not
used, the compiler is likely to completely remove all mentions of x. Why
bother generating code and allocating stack space for a value nobody will
notice is missing?

Rust uses the LLVM compiler infrastructure as its backend, the same thing
that the clang C compiler and clang++ C++ compiler use to generate
machine code. LLVM is very smart, and will do things such as delete dead
code, reorder operations to better saturate out-of-order CPUs, merge
redundant operations (`x += 1; x += 1` will be transformed to `x += 2`), keep
things in registers rather than ever touching memory, turn loops of normal
arithmetic into loops using SIMD/vector instructions. The point is, it is not clear 
what the code is actually going to look like. The only thing that is guranteed is 
that the compiler isn’t allowed to reorder things like print statements around each
other, or move `x += 1` to after a function call that uses `x`. 

However, if there is access to another thread, these changes can be observed (with raw
pointers at least, Rust won’t normally let you do this sort of thing without
synchronization for a reason). So when multithreading, the developer must be explicit
to the compiler: “I want all writes performed before this point to be visible
before this operation, so other threads see what I want them to see”. That’s
where [atomics](atomics.md) come into play.

## Hardware Reordering
Despite compiler reordering, depending on the hardware 
architecture, some operations may be done in a different order by the CPU. This may 
be the case due to how memory is accessed internally. Global memory can be
accessible everywhere but is slow, and cache memory is localized and faster.
Programs may have different threads running at the same time. Rust
guarantees that in each thread, the ordering will be correct. Despite that,
having different memory access speeds means that if two threads are
accessing memories that are vastly different in retrieval speed, the order in
which those threads run operations may be in the wrong order relative to
each other. If you now take a wrapper
class into consideration, ordering might be
thrown off even more. In these cases, Rust and Java’s atomic design 
will put more strain on hardware by stalling some threads so that order guarantees
are kept.

To learn more about reordering, it is recommended to read the official Rust 
resource *The Rustonomicon* chapter [8.3](https://doc.rust-lang.org/nomicon/atomics.html).
