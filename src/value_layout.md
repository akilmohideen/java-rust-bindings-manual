# Value Layout
`ValueLayout` is the most primitive layout type, representing the layout of,
well, primitives. They are:
<br>  
1. ValueLayout.ADDRESS
1. ValueLayout.JAVA_BOOLEAN
1. ValueLayout.JAVA_BYTE
1. ValueLayout.JAVA_CHAR
1. ValueLayout.JAVA_DOUBLE
1. ValueLayout.JAVA_FLOAT
1. ValueLayout.JAVA_INT
1. ValueLayout.JAVA_LONG
1. ValueLayout.JAVA_SHORT
1. ValueLayout.ADDRESS_UNALIGNED
1. ValueLayout.JAVA_CHAR_UNALIGNED
1. ValueLayout.JAVA_DOUBLE_UNALIGNED
1. ValueLayout.JAVA_FLOAT_UNALIGNED
1. ValueLayout.JAVA_INT_UNALIGNED
1. ValueLayout.JAVA_LONG_UNALIGNED
1. ValueLayout.JAVA_SHORT_UNALIGNED
 
These all correspond to the Java
primitives (`ADDRESS` is a bit special), [aligned and unaligned](size_and_alignment.md), which have
direct mappings to C primitive types.  
<br>  

## Type Mappings: Java, C, and Rust 
| **Java Type** | **C Type** | **Rust Type** | **Description** |
 |-----------------------|---------------------------|-------------------------|-------------------------------------| 
| `ValueLayout.ADDRESS` | Pointer | `*mut`, `*const` | Pointer to a memory location. | 
| `ValueLayout.JAVA_INT`| `int` | `i32` | 32-bit signed integer. |
| `ValueLayout.JAVA_LONG`| `long` | `i64` | 64-bit signed integer. | 
| `ValueLayout.JAVA_SHORT`| `short` | `i16` | 16-bit signed integer. |
| `ValueLayout.JAVA_BYTE`| `char` | `i8` | 8-bit signed integer. | 
| `ValueLayout.JAVA_BOOLEAN`| `char (0 or 1)` | `bool` | Boolean value (true or false). | 
| `ValueLayout.JAVA_FLOAT`| `float` | `f32` | 32-bit floating-point number. | 
| `ValueLayout.JAVA_DOUBLE`| `double` | `f64` | 64-bit floating-point number. | 
| `ValueLayout.JAVA_CHAR`| `short (UTF-16)` | `u16` | 16-bit unsigned integer for UTF-16. | 

## Unsigned Types
| **Java Type** | **C Type** | **Rust Type** | **Description** |
 |-----------------------|---------------------------|-------------------------|-------------------------------------|
| `ValueLayout.JAVA_INT` | `unsigned int` | `u8` | 8-bit unsigned integer. | 
| `ValueLayout.JAVA_INT` | `unsigned int` | `u16` | 16-bit unsigned integer. | 
| `ValueLayout.JAVA_LONG` | `unsigned long` | `u32` | 32-bit unsigned integer. | 
| `ValueLayout.JAVA_LONG`| `unsigned long` | `u64` | 64-bit unsigned integer. | 

<br>  

So the `_UNALIGNED` versions are exactly the same as their counterparts
except that they have an alignment of 1. This allows storing them unaligned,
but it will also force the JVM to issue special instruction sequences to load
values, since most CPU architectures do not natively support unaligned loads
and stores from or to memory. It is also worth noting that
`ValueLayout.JAVA_DOUBLE` and `ValueLayout.JAVA_LONG` have
platform-dependent alignment because some CPU architectures require
natural alignment (size = alignment, so 8 in this case) whereas some like
x86 only require an alignment of 4. All other primitives are defined to have
natural alignment.

Beyond representing primitive types, `ValueLayouts` also provide access to
different byte [ordering](orderings.md) (also known as endianness) through the
`.withOrder(ByteOrder)` method. The choices for `ByteOrder` are `BIG_ENDIAN`,
and `LITTLE_ENDIAN`, although the static method `ByteOrder.nativeOrder()`
will return whichever of those your CPU natively uses (usually
`LITTLE_ENDIAN`). This is required by many serialization formats, such as
most network formats, because many of them require `BIG_ENDIAN` byte
order while most CPU architectures only natively support `LITTLE_ENDIAN`.
Rust doesn’t have `int`, `long`, etc., so we must use a different translation to

For additional information on `ValueLayout`, visit Oracle's [official documentation](https://cr.openjdk.org/~mcimadamore/jdk/FFM_22_PR/javadoc/java.base/java/lang/foreign/ValueLayout.html), and official Rust resource [*The Rustonomicon*](https://doc.rust-lang.org/nomicon/).
