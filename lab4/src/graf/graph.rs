pub trait Graph {
    fn new(n: usize) -> Self where Self: Sized;
    fn distance(&self, v1: usize, v2: usize) -> usize;
    fn move_resource(&mut self, dest: usize) -> usize;
    fn resource_location(&self) -> usize;
}
