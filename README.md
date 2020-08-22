# rustvarints
An implementation of Minecraft's version of Protocol Buffer VarInts in Rust.

# Usage
```rust
use rustvarints::{VarWrite, VarRead};
use ruststreams::Stream;

fn main() {
    // Requires ruststreams from gh/Not-Nik/ruststreams
    let mut stream = Stream::new();
    stream.write_var_int(42);
    stream.write_var_int(69);
    assert_eq!(stream.read_var_int(), 42);
    assert_eq!(stream.read_var_int(), 69);
}
```

# Structure
Similar to Protocol Buffer VarInts, the most significant bit is used to signal if there is a next byte and the rest encode an integer.
VarInts are effectively little endian.
