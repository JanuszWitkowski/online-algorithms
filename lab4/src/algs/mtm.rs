use crate::graf::graph::Graph;
use crate::algs::alg::Alg;

pub struct MoveToMin {
    phase_reqs: Vec<usize>,
    phase_dur:  usize,
    req_ctr:    usize,
    move_cost:  usize,
    total_cost: usize,
}


impl MoveToMin {
    fn minimizing_node(&self, graph: &dyn Graph) -> usize {
        let mut minimizing_node = 1;
        let mut node_sum = self.phase_reqs.iter().map(|z| graph.distance(*z, 1)).sum::<usize>();
        let mut min_value = node_sum;
        for node in 2..=graph.number_of_nodes() {
            node_sum = self.phase_reqs.iter().map(|z| graph.distance(*z, node)).sum::<usize>();
            // match node_sum < min_value {
            //     true => {
            //         min_value = node_sum;
            //         minimizing_node = node;
            //     },
            //     _ => {},
            // }
            if let true = node_sum < min_value {
                min_value = node_sum;
                minimizing_node = node;
            }
        }
        minimizing_node
    }
}


impl Alg for MoveToMin {
    fn new(d_value: usize) -> Self {
        MoveToMin{ 
            phase_reqs: Vec::new(),
            phase_dur: d_value, 
            req_ctr: 0, 
            move_cost: d_value,
            total_cost: 0 
        }
    }

    fn total_cost(&self) -> usize {
        self.total_cost
    }

    fn clear(&mut self) {
        self.phase_reqs.clear();
        self.req_ctr = 0;
        self.total_cost = 0;
    }

    fn request(&mut self, graph: &mut dyn Graph, dest: usize) -> usize {
        let mut cost = graph.request(dest);
        self.phase_reqs.push(dest);
        self.req_ctr += 1;

        if self.req_ctr >= self.phase_dur {
            cost += self.move_cost * graph.move_resource(self.minimizing_node(graph));   // UNCOMMENT THIS!
            self.phase_reqs.clear();
            self.req_ctr = 0;
        }

        self.total_cost += cost;
        cost
    }

    fn name(&self) -> &'static str {
        "MoveToMin"
    }
}

