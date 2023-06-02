pub trait Graph {
    fn new(n: usize) -> Self where Self: Sized;
    fn distance(&self, v1: usize, v2: usize) -> usize;
    // fn request(&self, dest: usize) -> usize;
    // fn move_resource(&mut self, dest: usize) -> usize;
    fn resource_location(&self) -> usize;
    fn set_resource_location(&mut self, dest: usize);
    fn number_of_nodes(&self) -> usize;

    fn request(&self, dest: usize) -> usize {
        self.distance(self.resource_location(), dest)
    }

    fn move_resource(&mut self, dest: usize) -> usize {
        let prev_location = self.resource_location();
        self.set_resource_location(dest);
        self.distance(prev_location, dest)
    }
}
