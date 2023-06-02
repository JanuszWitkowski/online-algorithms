use crate::graf::graph::Graph;

const NUMBER_OF_DIMENSIONS: usize = 3;

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
        u1 = match u1 {
            0 => d-1,
            x => x-1
        };
        u2 = (u2 + 1) % d;
        counter += 1;
    }
    counter
}

fn get_distance_table(d: usize) -> Vec<Vec<usize>> {
    let mut table = Vec::new();
    for i in 0..d {
        let mut row = Vec::new();
        for j in 0..d {
            row.push(calculate_distance(i, j, d));
        }
        table.push(row);
    }
    table
}


impl Torus {
    fn coordinates(&self, node: usize) -> [usize; NUMBER_OF_DIMENSIONS] {
        let mut coordinates = [0usize; NUMBER_OF_DIMENSIONS];
        let mut node_value = node - 1;
        // for coord in coordinates.iter_mut().take(NUMBER_OF_DIMENSIONS) {
        //     *coord = node_value % self.dimension_length;
        //     node_value /= self.dimension_length;
        // }
        for i in (0..NUMBER_OF_DIMENSIONS).rev() {
            coordinates[i] = node_value % self.dimension_length;
            node_value /= self.dimension_length;
        }
        coordinates
    }
}


impl Graph for Torus {
    fn new(n: usize) -> Self {
        let d = (n as f64).cbrt().ceil() as usize;
        Torus {
            number_of_nodes: n,
            dimension_length: d,
            resource_location: 1,
            distance_table: get_distance_table(d)
        }
    }

    fn distance(&self, v1: usize, v2: usize) -> usize {
        let (c1, c2) = (self.coordinates(v1), self.coordinates(v2));
        let mut cost = 0;
        for i in 0..NUMBER_OF_DIMENSIONS {
            cost += self.distance_table[c1[i]][c2[i]];
        }
        cost
    }

    // fn request(&self, dest: usize) -> usize {
    //     self.distance(self.resource_location, dest)
    // }

    // fn move_resource(&mut self, dest: usize) -> usize {
    //     //
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

    fn name(&self) -> &'static str {
        "Torus"
    }
}



#[cfg(test)]
mod tests {
    use crate::graf::graph::Graph;
    use crate::graf::torus::Torus;

    #[test]
    fn test_torus_coordinates() {
        let n = 64;
        let mut torus = Torus::new(n);
        for v in 1..=n {
            println!("{:?}", torus.coordinates(v));
        }
    }
}

