macro_rules! crate_as_mod {
    ($c: ident, $m: ident, $f: expr) => (
        #[cfg(feature = $f)]
        extern crate $c;

        #[cfg(feature = $f)]
        pub mod $m {
            pub use $c::*;
        }
    )
}

crate_as_mod!(fera_ext, ext, "ext");
crate_as_mod!(fera_fun, fun, "fun");
crate_as_mod!(fera_graph, graph, "graph");
crate_as_mod!(fera_optional, optional, "optional");
crate_as_mod!(fera_unionfind, unionfind, "unionfind");
