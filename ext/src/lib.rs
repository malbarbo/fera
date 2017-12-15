// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![doc(html_root_url = "https://docs.rs/fera-ext/0.1.0/")]

//! Extensions traits for [`std`] types.
//!
//! This crate can be used through [`fera`] crate.
//!
//! [`fera`]: https://docs.rs/fera
//! [`std`]: https://doc.rust-lang.org/stable/std/
extern crate rand;

mod vec;

pub use vec::VecExt;
