#[derive(PartialEq, Clone, Copy)]
enum Phase {
    ONE,
    TWO,
    THREE,
    FOUR,
    FIVE,
}

#[derive(PartialEq, Clone, Copy)]
enum RequestType {
    READ,
    WRITE,
}


struct Vertice {
    id:         usize,
    counter:    usize,
    phase:      Phase,
    has_copy:   bool,
    d_value:    usize,
}

impl Vertice {
    fn new(id: usize, phase: Phase, has_copy: bool, d_value: usize) -> Self {
        Vertice{ id, counter: 0, phase, has_copy, d_value }
    }

    fn get_id(&self) -> usize {
        self.id
    }

    fn get_counter(&self) -> usize {
        self.counter
    }

    fn counter_inc(&mut self) {
        self.counter += 1;
    }

    fn counter_dec(&mut self) {
        self.counter -= 1;
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

    fn set_copy(&mut self) {
        self.has_copy = true;
    }

    fn unset_copy(&mut self) {
        self.has_copy = false;
    }

    fn set_phase(&mut self, new_phase: Phase) {
        self.phase = new_phase;
    }

    fn get_phase(&self) -> Phase {
        self.phase
    }

    fn is_waiting(&self) -> bool {
        self.phase == Phase::FOUR
    }

}


pub struct Network {
    vertices:   Vec<Vertice>,
    copies:     usize,
    max_copies: usize,
    waiting:    usize,
    d_value:    usize,
    acc_cost:   usize,
    acc_copies: usize,
    req_ctr:    usize,
}

impl Network {
    pub fn new(n: usize, d: usize) -> Self {
        let mut graph = Network{ 
            vertices: Vec::new(), 
            copies: 1, 
            max_copies: 1,
            waiting: 1, 
            d_value: d,
            acc_cost: 0,
            acc_copies: 1,
            req_ctr: 0,
        };
        graph.vertices.push(Vertice::new(1, Phase::FOUR, true, d));
        for id in 2..=n {
            graph.vertices.push(Vertice::new(id, Phase::ONE, false, d));
        }
        graph
    }


    fn request_cost(&self, request_type: RequestType, vertice_id: usize) -> usize {
        match request_type {
            RequestType::READ => {
                match self.vertice_has_copy(vertice_id) {
                    true => 0,
                    false => 1,
                }
            },
            RequestType::WRITE => {
                self.copies - match self.vertice_has_copy(vertice_id) {
                    true => 1,
                    false => 0,
                }
            }
        }
    }



    // VERTICE HANDLERS
    fn vertice_get_phase(&self, vertice_id: usize) -> Phase {
        self.vertices[vertice_id-1].get_phase()
    }

    fn vertice_set_phase(&mut self, vertice_id: usize, new_phase: Phase) {
        self.vertices[vertice_id-1].set_phase(new_phase);
    }

    fn vertice_counter_full(&self, vertice_id: usize) -> bool {
        self.vertices[vertice_id-1].counter_full()
    }

    fn vertice_counter_empty(&self, vertice_id: usize) -> bool {
        self.vertices[vertice_id-1].counter_empty()
    }

    fn vertice_counter_inc(&mut self, vertice_id: usize) {
        self.vertices[vertice_id-1].counter_inc();
    }

    fn vertice_counter_dec(&mut self, vertice_id: usize) {
        self.vertices[vertice_id-1].counter_dec();
    }

    fn vertice_has_copy(&self, vertice_id: usize) -> bool {
        self.vertices[vertice_id-1].has_copy()
    }

    fn vertice_set_copy(&mut self, vertice_id: usize) {
        self.vertices[vertice_id-1].set_copy();
    }

    fn vertice_unset_copy(&mut self, vertice_id: usize) {
        self.vertices[vertice_id-1].unset_copy();
    }



    fn request_on_vertice(&mut self, request_type: RequestType, request_id: usize, vertice_id: usize) -> usize {
        let mut cost = 0;
        loop {
            match self.vertice_get_phase(vertice_id) {
                Phase::ONE => {
                    if request_id == vertice_id {
                        if (!self.vertice_counter_full(vertice_id) && request_type == RequestType::READ) || 
                                (request_type == RequestType::WRITE && self.copies == 1 && self.waiting > 0) {
                            self.vertice_counter_inc(vertice_id);
                            cost += self.request_cost(request_type, vertice_id);
                        }
                    }
                    if self.vertice_counter_full(vertice_id) {
                        self.vertice_set_phase(vertice_id, Phase::TWO);
                    } else {
                        break;
                    }
                },
                Phase::TWO => {
                    self.vertice_set_copy(vertice_id);
                    self.copies += 1;
                    if self.copies > self.max_copies {
                        self.max_copies = self.copies;
                    }
                    self.vertice_set_phase(vertice_id, Phase::THREE);
                },
                Phase::THREE => {
                    // todo: pamiętaj żeby przy wyjściu z tej fazy dodać waiting+=1
                },
                Phase::FOUR => {
                    if self.vertices.iter().filter(|v| v.has_copy()).count() == 1 {
                        break;
                    } else {
                        self.waiting -= 1;
                        self.vertice_set_phase(vertice_id, Phase::FIVE);
                    }
                },
                Phase::FIVE => {
                    self.vertice_unset_copy(vertice_id);
                    self.copies -= 1;
                    self.vertice_set_phase(vertice_id, Phase::ONE);
                },
            }
        }
        cost
    }

    pub fn request(&mut self, request_type: RequestType, request_id: usize) -> usize {
        let mut cost = 0;
        let mut total_cost = 0;
        for vertice_id in 1..=self.vertices.len() {
            cost = self.request_on_vertice(request_type, request_id, vertice_id);
            total_cost += cost;
            self.req_ctr += 1;
        }
        self.acc_copies += self.copies;
        self.acc_cost += total_cost;
        total_cost
    }

    pub fn results(&self) -> (f64, usize, f64) {
        (
            self.acc_cost as f64 / self.req_ctr as f64, 
            self.max_copies, 
            self.acc_copies as f64 / self.req_ctr as f64
        )
    }

}


