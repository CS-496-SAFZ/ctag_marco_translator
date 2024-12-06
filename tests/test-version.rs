 Here is how you can convert the given C code to Rust:

```rust
#![allow(unused_imports)]
use libc::{c_int, c_char, size_t};
use std::ptr;

extern "C" {
    fn yaml_get_version(major: *mut c_int, minor: *mut c_int, patch: *mut c_int);
    fn yaml_get_version_string() -> *const c_char;
}

fn main() {
    let mut major = -1;
    let mut minor = -1;
    let mut patch = -1;
    let mut buf = [0 as c_char; 64];

    unsafe {
        yaml_get_version(&mut major, &mut minor, &mut patch);
        let version = CStr::from_ptr(yaml_get_version_string()).to_str().unwrap();
        let _ = write!(&mut buf[..] as &mut [_], "{:.*}{:.*}{:.*}", major, minor, patch);
        assert_eq!(buf.as_ref(), version.as_bytes());
    }

    /* Print structure sizes. */
    println!("sizeof(token) = {}", std::mem::size_of::<yaml_token_t>());
    println!("sizeof(event) = {}", std::mem::size_of::<yaml_event_t>());
    println!("sizeof(parser) = {}", std::mem::size_of::<yaml_parser_t>());
}

#[repr(C)]
struct yaml_token_t {
    _data: [u8; 0],
}

#[repr(C)]
struct yaml_event_t {
    _data: [u8; 0], 
}

#[repr(C)]
struct yaml_parser_t {
    _data: [u8; 0],
}
```

The key changes:

- Use Rust's stdlib instead of C stdlib 
- Unsafe blocks to call C functions
- Use `println!` for printing
- Create dummy structs for C structs used
- Use `repr(C)` and arrays instead of `sizeof`
- Use `assert_eq!` for assertions
- Use `write!` to render formatted string to buffer