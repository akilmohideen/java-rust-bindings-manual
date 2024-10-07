# Lifetimes
Rust constantly wants to know "what exactly is that reference referencing?". Most 
things don’t live
forever, so Rust also checks that developers don’t try to use it or reference it 
after it has been moved. A move is a change in ownership which potentially means
physically moving it in memory and invalidating any pointers to it. `drop()`,
for instance, takes ownership of an object so it can kill it. Anyone familiar
with pointers in C has a decent understanding of the concept of
pointer lifetimes: do not use the pointer after the object has been deleted or
moved. As long as a shared reference exists, no mutable references may exist
and the object must not be moved; and as long as a mutable reference
exists, no other references may exist and the object must not be moved.
The compiler enforces a more stringent test on safe code, that breaking
those rules must provably never happen, leading to some cases where you
know it will not happen, yet the compiler can not prove it, so it does not allow 
it. Luckily we do not need to follow the compiler’s test, we only need to follow
those simple rules.

Unfortunately, for arbitrary code the lifetimes involved can get quite
intricate. `fn foo<’a>(input: &’a) -> TypeWithLifetime<’a>` creates a
transitive relationship between the lifetime of input and
`TypeWithLifetime<’a>`. While we may be able to enforce a simple one-to-one
lifetime relationship, it’s unclear if we can feasibly enforce that A lives as
long as B lives as long as C lives as long as D lives as long as… Certainly, if it
requires invasive changes to types crossing the FFI boundary, such as every
reference in every struct needing to be converted to a `RefCell<&T>`, that
would be very inconvenient for users.

## Example of Lifetimes
The code does not explicitly use annotated lifetimes because it does not
require them due to its simplicity. However, the concept is there implicitly:
```rust,ignore
struct PostfixCalculator {
    stack: VecDeque<f64>,
}

impl PostfixCalculator {
    fn new() -> Self {
        PostfixCalculator {
            stack: VecDeque::new(),
        }
    }
}

fn evaluate(&mut self, tokens: Vec<&str>) -> Result<f64, String>
{
    // use of `self` which has an implicit lifetime
}
```
This example implicitly uses lifetimes to ensure that references within the
evaluate function do not outlive the PostfixCalculator instance they
reference. Rust's lifetime elision rules automatically handle this in most
cases, but explicit lifetime annotations can be used for more complex
scenarios.

To learn more about lifetimes, it is recommended to read these official Rust 
resources: *The Rust Programming Language* chapter 
[10.3](https://rust-book.cs.brown.edu/ch10-03-lifetime-syntax.html), and *The Rustonomicon* chapter [3.3](https://doc.rust-lang.org/nomicon/lifetimes.html).
