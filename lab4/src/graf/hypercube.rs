use crate::graf::graph::Graph;

pub struct Hypercube {
    number_of_nodes:    usize,
    number_of_bits:     usize,
    resource_location:  usize,
}

impl Graph for Hypercube {
    fn new(n: usize) -> Self {
        Hypercube{ 
            number_of_nodes: n, 
            number_of_bits: (n as f64).log2().ceil() as usize, 
            resource_location: 1 
        }
    }

    fn distance(&self, v1: usize, v2: usize) -> usize {
        let mut counter = 0;
        for b in 0..self.number_of_bits {
            if (v1 >> b) & 1 != (v2 >> b) & 1 {
                counter += 1;
            }
        }
        counter
    }

    // fn request(&self, dest: usize) -> usize {
    //     self.distance(self.resource_location, dest)
    // }

    // fn move_resource(&mut self, dest: usize) -> usize {
    //     let prev_location = self.resource_location;
    //     self.resource_location = dest;
    //     self.distance(prev_location, dest)
    // }

    fn resource_location(&self) -> usize {
        self.resource_location
    }

    fn set_resource_location(&mut self, dest: usize) {
        self.resource_location = dest;
    }

    fn number_of_nodes(&self) -> usize {
        self.number_of_nodes
    }
}
