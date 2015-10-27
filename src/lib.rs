#[macro_export]
macro_rules! set {
    () => {{
        use std::collections::HashSet;
        HashSet::new()
    }};
    ($($x:expr),+) => {{
        use std::collections::HashSet;
        [$($x,)+].iter().map(|&x| x).collect::<HashSet<_>>()
    }}
}

// https://stackoverflow.com/questions/30291584/macro-for-defining-trait-aliases

#[macro_export]
macro_rules! items {
    ($($item:item)*) => ($($item)*);
}

#[macro_export]
macro_rules! trait_alias {
    ($name:ident : $($base:tt)+) => {
        items! {
            pub trait $name: $($base)+ { }
            impl<T> $name for T where T: $($base)+ { }
        }
    };
}

extern crate rand;
extern crate ds;

#[cfg(test)]
#[macro_use]
pub mod tests;

pub mod graph;

pub mod subgraph;

pub mod static_;

pub mod builder;
pub use builder::*;

pub mod choose;

pub mod iter;

pub mod kruskal;
pub mod path;
pub mod props;
pub mod traverse;
pub mod unionfind;

pub mod prelude {
    pub use graph::*;
    pub use builder::WithBuilder;
    pub use static_::*;
    pub use iter::*;
}
