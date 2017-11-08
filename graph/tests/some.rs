// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[macro_use]
extern crate fera_graph;
extern crate fera_fun;

use fera_fun::enumerate;
use fera_graph::prelude::*;
use fera_graph::algs::{Components, Cycles, Trees};

struct Case {
    g: StaticGraph,
    is_connected: bool,
    is_acyclic: bool,
    is_tree: bool,
}

fn cases() -> Vec<Case> {
    vec![Case {
             // 0
             g: graph!(),
             is_connected: true,
             is_acyclic: true,
             is_tree: true,
         },
         Case {
             // 1
             g: graph!(1),
             is_connected: true,
             is_acyclic: true,
             is_tree: true,
         },
         Case {
             // 2
             g: graph!(2),
             is_connected: false,
             is_acyclic: true,
             is_tree: false,
         },
         Case {
             // 3
             g: graph!(2, (0, 1)),
             is_connected: true,
             is_acyclic: true,
             is_tree: true,
         },
         Case {
             // 4
             g: graph!(3, (2, 1)),
             is_connected: false,
             is_acyclic: true,
             is_tree: false,
         },
         Case {
             // 5
             g: graph!(3, (2, 1)),
             is_connected: false,
             is_acyclic: true,
             is_tree: false,
         },
         Case {
             // 6
             g: graph!(3, (0, 1), (1, 2)),
             is_connected: true,
             is_acyclic: true,
             is_tree: true,
         },
         Case {
             // 7
             g: graph!(3, (0, 1), (0, 2), (1, 2)),
             is_connected: true,
             is_acyclic: false,
             is_tree: false,
         },
         Case {
             // 8
             g: graph!(4, (0, 1), (0, 2)),
             is_connected: false,
             is_acyclic: true,
             is_tree: false,
         },
         Case {
             // 9
             g: graph!(4, (1, 2), (2, 3), (3, 1)),
             is_connected: false,
             is_acyclic: false,
             is_tree: false,
         }]
}

#[test]
fn is_connected() {
    for (i, case) in enumerate(cases()) {
        assert_eq!(case.is_connected, case.g.is_connected(), "Case {}", i);
    }
}

#[test]
fn is_acyclic() {
    for (i, case) in enumerate(cases()) {
        assert_eq!(case.is_acyclic, case.g.is_acyclic(), "Case {}", i);
    }
}

#[test]
fn is_tree() {
    for (i, case) in enumerate(cases()) {
        assert_eq!(case.is_tree, case.g.is_tree(), "Case {}", i);
    }
}
