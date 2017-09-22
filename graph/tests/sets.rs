extern crate fera_graph;
#[macro_use]
extern crate quickcheck;

use fera_graph::prelude::*;
use fera_graph::sets::FastVecSet;

quickcheck! {
    fn sets(vertices: Vec<u8>) -> bool {
        let n = 20;
        let g = CompleteGraph::new(n);
        let mut count = 0;
        let mut expected = vec![false; n as usize];
        let mut actual = FastVecSet::new_vertex_set(&g);

        for v in vertices {
            let v = u32::from(v) % n;
            let vu = v as usize;

            if expected[vu] {
                count -= 1;
                expected[vu] = false;
                actual.remove(v);
            } else {
                count += 1;
                expected[vu] = true;
                actual.insert(v);
            }

            for (u, &exp) in expected.iter().enumerate() {
                assert_eq!(exp, actual.contains(u as u32));
            }

            assert_eq!(count == 0, actual.is_empty());
            assert_eq!(count, actual.len());
            assert_eq!(count, actual.iter().count());
            let mut vec = vec![false; n as usize];
            for u in &actual {
                vec[u as usize] = true;
            }
            assert_eq!(expected, vec);
        }

        actual.clear();
        assert!((0..n).all(|u| !actual.contains(u)));
        assert!(actual.is_empty());
        assert_eq!(0, actual.len());
        assert_eq!(0, actual.iter().count());

        true
    }
}
