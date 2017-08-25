mod node;
mod tree;

pub use self::node::*;
pub use self::tree::*;

use std::ptr;

pub fn link<N: Node>(x: &N, y: &N) {
    // TODO: return an error
    debug_assert_ne!(x.find_root(), y.find_root());
    x.make_root();
    x.set_parent(Some(y));
}

pub fn cut<N: Node>(x: &N, y: &N) {
    // TODO: return an error
    debug_assert_eq!(x.find_root(), y.find_root());
    x.make_root();
    y.expose();
    debug_assert_eq!(Some(x), y.right());
    debug_assert_eq!(None, x.left());
    debug_assert_eq!(None, x.right());
    y.right().unwrap().set_parent(None);
    y.set_right(None);
}

pub fn is_connected<N: Node>(x: &N, y: &N) -> bool {
    ptr::eq(x, y) || x.find_root() == y.find_root()
}
