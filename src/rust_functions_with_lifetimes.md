# Rust Functions with Lifetimes

Lifetimes ensure that references in Rust are valid as long as they are needed. When exposing such functions to Java, care must be taken to ensure that the references remain valid and are not prematurely deallocated by Java’s garbage collector (GC).

```rust
struct Foo<'a> {
    ptr: Option<&'a i32>,
    number: i32,
}

impl<'a> Foo<'a> {
    #[no_mangle]
    extern "C" fn new() -> Box<Self> {
        Box::new(Foo { ptr: None, number: 42 })
    }

    #[no_mangle]
    extern "C" fn update(&'a mut self, ptr: &'a i32) {
        self.ptr = Some(ptr);
    }

    #[no_mangle]
    extern "C" fn get(&self) -> i32 {
        *self.ptr.unwrap_or(&-1)
    }
}
```
