# Data Races
Data races occur when multiple threads try to access the same 
[memory segment](memory_segment.md), trying to write to it, and they can cause undefined behavior. Safe
Rust guarantees that no data races will occur, and a big player for this is the
[ownership](ownership.md) model. By definition, if a value can only have one owner 
(can make changes), then it can only be written to by its single owner. However,
general race conditions are not prevented in Rust. They simply can’t be
prevented from a mathematical standpoint, due to the way the scheduler
works in different operating systems. This is something that is out of the 
developer's'
control. This means that while a program may get deadlocked, or have
incorrect synchronization, a Rust program will still be safe.

To learn more about subtyping and variance, it is recommended to read the official Rust 
resource *The Rustonomicon* chapter [8.1](https://doc.rust-lang.org/nomicon/races.html).
