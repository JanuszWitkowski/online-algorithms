use rand::Rng; // 0.8.5

const COST_ACCESS:  usize = 0;
const COST_FAULT:   usize = 1;

const NAME_FIFO:    &'static str = "First-In-First-Out";
const NAME_FWF:     &'static str = "Flush-When-Full";
const NAME_LRU:     &'static str = "Least-Recently-Used";
const NAME_LFU:     &'static str = "Least-Frequently-Used";
const NAME_RAND:    &'static str = "Random";
const NAME_RMA:     &'static str = "Random-Markup-Algorithm";


pub trait Cache {
    fn access(&mut self, elem: usize) -> usize;
    fn name(&self) -> &'static str;
}


// Private struct for cache with variable-length
struct Register {
    cache: Vec<usize>,
    max_len: usize,
}
impl Register {
    fn new(max_length: usize) -> Self {
        Register{
            cache: Vec::new(),
            max_len: max_length
        }
    }
    fn is_full(&self) -> bool {
        self.cache.len() == self.max_len
    }
    fn is_empty(&self) -> bool {
        self.cache.len() == 0
    }
    fn len(&self) -> usize {
        self.cache.len()
    }
    fn max_len(&self) -> usize {
        self.max_len
    }
    fn contains(&self, elem: usize) -> bool {
        if self.cache.contains(&elem) {
            return true;
        }
        false
    }
    fn index_of(&self, elem: usize) -> Option<usize> {
        for index in 0..self.cache.len() {
            if self.cache[index] == elem {
                return Some(index);
            }
        }
        None
    }
    fn insert(&mut self, elem: usize, position: usize) {
        if self.cache.len() < self.max_len {
            self.cache.insert(elem, position);
        }
    }
    fn push(&mut self, elem: usize) {
        if self.cache.len() < self.max_len {
            self.cache.push(elem);
        }
    }
    fn replace(&mut self, elem:usize, position: usize) {
        if position < self.cache.len() {
            self.cache[position] = elem;
        }
    }
    fn remove(&mut self, position: usize) {
        if position < self.cache.len() {
            self.cache.remove(position);
        }
    }
    fn clear(&mut self) {
        self.cache.clear();
    }
}


// First In First Out
pub struct FIFO {
    reg: Register,
}
impl FIFO {
    pub fn new(max_length: usize) -> Self {
        FIFO { reg: Register::new(max_length) }
    }
}
impl Cache for FIFO {
    fn name(&self) -> &'static str {
        NAME_FIFO
    }
    fn access(&mut self, elem: usize) -> usize {
        match self.reg.contains(elem) {
            true => COST_ACCESS,
            false => {
                if self.reg.is_full() {
                    self.reg.remove(0);
                }
                self.reg.push(elem);
                COST_FAULT
            },
        }
    }
}

// Flush When Full
pub struct FWF {
    reg: Register,
}
impl FWF {
    pub fn new(max_length: usize) -> Self {
        FWF { reg: Register::new(max_length) }
    }
}
impl Cache for FWF {
    fn name(&self) -> &'static str {
        NAME_FWF
    }
    fn access(&mut self, elem: usize) -> usize {
        match self.reg.contains(elem) {
            true => COST_ACCESS,
            false => {
                if self.reg.is_full() {
                    self.reg.clear();
                }
                self.reg.push(elem);
                COST_FAULT
            },
        }
    }
}

// Random
pub struct RAND {
    reg: Register,
}
impl RAND {
    pub fn new(max_length: usize) -> Self {
        RAND { reg: Register::new(max_length) }
    }
}
impl Cache for RAND {
    fn name(&self) -> &'static str {
        NAME_RAND
    }
    fn access(&mut self, elem: usize) -> usize {
        match self.reg.contains(elem) {
            true => COST_ACCESS,
            false => {
                if self.reg.is_full() {
                    // let index = rand::thread_rng().gen_range(1..=self.reg.max_len());
                    // self.reg.remove(index);
                    self.reg.remove(rand::thread_rng().gen_range(0..self.reg.max_len()));
                }
                self.reg.push(elem);
                COST_FAULT
            },
        }
    }
}

// TODO: Maybe use some kind of structure for this?
// Least Recently Used
// pub struct LRU {
//     reg: Register,
// }
// impl LRU {
//     pub fn new(max_length: usize) -> Self {
//         LRU { reg: Register::new(max_length) }
//     }
// }
// impl Cache for LRU {
//     fn name(&self) -> &'static str {
//         NAME_LRU
//     }
//     fn access(&mut self, elem: usize) -> usize {    // TODO: Implement LRU properly
//         match self.reg.contains(elem) {
//             true => COST_ACCESS,
//             false => {
//                 if self.reg.is_full() {
//                     self.reg.remove(0);
//                 }
//                 self.reg.push(elem);
//                 COST_FAULT
//             },
//         }
//     }
// }

// TODO: Maybe use some kind of Heap for this?
// Least Frequently Used
// pub struct LFU {
//     reg: Register,
//     usage: 
// }
// impl LFU {
//     pub fn new(max_length: usize) -> Self {
//         LFU { reg: Register::new(max_length) }
//     }
// }
// impl Cache for LFU {
//     fn name(&self) -> &'static str {
//         NAME_LFU
//     }
//     fn access(&mut self, elem: usize) -> usize {
//         match self.reg.contains(elem) {
//             true => COST_ACCESS,
//             false => {
//                 if self.reg.is_full() {
//                     self.reg.remove(0);
//                 }
//                 self.reg.push(elem);
//                 COST_FAULT
//             },
//         }
//     }
// }

// Random Markup Algorithm
pub struct RMA {
    reg: Register,
    mark: Vec<bool>,
    total_unmarked: usize,
}
impl RMA {
    pub fn new(max_length: usize) -> Self {
        RMA { 
            reg: Register::new(max_length),
            mark: Vec::new(),
            total_unmarked: 0
        }
    }
}
impl Cache for RMA {
    fn name(&self) -> &'static str {
        NAME_RMA
    }
    fn access(&mut self, elem: usize) -> usize {
        match self.reg.index_of(elem) {
            Some(index) => {
                if !self.mark[index] {
                    self.total_unmarked -= 1;
                }
                self.mark[index] = true;
                COST_ACCESS
            },
            None => {
                if !self.reg.is_full() {
                    self.reg.push(elem);
                    self.mark.push(true);
                }
                else {
                    if self.total_unmarked == 0 {
                        self.mark = self.mark.iter().map(|_| false).collect();
                        self.total_unmarked = self.reg.max_len();
                    }
                    let mut random_unmarked = rand::thread_rng().gen_range(0..self.total_unmarked);
                    // for i in 0..self.reg.len() {
                    //     if self.
                    // }
                    self.reg.replace(elem, 0);  // TODO: DOKONCZYC I POPRAWIC!!!
                }
                COST_FAULT
            },
        }
    }
}
