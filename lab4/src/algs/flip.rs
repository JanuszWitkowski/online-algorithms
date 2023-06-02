use crate::graf::graph::Graph;
use crate::algs::alg::Alg;
use fastrand;

pub struct Flip {
    graph:      &'static mut dyn Graph,
    move_cost:  usize,
    total_cost: usize,
    move_prob:  f64,    // 1/2D
}


// impl Flip {
// }


impl Alg for Flip {
    fn new(graph: &'static mut dyn Graph, d_value: usize) -> Self {
        Flip {
            graph,
            move_cost: d_value,
            total_cost: 0,
            move_prob: 1.0/(2.0 * d_value as f64)
        }
    }

    fn total_cost(&self) -> usize {
        self.total_cost
    }

    fn clear(&mut self) {
        self.total_cost = 0;
    }

    fn request(&mut self, dest: usize) -> usize {
        let cost = match fastrand::f64() <= self.move_prob {
            true => self.move_cost * self.graph.move_resource(dest),
            false => self.graph.request(dest)
        };
        self.total_cost += cost;
        cost
    }
}
