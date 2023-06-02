use crate::graf::graph::Graph;

pub trait Alg {
    fn new(d_value: usize) -> Self where Self: Sized;
    fn request(&mut self, graph: &mut dyn Graph, dest: usize) -> usize;
    fn total_cost(&self) -> usize;
    fn clear(&mut self);
    fn name(&self) -> &'static str;
}
