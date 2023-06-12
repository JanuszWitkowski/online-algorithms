enum Phase {
    ONE,
    TWO,
    THREE,
    FOUR,
    FIVE,
}

enum RequestType {
    READ,
    WRITE,
}


struct Vertice {
    id:         usize,
    counter:    usize,
    phase,      Phase,
    has_copy:   bool,
    d_value:    usize,
}

impl Vertice {
    fn new(id: usize, phase: Phase, has_copy: bool, d_value: usize) -> Self {
        Vertice{ id, counter: 0, phase, has_copy, d_value }
    }

    fn get_counter(&self) -> usize {
        self.counter
    }

    fn counter_full(&self) -> bool {
        self.counter >= self.d_value
    }

    fn counter_empty(&self) -> bool {
        self.counter == 0
    }

    fn has_copy(&self) -> bool {
        self.has_copy
    }

    fn get_phase(&self) -> Phase {
        self.state
    }

    // fn set_state(&self, )

    // fn request(&self, request_id: usize, is_read_request: bool) -> usize {
    //     match self.phase {
    //         Phase::ONE => {
    //             if request_id == self.id {
    //                 if (self.counter < self.d_value && is_read_request) || (!is_read_request && )
    //             }
    //         },
    //         Phase::TWO => {
    //             //
    //         },
    //         Phase::THREE => {
    //             //
    //         },
    //         Phase::FOUR => {
    //             //
    //         },
    //         Phase::FIVE => {
    //             //
    //         },
    //     }
    // }
}


pub struct Network {
    vertices:   Vec<Vertice>,
    copies:     usize,
}

impl Network {
    pub fn new(n: usize, d: usize) -> Self {
        let mut graph = Network{vertices: Vec::new(), copies: 1};
        graph.vertices.push(Vertice::new(1, Phase::FOUR, true, d));
        for id in 2..=n {
            graph.vertices.push(Vertice::new(id, Phase::ONE, false, d));
        }
        graph
    }

    fn request_on_vertice(&mut self, vertice_id: usize, request_id: usize, request_type: RequestType) -> usize {
        let &mut vertice = *(self.vertices[vertice_id-1]);
        match vertice.get_phase() {
            Phase::ONE => {
                if request_id == self.id {
                    if (self.counter < self.d_value && is_read_request) || (!is_read_request && )
                }
            },
            Phase::TWO => {
                //
            },
            Phase::THREE => {
                //
            },
            Phase::FOUR => {
                //
            },
            Phase::FIVE => {
                //
            },
        }
    }

}


