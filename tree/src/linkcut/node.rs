// Inspired by
// https://github.com/indy256/codelibrary/blob/master/java/src/LinkCutTreeConnectivity.java
// http://codeforces.com/contest/117/submission/860934
// https://github.com/sambayless/monosat/blob/master/src/monosat/dgl/alg/LinkCutTree.h

use std::fmt::Debug;

use self::Kind::*;
use self::Dir::*;

pub trait Node: Sized + PartialEq + Debug {
    fn find_root(&self) -> &Self {
        self.expose();
        let mut x = self;
        while let Some(right) = x.right() {
            x = right;
        }
        x
    }

    fn make_root(&self) {
        self.expose();
        self.flip_revert();
    }

    fn expose(&self) -> Option<&Self> {
        let mut last = None;
        let mut p = Some(self);
        while let Some(x) = p {
            x.splay();
            x.set_left(last);
            last = Some(x);
            p = x.parent();
        }
        self.splay();
        last
    }

    fn splay(&self) {
        while let Child(p, self_dir) = kind(self) {
            if let Child(pp, _) = kind(p) {
                pp.push();
            }
            p.push();
            self.push();
            if let Child(_, p_dir) = kind(p) {
                if self_dir == p_dir {
                    p.rotate();
                } else {
                    self.rotate();
                }
            }
            self.rotate();
        }
        self.push();
    }

    fn push(&self) {
        if self.revert() {
            self.flip_revert();
            let left = self.left();
            let right = self.right();
            self.set_left(right);
            if let Some(left) = self.left() {
                left.flip_revert();
            }
            self.set_right(left);
            if let Some(right) = self.right() {
                right.flip_revert();
            }
        }
    }

    //     pp                 pp
    //      \                  \
    //       p                self
    //      / \        ->     /  \
    //   self  p.r          s.l  p
    //   /  \                   / \
    // s.l  s.r               s.r  p.r
    fn rotate(&self) {
        let p = self.parent().unwrap();
        let pp = p.parent();

        if p.left() == Some(self) {
            p.connect_left(self.right());
            self.connect_right(Some(p));
        } else {
            p.connect_right(self.left());
            self.connect_left(Some(p));
        }

        if let Some(pp) = pp {
            if pp.left() == Some(p) {
                pp.connect_left(Some(self));
            } else if pp.right() == Some(p) {
                pp.connect_right(Some(self));
            } else {
                self.set_parent(Some(pp));
            }
        } else {
            self.set_parent(None);
        }
    }

    fn connect_left(&self, x: Option<&Self>) {
        self.set_left(x);
        if let Some(x) = x {
            x.set_parent(Some(self));
        }
    }

    fn connect_right(&self, x: Option<&Self>) {
        self.set_right(x);
        if let Some(x) = x {
            x.set_parent(Some(self));
        }
    }

    fn revert(&self) -> bool;

    fn flip_revert(&self);

    fn left(&self) -> Option<&Self>;

    fn set_left(&self, left: Option<&Self>);

    fn right(&self) -> Option<&Self>;

    fn set_right(&self, right: Option<&Self>);

    fn parent(&self) -> Option<&Self>;

    fn set_parent(&self, parent: Option<&Self>);
}

fn kind<N: Node>(x: &N) -> Kind<&N> {
    if let Some(parent) = x.parent() {
        if parent.left() == Some(x) {
            return Child(parent, Left);
        }
        if parent.right() == Some(x) {
            return Child(parent, Right);
        }
    }
    Root
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind<T> {
    Root,
    Child(T, Dir),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    Left,
    Right,
}
