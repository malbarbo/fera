//! An aggregation of algorithms, data structures and supporting crates.
//!
//! This crate does not directly provides any item, it only reexports modules corresponding to
//! others crates. Each module is enable with a feature with the same name. All features are
//! disable by default. To avoid longer compile times, it is recommend to enable only the features
//! that will be used.
//!
//! # Example
//!
//! To use `ext` and `fun` crates in this example:
//!
//! ```rust
//! extern crate fera;
//!
//! use fera::ext::VecExt;
//! use fera::fun::vec;
//!
//! # fn main() {
//! assert_eq!(vec![3, 2, 1], vec(1..4).reversed());
//! # }
//! ```
//!
//! it is necessary to add this to `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! fera = {
//!   version = "0.1",
//!   features = ["ext", "fun"]
//! }
//! ```

macro_rules! crate_as_mod {
    ($c: ident, $m: ident, $f: expr, $d: expr) => (
        #[cfg(feature = $f)]
        extern crate $c;

        #[doc = $d]
        #[cfg(feature = $f)]
        pub mod $m {
            pub use $c::*;
        }
    )
}

crate_as_mod!(fera_ext, ext, "ext", "Extension traits for std types.");
crate_as_mod!(fera_fun, fun, "fun", "Free function for fun programming.");
crate_as_mod!(fera_graph, graph, "graph", "Graph data structures and algorithms.");
crate_as_mod!(fera_optional, optional, "optional", "Generic optional value.");
crate_as_mod!(fera_unionfind, unionfind, "unionfind", "Union find data structure.");
