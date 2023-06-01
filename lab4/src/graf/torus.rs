use crate::graf::graph::Graph;

pub struct Torus {
    number_of_nodes:    usize,
    dimension_length:   usize,
    resource_location:  usize,
    distance_table:     Vec<Vec<usize>>,
}


fn calculate_distance(u: usize, v: usize, d: usize) -> usize {
    let (mut u1, mut u2) = (u, u);
    let mut counter = 0;
    while u1 != v && u2 != v {
        if u1 == 0 {
            u1 = d-1;
        }
        counter += 1;
    }
    counter
}

fn get_distance_table(d: usize) -> Vec<Vec<usize>> {
    let mut table = Vec::new();
    for i in 0..d {
        let mut row = Vec::new()
        for j in 0..d {
            row.push()
        }
        table.push(row);
    }
}


impl Graph for Torus {
    fn new(n: usize) -> Self {
        Torus {
            number_of_nodes: n,
            dimension_length: (n as f64).cbrt().ceil() as usize,
            resource_location: 1,
            distance_table: 
        }
    }

    fn distance(&self, v1: usize, v2: usize) -> usize {
        //
    }

    // fn request(&self, dest: usize) -> usize {
    //     self.distance(self.resource_location, dest)
    // }

    fn move_resource(&mut self, dest: usize) -> usize {
        //
    }

    fn resource_location(&self) -> usize {
        self.resource_location
    }

    fn number_of_nodes(&self) -> usize {
        self.number_of_nodes
    }
}

