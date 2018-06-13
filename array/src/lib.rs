//! Traits for array like structs and some implementations.

#![cfg_attr(feature = "nightly", feature(test))]

#[cfg(test)]
extern crate testdrop;

#[cfg(all(test, feature = "nightly"))]
extern crate rand;
#[cfg(feature = "nightly")]
extern crate test;

extern crate fera_fun;

#[cfg(test)]
macro_rules! delegate_tests {
    ($T: ident, $($names: ident),+) => (
        $(
            #[test]
            fn $names() {
                $T::$names();
            }
        )*
    )
}

#[cfg(feature = "nightly")]
macro_rules! delegate_benchs {
    ($T: ident, $($names: ident),+) => (
        $(
            #[bench]
            fn $names(b: &mut Bencher) {
                $T::$names(b);
            }
        )*
    )
}

macro_rules! inline_mod {
    ($($name:ident),+,) => (
        $(
            mod $name;
            pub use $name::*;
        )*
    )
}

inline_mod! {
    array,
    cow,
    nested,
    prefixed,
    rc,
    vec,
}
