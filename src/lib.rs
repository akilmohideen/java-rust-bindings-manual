/// # Advanced Java Bindings for Complex Rust Libraries
///
/// This module demonstrates how to create bindings between Java and a more complex Rust library.
/// It covers scenarios such as handling static data, generating random numbers, and working with arrays,
/// showcasing how to manage these operations across the Rust-Java boundary.
///
/// ## Overview
///
/// In this example, the Rust library provides several functions related to random number generation,
/// including thread-local random number generators (`ThreadRng`) and shuffling arrays. These functions 
/// involve more complex data structures and operations, necessitating careful memory management 
/// and function linking in Java.
///
/// ## Rust Functions and Static Data
///
/// ### 1. Exposing Static Data and Functions
///
/// The Rust library exposes both static data and functions to Java. Static data, such as the size and 
/// alignment of the `ThreadRng` struct, is exported so that Java can correctly allocate memory for it.
///
/// ```rust
/// use core::mem::{size_of, align_of};
/// use rand::prelude::*;
///
/// #[no_mangle]
/// pub static THREAD_RNG_SIZE: usize = size_of::<ThreadRng>();
/// #[no_mangle]
/// pub static THREAD_RNG_ALIGN: usize = align_of::<ThreadRng>();
///
/// #[no_mangle]
/// unsafe extern "C" fn new_thread_rng(thread_rng: *mut ThreadRng) {
///     thread_rng.write(rand::thread_rng());
/// }
///
/// #[no_mangle]
/// unsafe extern "C" fn thread_rng_f64_gen(thread_rng: &mut ThreadRng) -> f64 {
///     thread_rng.gen()
/// }
///
/// #[no_mangle]
/// unsafe extern "C" fn thread_rng_i64_gen(thread_rng: &mut ThreadRng) -> i64 {
///     thread_rng.gen()
/// }
///
/// #[no_mangle]
/// unsafe extern "C" fn random_f64() -> f64 {
///     rand::random()
/// }
///
/// #[no_mangle]
/// unsafe extern "C" fn shuffle_f64s_from_thread_rng(array: *mut f64, thread_rng: &mut ThreadRng, n: i32) {
///     let arr = core::slice::from_raw_parts_mut(array, n.try_into().unwrap_or_default());
///     arr.shuffle(thread_rng);
/// }
/// ```
///
/// ### 2. Static Data
///
/// Static data such as `THREAD_RNG_SIZE` and `THREAD_RNG_ALIGN` are exported to help Java correctly 
/// allocate memory for the `ThreadRng` struct. These static variables are crucial for ensuring 
/// that Java correctly understands the memory layout of Rust types.
///
/// ### 3. Complex Functions
///
/// The functions `new_thread_rng`, `thread_rng_f64_gen`, `thread_rng_i64_gen`, and `shuffle_f64s_from_thread_rng` 
/// perform operations on the `ThreadRng` struct, generate random numbers, and shuffle arrays. 
/// These functions must be carefully linked and called from Java, ensuring that memory is correctly 
/// managed and that data is safely transferred between the languages.
///
/// ## Compiling the Rust Code to a Shared Library
///
/// As with the previous examples, the Rust code needs to be compiled into a shared library that can 
/// be used by Java:
///
/// ```toml
/// [lib]
/// crate-type = ["cdylib"]
/// ```
///
/// Compile the library using:
///
/// ```bash
/// cargo build --release
/// ```
///
/// This will generate a shared library (`.so`, `.dll`, or `.dylib` depending on your OS) in the `target/release/` directory.
///
/// ## Java Integration
///
/// ### 1. Required Imports in Java
///
/// The following imports are necessary for interacting with the Rust library:
///
/// ```java
/// import java.lang.foreign.ValueLayout;
/// import java.lang.invoke.MethodHandle;
/// import java.lang.foreign.Arena;
/// import java.lang.foreign.FunctionDescriptor;
/// import java.lang.foreign.MemorySegment;
/// ```
///
/// - **`ValueLayout`**: Defines memory layouts for data types, such as `JAVA_DOUBLE` and `JAVA_LONG`, used in function signatures.
/// - **`MethodHandle`**: Represents a dynamically invoked method handle, crucial for calling Rust functions from Java.
/// - **`Arena`**: Manages memory allocation in Java for native operations, ensuring that memory is properly scoped and freed.
/// - **`FunctionDescriptor`**: Describes the signature of a native function.
/// - **`MemorySegment`**: Represents a memory region, essential for managing memory passed between Java and Rust.
///
/// ### 2. Loading the Shared Library and Accessing Static Data
///
/// Java code must load the shared Rust library and access static data to correctly allocate memory:
///
/// ```java
/// static {
///     size = Global.lib.find("THREAD_RNG_SIZE").orElseThrow().reinterpret(8).get(ValueLayout.JAVA_LONG, 0);
///     align = Global.lib.find("THREAD_RNG_ALIGN").orElseThrow().reinterpret(8).get(ValueLayout.JAVA_LONG, 0);
///
///     var newDesc = FunctionDescriptor.ofVoid(ValueLayout.ADDRESS);
///     var newAddr = Global.lib.find("new_thread_rng").orElseThrow();
///     new_ = Global.linker.downcallHandle(newAddr, newDesc);
/// 
///     var genf64Desc = FunctionDescriptor.of(ValueLayout.JAVA_DOUBLE, ValueLayout.ADDRESS);
///     var genf64Addr = Global.lib.find("thread_rng_f64_gen").orElseThrow();
///     gen_f64 = Global.linker.downcallHandle(genf64Addr, genf64Desc);
///
///     var geni64Desc = FunctionDescriptor.of(ValueLayout.JAVA_LONG, ValueLayout.ADDRESS);
///     var geni64Addr = Global.lib.find("thread_rng_i64_gen").orElseThrow();
///     gen_i64 = Global.linker.downcallHandle(geni64Addr, geni64Desc);
///
///     var shufflef64sDesc = FunctionDescriptor.ofVoid(ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.JAVA_INT);
///     var shufflef64sAddr = Global.lib.find("shuffle_f64s_from_thread_rng").orElseThrow();
///     shuffle_f64s = Global.linker.downcallHandle(shufflef64sAddr, shufflef64sDesc);
/// }
/// ```
///
/// ### 3. Memory Management and Function Invocation
///
/// Create instances of `ThreadRng` and interact with the Rust library using the linked functions. 
/// Memory is managed using `Arena.ofConfined()` to ensure that it is correctly allocated and freed:
///
/// ```java
/// ThreadRng() throws Throwable {
///     this.arena = Arena.ofConfined();
///     this.value = this.arena.allocate(size, align);
///     new_.invokeExact(this.value);
/// }
///
/// double genDouble() throws Throwable {
///     return (double)gen_f64.invokeExact(this.value);
/// }
///
/// long genLong() throws Throwable {
///     return (long)gen_i64.invokeExact(this.value);
/// }
///
/// void shuffleDoubles(double[] array) throws Throwable {
///     try (var arena = Arena.ofConfined()) {
///         var arr = arena.allocate(array.length * ValueLayout.JAVA_DOUBLE.byteSize(), ValueLayout.JAVA_DOUBLE.byteAlignment());
///         MemorySegment.copy(array, 0, arr, ValueLayout.JAVA_DOUBLE, 0, array.length);
///
///         shuffle_f64s.invokeExact(arr, this.value, array.length);
///         MemorySegment.copy(arr, ValueLayout.JAVA_DOUBLE, 0, array, 0, array.length);
///     }
/// }
/// ```
///
/// ### 4. Resource Management and Garbage Collection
///
/// Java's garbage collection (GC) must be considered when interacting with native code. 
/// The memory managed by `Arena` is scoped, and the arena must be closed when no longer needed:
///
/// ```java
/// try (var arena = Arena.ofConfined()) {
///     // Memory allocated within this arena will be freed when the arena is closed.
///     // Ensure that the memory is not used beyond the arena's scope.
/// }
/// ```
///
/// Simulating GC activity and ensuring that memory is managed correctly:
///
/// ```java
/// System.gc();
/// ```
///
/// ### 5. Testing and Debugging
///
/// - **Compile and Run**: Ensure the Rust library is correctly linked and the Java code compiles without errors. Run the Java application to verify that it interacts with the Rust library as expected.
/// - **Debugging**: Pay special attention to memory management and function signatures. Ensure that data is correctly transferred between Java and Rust, and that memory is correctly allocated and freed.
///
/// ## Conclusion
///
/// This documentation provides an advanced guide for creating Java bindings to a complex Rust library. 
/// It covers the handling of static data, complex functions, memory management, and interactions between 
/// Java's garbage collection and Rust's memory safety guarantees. As you develop more complex Rust libraries, 
/// this guide can be extended to cover additional scenarios, ensuring safe and effective integration between Java and Rust.
