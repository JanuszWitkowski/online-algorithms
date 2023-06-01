use crate::graf::graph::Graph;

pub trait Alg {
    fn new(graph: &'static mut dyn Graph, d_value: usize) -> Self where Self: Sized;
    fn request(&mut self, dest: usize) -> usize;
    fn total_cost(&self) -> usize;
    fn clear(&mut self);
}
