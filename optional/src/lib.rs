//! An optional value trait and some implementations.
//!
//! There are two main uses for this:
//!
//! 1. To abstract over the representation of optional values in generic code
//!    ([`Optional`] trait);
//! 2. To represent optional value in a space efficient way ([`OptionalBool`], [`OptionalMax`],
//!    [`OptionalMin`], etc).
//!
//! Note that this is complementary to [`std::option::Option`], not a replacement. The idea is to
//! use [`std::option::Option`] interface by converting an [`Optional`] value to an
//! [`std::option::Option`] value.
//!
//! The [`optional`] crate is similar, but we think that this module is more generic, mainly
//! because [`optional`] crate is not concerned with the use case 1.
//!
//! This crate can be used through [`fera`] crate.
//!
//! # Example
//!
//! One can use `OptionalMax<usize>` to represent an optional `usize` where the `None` value is
//! represented by `usize::MAX`.
//!
//! ```
//! use fera_optional::*;
//! use std::mem;
//!
//! assert_eq!(mem::size_of::<usize>(), mem::size_of::<OptionalMax<usize>>());
//!
//! // default
//! let y = OptionalMax::<usize>::default();
//! assert_eq!(None, y.into_option());
//!
//! // from a value
//! let x = OptionalMax::from(10usize);
//! assert_eq!(Some(&10), x.to_option_ref());
//! assert_eq!(10, *x.to_option_ref().unwrap());
//!
//! // from an optional value
//! let z = Some(10);
//! let w = OptionalMax::from(z);
//! assert_eq!(Some(10), w.into_option());
//! ```
//!
//! Using `OptionalBool`
//!
//! ```
//! use fera_optional::*;
//! use std::mem;
//!
//! assert_eq!(1, mem::size_of::<OptionalBool>());
//!
//! let mut x = OptionalBool::from(false);
//! assert!(!*x.to_option_ref().unwrap());
//!
//! // change the value
//! *x.to_option_mut().unwrap() = true;
//! assert!(*x.to_option_ref().unwrap());
//! ```
//!
//! [`fera`]: https://docs.rs/fera
//! [`OptionalBool`]: struct.OptionalBool.html
//! [`optional`]: https://crates.io/crates/optional
//! [`OptionalMax`]: type.OptionalMax.html
//! [`OptionalMin`]: type.OptionalMin.html
//! [`Optional`]: trait.Optional.html
//! [`std::option::Option`]: https://doc.rust-lang.org/stable/std/option/enum.Option.html

extern crate num_traits;

use std::marker::PhantomData;
use std::mem;
use std::fmt;

use num_traits::bounds::Bounded;

/// An `Optional` that uses `T::max_value()` as `None`.
pub type OptionalMax<T> = Optioned<T, MaxNone<T>>;

/// An `Optional` that uses `T::min_value()` as `None`.
pub type OptionalMin<T> = Optioned<T, MinNone<T>>;

/// A trait that represents an optional value. This is a complement to [`std::option::Option`] that
/// allows implementations to choose how to represent `Some` and `None`.
///
/// [`std::option::Option`]: https://doc.rust-lang.org/stable/std/option/enum.Option.html
pub trait Optional<T>: Default + From<T> {
    /// Returns an `Option<&T>` that is equivalent to this `Optional`.
    fn to_option_ref(&self) -> Option<&T>;

    /// Returns an `Option<&mut T>` that is equivalent to this `Optional`.
    fn to_option_mut(&mut self) -> Option<&mut T>;

    /// Converts this `Optional` to the equivalent `Option`.
    fn into_option(self) -> Option<T>;
}

impl<T> Optional<T> for Option<T> {
    fn to_option_ref(&self) -> Option<&T> {
        self.as_ref()
    }

    fn to_option_mut(&mut self) -> Option<&mut T> {
        self.as_mut()
    }

    fn into_option(self) -> Option<T> {
        self
    }
}

/// An `Optional` that represents `None` with a specified value (`B::none()`) of `T` domain.
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Optioned<T, B: BuildNone<T>> {
    value: T,
    _marker: PhantomData<B>,
}

/// A builder for `None` values for type `T`.
pub trait BuildNone<T> {
    fn none() -> T;

    fn desc() -> &'static str;
}

impl<T: Eq + fmt::Debug, B: BuildNone<T>> fmt::Debug for Optioned<T, B> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Optional{}::{:?}", B::desc(), self.to_option_ref())
    }
}

impl<T, B: BuildNone<T>> Default for Optioned<T, B> {
    fn default() -> Self {
        Optioned {
            value: B::none(),
            _marker: PhantomData,
        }
    }
}

impl<T, B: BuildNone<T>> From<Option<T>> for Optioned<T, B> {
    fn from(value: Option<T>) -> Self {
        if let Some(v) = value {
            Optioned::from(v)
        } else {
            Optioned::default()
        }
    }
}

impl<T, B: BuildNone<T>> From<T> for Optioned<T, B> {
    fn from(value: T) -> Self {
        Optioned {
            value: value,
            _marker: PhantomData,
        }
    }
}

impl<T: Eq, B: BuildNone<T>> Optional<T> for Optioned<T, B> {
    #[inline(always)]
    fn to_option_ref(&self) -> Option<&T> {
        if self.value == B::none() {
            None
        } else {
            Some(&self.value)
        }
    }

    #[inline(always)]
    fn to_option_mut(&mut self) -> Option<&mut T> {
        if self.value == B::none() {
            None
        } else {
            Some(&mut self.value)
        }
    }

    #[inline(always)]
    fn into_option(self) -> Option<T> {
        if self.value == B::none() {
            None
        } else {
            Some(self.value)
        }
    }
}

/// Creates `T::max_value()` as `None`.
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct MaxNone<T>(PhantomData<T>);

impl<T: Bounded> BuildNone<T> for MaxNone<T> {
    fn none() -> T {
        T::max_value()
    }

    fn desc() -> &'static str {
        "Max"
    }
}

/// Creates `T::min_value()` as `None`.
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct MinNone<T>(PhantomData<T>);

impl<T: Bounded> BuildNone<T> for MinNone<T> {
    fn none() -> T {
        T::min_value()
    }

    fn desc() -> &'static str {
        "Min"
    }
}


/// An `Optional` for `bool` with 1 byte size.
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Default)]
pub struct OptionalBool(OptionalMax<u8>);

impl From<bool> for OptionalBool {
    fn from(value: bool) -> Self {
        OptionalBool(OptionalMax::from(unsafe { mem::transmute::<bool, u8>(value) }))
    }
}

impl fmt::Debug for OptionalBool {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "OptionalBool::{:?}", self.to_option_ref())
    }
}

impl From<Option<bool>> for OptionalBool {
    fn from(value: Option<bool>) -> Self {
        if let Some(v) = value {
            OptionalBool::from(v)
        } else {
            OptionalBool::default()
        }
    }
}

impl Optional<bool> for OptionalBool {
    #[inline(always)]
    fn to_option_ref(&self) -> Option<&bool> {
        unsafe { mem::transmute(self.0.to_option_ref()) }
    }

    #[inline(always)]
    fn to_option_mut(&mut self) -> Option<&mut bool> {
        unsafe { mem::transmute(self.0.to_option_mut()) }
    }

    #[inline(always)]
    fn into_option(self) -> Option<bool> {
        unsafe { mem::transmute(self.0.into_option()) }
    }
}

#[cfg(test)]
pub trait OptionalTest {
    type Item: ::std::fmt::Debug + PartialEq;
    type Sut: Optional<Self::Item>;

    fn values() -> (Self::Item, Self::Item);

    fn sut_default() -> Self::Sut {
        Self::Sut::default()
    }

    fn sut_from(value: Self::Item) -> Self::Sut {
        Self::Sut::from(value)
    }

    fn to_option_ref() {
        let (v1, v2) = Self::values();
        let (x1, x2) = Self::values();
        assert_eq!(None, Self::sut_default().to_option_ref());
        assert_eq!(Some(&v1), Self::sut_from(x1).to_option_ref());
        assert_eq!(Some(&v2), Self::sut_from(x2).to_option_ref());
    }

    fn to_option_mut() {
        let (mut v1, mut v2) = Self::values();
        let (x1, x2) = Self::values();
        assert_eq!(None, Self::sut_default().to_option_mut());
        assert_eq!(Some(&mut v1), Self::sut_from(x1).to_option_mut());
        assert_eq!(Some(&mut v2), Self::sut_from(x2).to_option_mut());

        let (x1, x2) = Self::values();
        let mut o = Self::sut_from(x1);
        *o.to_option_mut().unwrap() = x2;
        assert_eq!(Some(&v2), o.to_option_ref());
        assert_eq!(Some(&mut v2), o.to_option_mut());
        assert_eq!(Some(v2), o.into_option());
    }

    fn into_option() {
        let (v1, v2) = Self::values();
        let (x1, x2) = Self::values();
        assert_eq!(None, Self::sut_default().into_option());
        assert_eq!(Some(v1), Self::sut_from(x1).into_option());
        assert_eq!(Some(v2), Self::sut_from(x2).into_option());
    }
}

#[cfg(test)]
mod tests {
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

    mod optional_bool {
        use *;
        struct T;

        impl OptionalTest for T {
            type Item = bool;
            type Sut = OptionalBool;

            fn values() -> (Self::Item, Self::Item) {
                (false, true)
            }
        }

        delegate_tests!{T, to_option_ref, to_option_mut, into_option}

        #[test]
        fn debug() {
            assert_eq!("OptionalBool::None",
                       format!("{:?}", OptionalBool::default()));
            assert_eq!("OptionalBool::Some(true)",
                       format!("{:?}", OptionalBool::from(true)));
            assert_eq!("OptionalBool::Some(false)",
                       format!("{:?}", OptionalBool::from(false)));
        }
    }

    mod optioned_u32 {
        use *;
        struct T;

        impl OptionalTest for T {
            type Item = u32;
            type Sut = OptionalMax<u32>;

            fn values() -> (Self::Item, Self::Item) {
                (10, 50)
            }
        }

        delegate_tests!{T, to_option_ref, to_option_mut, into_option}

        #[test]
        fn debug() {
            assert_eq!("OptionalMax::None",
                       format!("{:?}", OptionalMax::<u32>::default()));
            assert_eq!("OptionalMax::Some(10)",
                       format!("{:?}", OptionalMax::from(10u32)));

            assert_eq!("OptionalMin::None",
                       format!("{:?}", OptionalMin::<u32>::default()));
            assert_eq!("OptionalMin::Some(10)",
                       format!("{:?}", OptionalMin::from(10u32)));
        }
    }
}
