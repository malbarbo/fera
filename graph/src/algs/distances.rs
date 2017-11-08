// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use prelude::*;
use props::{Color, FnProp};
use traverse::*;
use fun::max_prop;

pub trait Distances: Incidence {
    fn diameter(&self) -> usize
        where Self: VertexList + WithVertexProp<usize> + WithVertexProp<Color>
    {
        let mut dist = self.default_vertex_prop(0);
        self.vertices()
            .map(|v| {
                dist.set_values(self.vertices(), usize::max_value());
                self.dfs(RecordDistance(&mut dist)).root(v).run();
                max_prop(FnProp(|x| dist[x]), self.vertices()).unwrap()
            })
            .max()
            .unwrap_or(0)
    }
}

impl<G: Incidence> Distances for G {}
