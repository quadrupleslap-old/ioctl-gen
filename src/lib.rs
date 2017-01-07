//! This crate lets you encode and decode ioctl numbers.
//! It's pretty much just a port of `ioctl.h`.
//!
//! Here are some examples:
//!
//! ```
//! #[macro_use]
//! extern crate ioctlify;
//!
//! # fn main() {
//! // Taken from <linux/videodev2.h>
//! const VIDIOC_RESERVED:   u32 = io!(b'V', 1);
//! const VIDIOC_STREAMON:   u32 = iow!(b'V', 18, 4);
//! const VIDIOC_LOG_STATUS: u32 = io!(b'V', 70);
//!
//! assert_eq!(ioc_type!(VIDIOC_RESERVED), b'V' as u32);
//! assert_eq!(VIDIOC_STREAMON, 1074026002);
//! assert_eq!(ioc_nr!(VIDIOC_LOG_STATUS), 70);
//! # }
//! ```

//TODO: Nonstandard platforms.
//TODO: When will mem::size_of become a const-fn?

/// The number of bits used for the number field.
pub const NRBITS: u32 = 8;
/// The number of bits used for the type field.
pub const TYPEBITS: u32 = 8;
/// The number of bits used for the size field.
pub const SIZEBITS: u32 = 14;
/// The number of bits used for the direction field.
pub const DIRBITS: u32 = 2;
/// Neither direction.
pub const NONE: u32 = 0;
/// The write direction.
pub const WRITE: u32 = 1;
/// The read direction.
pub const READ: u32 = 2;

/// Bitmask for the number field.
pub const NRMASK: u32 = (1 << NRBITS) - 1;
/// Bitmask for the type field.
pub const TYPEMASK: u32 = (1 << TYPEBITS) - 1;
/// Bitmask for the size field.
pub const SIZEMASK: u32 = (1 << SIZEBITS) - 1;
/// Bitmask for the direction field.
pub const DIRMASK: u32 = (1 << DIRBITS) - 1;

/// Offset of the number field.
pub const NRSHIFT: u32 = 0;
/// Offset of the type field.
pub const TYPESHIFT: u32 = NRSHIFT + NRBITS;
/// Offset of the size field.
pub const SIZESHIFT: u32 = TYPESHIFT + TYPEBITS;
/// Offset of the direction field.
pub const DIRSHIFT: u32 = SIZESHIFT + SIZEBITS;

/// Creates the ioctl number for the given data.
///
/// `io`, `ior`, `iow` and `iowr` are preferred over this macro.
#[macro_export]
macro_rules! ioc {
    ($dr:expr, $ty:expr, $nr:expr, $sz:expr) => {
        (($dr as u32) << $crate::DIRSHIFT)  |
        (($ty as u32) << $crate::TYPESHIFT) |
        (($nr as u32) << $crate::NRSHIFT)   |
        (($sz as u32) << $crate::SIZESHIFT)
    }
}

/// Creates the ioctl number for an operation that isn't reading or writing.
#[macro_export]
macro_rules! io {
    ($ty:expr, $nr:expr) => {
        ioc!($crate::NONE, $ty, $nr, 0)
    }
}

/// Creates the ioctl number for a read-only operation.
#[macro_export]
macro_rules! ior {
    ($ty:expr, $nr:expr, $sz:expr) => {
        ioc!($crate::READ, $ty, $nr, $sz)
    }
}

/// Creates the ioctl number for a write-only operation.
#[macro_export]
macro_rules! iow {
    ($ty:expr, $nr:expr, $sz:expr) => {
        ioc!($crate::WRITE, $ty, $nr, $sz)
    }
}

/// Creates the ioctl number for a read/write operation.
#[macro_export]
macro_rules! iowr {
    ($ty:expr, $nr:expr, $sz:expr) => {
        ioc!($crate::READ | $crate::WRITE, $ty, $nr, $sz)
    }
}

/// Decodes the access mode / direction from an ioctl number.
#[macro_export]
macro_rules! ioc_dir {
    ($n:expr) => { ($n >> $crate::DIRSHIFT) & $crate::DIRMASK }
}

/// Decodes the type from an ioctl number.
#[macro_export]
macro_rules! ioc_type {
    ($n:expr) => { ($n >> $crate::TYPESHIFT) & $crate::TYPEMASK }
}

/// Decodes the function number from an ioctl number.
#[macro_export]
macro_rules! ioc_nr {
    ($n:expr) => { ($n >> $crate::NRSHIFT) & $crate::NRMASK }
}

/// Decodes the parameter size from an ioctl number.
#[macro_export]
macro_rules! ioc_size {
    ($n:expr) => { ($n >> $crate::SIZESHIFT) & $crate::SIZEMASK }
}

