# `ioctl-gen`

[Documentation](https://docs.rs/ioctl-gen)

Making those magic `ioctl` numbers is really annoying without macros, so here are some macros. And if you were wondering, yes, they work in constants.

## Example

```rust
#[macro_use]
extern crate ioctlify;

// Taken from <linux/videodev2.h>
const VIDIOC_RESERVED:   u32 = io!(b'V', 1);
const VIDIOC_STREAMON:   u32 = iow!(b'V', 18, 4);
const VIDIOC_LOG_STATUS: u32 = io!(b'V', 70);

assert_eq!(ioc_type!(VIDIOC_RESERVED), b'V' as u32);
assert_eq!(VIDIOC_STREAMON, 1074026002);
assert_eq!(ioc_nr!(VIDIOC_LOG_STATUS), 70);
```

## Installation

In `Cargo.toml`:

```toml
[dependencies]
ioctl-gen = "0.1.0"
```

## License

[The MIT license.](LICENSE.md)

