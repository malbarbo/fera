# fera

An aggregation of algorithms, data structures and supporting crates.

This crate does not directly provides any item, it only reexports modules
corresponding to others crates. Each module is enable with a feature with the
same name. All features are disable by default. To avoid longer compile times,
it is recommend to enable only the features that will be used.

[![Docs.rs](https://docs.rs/fera/badge.svg)](https://docs.rs/fera/)
[![Crates.io](https://img.shields.io/crates/v/fera.svg)](https://crates.io/crates/fera)

## Crates

- [`fera-array`]: Arrays traits and implementations (prefixed, copy on write,
  nested, etc).
- [`fera-ext`]: Extensions traits for [`std`] types.
- [`fera-fun`]: Free functions for fun programming.
- [`fera-graph`]: Graph data structures and algorithms.
- [`fera-optional`]: An optional value trait and some implementations.
- [`fera-unionfind`]: [Union-find] (disjoint-set) data structure
  implementation.

## Example

To use `ext` and `fun` crates in this example:

```rust
extern crate fera;

use fera::ext::VecExt;
use fera::fun::vec;

fn main() {
    assert_eq!(vec![3, 2, 1], vec(1..4).reversed());
}
```

it is necessary to add this to `Cargo.toml`:

```toml
[dependencies]
fera = { version = "0.2", features = ["ext", "fun"] }
```


## License

Licensed under [Mozilla Public License 2.0][mpl]. Contributions will be
accepted under the same license.

[`fera-array`]: https://github.com/malbarbo/fera/tree/master/array
[`fera-ext`]: https://github.com/malbarbo/fera/tree/master/ext
[`fera-fun`]: https://github.com/malbarbo/fera/tree/master/fun
[`fera-graph`]: https://github.com/malbarbo/fera/tree/master/graph
[`fera-optional`]: https://github.com/malbarbo/fera/tree/master/optional
[`fera-unionfind`]: https://github.com/malbarbo/fera/tree/master/unionfind
[mpl]: https://www.mozilla.org/en-US/MPL/2.0/
[`std`]: https://doc.rust-lang.org/stable/std/
[Union-find]: https://en.wikipedia.org/wiki/Disjoint-set_data_structure
