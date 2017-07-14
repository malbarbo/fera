//! Extensions traits for [`std`] types.
//!
//! This crate can be used through [`fera`] crate.
//!
//! [`fera`]: https://docs.rs/fera
//! [`std`]: https://doc.rust-lang.org/stable/std/
extern crate rand;

mod vec;

pub use vec::VecExt;
