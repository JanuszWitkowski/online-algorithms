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
            self.cache.insert(position, elem);
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
                    for i in 0..self.reg.len() {
                        if !self.mark[i] {
                            if random_unmarked == 0 {
                                self.reg.replace(elem, i);
                                self.mark[i] = true;
                                self.total_unmarked -= 1;
                                break;
                            }
                            random_unmarked -= 1;
                        }
                    }
                }
                COST_FAULT
            },
        }
    }
}



#[cfg(test)]
mod tests {
    use crate::cache::Register;

    #[test]
    fn test_register_basics() {
        let max_length: usize = 5;
        let mut reg = Register::new(max_length);
        assert!(reg.is_empty());
        assert!(!reg.is_full());
        assert_eq!(reg.max_len(), max_length);

        let middle_length: usize = 3;
        for elem in 1..=middle_length {
            reg.push(elem);
        }
        assert!(!reg.is_empty());
        assert!(!reg.is_full());
        assert_eq!(reg.len(), middle_length);

        for elem in (middle_length + 1)..=max_length {
            reg.push(elem);
        }
        assert!(!reg.is_empty());
        assert!(reg.is_full());

        reg.remove(0);
        assert!(!reg.is_empty());
        assert!(!reg.is_full());
        assert_eq!(reg.len(), max_length - 1);

        let special_elem: usize = 69;
        reg.insert(special_elem, 0);
        assert!(!reg.is_empty());
        assert!(reg.is_full());
        assert!(reg.contains(special_elem));
        assert_eq!(reg.index_of(special_elem), Some(0));

        reg.remove(0);
        assert!(!reg.is_empty());
        assert!(!reg.is_full());
        assert!(!reg.contains(special_elem));

        reg.clear();
        assert!(reg.is_empty());
        assert!(!reg.is_full());
    }

    #[test]
    fn test_register_adding_over_limit() {
        let max_length: usize = 5;
        let mut reg = Register::new(max_length);
        for elem in 1..=max_length {
            reg.push(elem);
        }
        assert!(reg.is_full());

        let elem_over_limit = max_length + 1;
        reg.push(elem_over_limit);
        assert!(reg.is_full());
        assert!(!reg.contains(elem_over_limit));

        reg.insert(elem_over_limit, 0);
        assert!(reg.is_full());
        assert!(!reg.contains(elem_over_limit));

        reg.clear();
        reg.push(elem_over_limit);
        assert!(reg.contains(elem_over_limit));
    }

    #[test]
    fn test_register_removing_out_of_bounds() {
        let max_length: usize = 5;
        let mut reg = Register::new(max_length);
        assert!(reg.is_empty());
        reg.remove(0);
        assert!(reg.is_empty());
        assert_eq!(reg.max_len(), max_length);
        reg.remove(1);
        assert!(reg.is_empty());
        assert_eq!(reg.max_len(), max_length);

        reg.push(1);
        assert!(!reg.is_empty());
        reg.remove(1);
        assert!(!reg.is_empty());

        reg.clear();
        for elem in 0..max_length {
            reg.push(elem);
        }
        reg.remove(0);
        assert!(!reg.is_empty());
        assert!(!reg.is_full());
        let current_length = reg.len();
        reg.remove(max_length);
        assert_eq!(reg.len(), current_length);
        reg.remove(current_length);
        assert_eq!(reg.len(), current_length);
    }
}
