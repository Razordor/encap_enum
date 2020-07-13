# Encap_Enum
![Rust](https://github.com/Razordor/encap_enum/workflows/Rust/badge.svg)

Encap_Enum provides the `encap_enum!` macro for defining enumerations, bitflags and groups of constants.

## Usage
Add this to your `Cargo.toml`:
```toml
[dependencies]
encap_enum = "0.3.0"
```
and this to your crate root:
```rust
#[macro_use]
extern crate encap_enum;
```

## Features
- iterators.
- bit flags.
- encapsulated flag values.
  - change `enum` visibility for both internal values and the enum itself.
- supports any number of attributes.
  - This includes doc comments, `#[repr(C)]`, `#[derive]`, and many others.
- No std dependency.
- FFI compatible.

### Example
```rust
encap_enum!{
    /// ClassStyle is a structure used in the Window class.
    #[repr(C)]
    pub enum ClassStyle: pub u32 {
        ByteAlignClient     = 0x0000_1000,
        ByteAlignWindow     = 0x0000_2000, /// Aligns window on a byte boundary.
        DoubleClicks        = 0x0000_0008,
        DropShadow          = 0x0002_0000,
        GlobalClass         = 0x0000_4000,
        // ...
    }
}
fn main() {
    println!("ByteAlignClient integer representation: {}", ClassStyle::ByteAlignClient.raw);
    println!("ByteAlignClient debug representation: {:?}", ClassStyle::ByteAlignClient);
}
```

### License
`encap_enum` is licenced under the [MIT Licence](https://github.com/Razordor/encap_enum/blob/master/LICENSE).

<details closed>
<summary>Recent Changes (Breaking Changes!)</summary>

* **Breaking:** changed tuple struct to regular struct for ffi compatibility.
  * To access data use `raw` variable.
* added `new` static function. follows inner visibility rules

</details>
