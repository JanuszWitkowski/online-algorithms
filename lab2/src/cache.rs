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
    fn change_len(&mut self, new_len: usize, new_n: usize);
    fn clear(&mut self);
    fn name(&self) -> &'static str;
    fn print(&self);
}


// Private struct for cache with variable-length
#[derive(Clone)]
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

    // fn is_empty(&self) -> bool {
    //     self.cache.len() == 0
    // }

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

    fn print(&self) {
        print!("[");
        for elem in &self.cache {
            print!(" {}", elem);
        }
        println!(" ]");
    }
}



// Private struct for priority-queue-type cache
#[derive(Clone)]
struct BinHeap {
    heap:   Vec<(usize, usize)>,
    max_len: usize
}

impl BinHeap {
    fn new(max_length: usize) -> Self {
        BinHeap { 
            heap: Vec::new(),
            max_len: max_length
        }
    }

    // fn parent(&self, i: usize) -> Option<usize> {
    //     match i > 0 {
    //         true => Some((i - 1) / 2),
    //         false => None,
    //     }
    // }

    fn left(&self, i: usize) -> Option<usize> {
        match 2 * i + 1 < self.heap.len() {
            true => Some(2 * i + 1),
            false => None,
        }
    }

    fn right(&self, i: usize) -> Option<usize> {
        match 2 * (i + 1) < self.heap.len() {
            true => Some(2 * (i + 1)),
            false => None,
        }
    }

    fn swap(&mut self, i1: usize, i2: usize) {
        let tmp = self.heap[i1];
        self.heap[i1] = self.heap[i2];
        self.heap[i2] = tmp;
    }

    // lil' bit ugly, should find a functional alternative...
    fn smallest_index(&self, i: usize, indexes: [Option<usize>; 2]) -> usize {
        let mut extreme_idx = i;
        for mi in indexes {
            extreme_idx = match mi {
                Some(idx) => {
                    if self.heap[extreme_idx].0 < self.heap[idx].0 {
                        extreme_idx
                    } else {
                        idx
                    }
                },
                None => extreme_idx
            }
        }
        extreme_idx
    }

    fn heapify(&mut self, i: usize) {
        let smallest_index = self.smallest_index(i, [self.left(i), self.right(i)]);
        if smallest_index != i {
            self.swap(i, smallest_index);
            self.heapify(smallest_index);
        }
    }

    fn inc_key(&mut self, i: usize) {
        self.heap[i] = (self.heap[i].0 + 1, self.heap[i].1);
        self.heapify(i);
    }

    fn is_full(&self) -> bool {
        self.heap.len() == self.max_len
    }

    // fn is_empty(&self) -> bool {
    //     self.heap.len() == 0
    // }

    // fn len(&self) -> usize {
    //     self.heap.len()
    // }

    // fn max_len(&self) -> usize {
    //     self.max_len
    // }

    fn index_of(&self, value: usize) -> Option<usize> {
        for index in (0..self.heap.len()).rev() {
            if self.heap[index].1 == value {
                return Some(index);
            }
        }
        None
    }

    fn push(&mut self, elem: (usize, usize)) {
        if self.heap.len() < self.max_len {
            self.heap.insert(0, elem);
            // self.heapify(0);
            self.inc_key(0);
        }
    }

    // fn delete_smallest(&mut self) {
    //     if self.heap.len() > 0 {
    //         self.heap.remove(0);
    //     }
    // }

    fn pop(&mut self) -> (usize, usize) {
        let (key, value) = self.heap[0];
        self.heap.remove(0);
        (key, value)
    }

    // fn pop_key(&mut self) -> usize {
    //     let key = self.heap[0].0;
    //     self.heap.remove(0);
    //     key
    // }

    fn clear(&mut self) {
        self.heap.clear();
    }

    fn print(&self) {
        print!("[");
        for elem in &self.heap {
            print!(" {}({})", elem.1, elem.0);
        }
        println!(" ]");
    }
}



// First In First Out
#[derive(Clone)]
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

    fn print(&self) {
        self.reg.print();
    }

    fn clear(&mut self) {
        self.reg.clear();
    }

    fn change_len(&mut self, new_len: usize, new_n: usize) {
        self.reg = Register::new(new_len);
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
#[derive(Clone)]
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

    fn print(&self) {
        self.reg.print();
    }

    fn clear(&mut self) {
        self.reg.clear();
    }

    fn change_len(&mut self, new_len: usize, new_n: usize) {
        self.reg = Register::new(new_len);
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
#[derive(Clone)]
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

    fn print(&self) {
        self.reg.print();
    }

    fn clear(&mut self) {
        self.reg.clear();
    }

    fn change_len(&mut self, new_len: usize, new_n: usize) {
        self.reg = Register::new(new_len);
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


// Least Recently Used
#[derive(Clone)]
pub struct LRU {
    reg: Register,
}

impl LRU {
    pub fn new(max_length: usize) -> Self {
        LRU { reg: Register::new(max_length) }
    }
}

impl Cache for LRU {
    fn name(&self) -> &'static str {
        NAME_LRU
    }

    fn print(&self) {
        self.reg.print();
    }

    fn clear(&mut self) {
        self.reg.clear();
    }

    fn change_len(&mut self, new_len: usize, new_n: usize) {
        self.reg = Register::new(new_len);
    }

    fn access(&mut self, elem: usize) -> usize {
        match self.reg.index_of(elem) {
            Some(index) => {
                // Move the element to the front - it was recently used
                self.reg.remove(index);
                self.reg.insert(elem, 0);
                COST_ACCESS
            },
            None => {
                // Remove the last element, as it is the least recently used one
                if self.reg.is_full() {
                    self.reg.remove(self.reg.max_len() - 1);
                }
                // Move the new element to the front
                self.reg.insert(elem, 0);
                COST_FAULT
            },
        }
    }
}


// Least Frequently Used
#[derive(Clone)]
pub struct LFU {
    reg: BinHeap,
    usage: Vec<(usize, usize)>
}

impl LFU {
    pub fn new(max_length: usize, n: usize) -> Self {
        let mut lfu = LFU { 
            reg: BinHeap::new(max_length),
            usage: Vec::new()
        };
        for i in 1..=n {
            lfu.usage.push((0, i));
        }
        lfu
    }

    fn get_elem_with_value(&self, value: usize) -> (usize, usize) {
        self.usage[value - 1]
    }

    fn update_key_for_value(&mut self, key: usize, value: usize) {
        self.usage[value - 1] = (key, value);
    }

    // pub fn print_usage(&self) {
    //     print!("{{");
    //     for (k, v) in &self.usage {
    //         print!(" {}({})", v, k);
    //     }
    //     println!(" }}");
    // }

}

impl Cache for LFU {
    fn name(&self) -> &'static str {
        NAME_LFU
    }

    fn print(&self) {
        self.reg.print();
    }

    fn clear(&mut self) {
        self.reg.clear();
        for i in 0..self.usage.len() {
            self.usage[i].0 = 0;
        }
    }

    fn change_len(&mut self, new_len: usize, new_n: usize) {
        self.reg = BinHeap::new(new_len);
        self.usage = Vec::new();
        for i in 1..=new_n {
            self.usage.push((0, i));
        }
    }

    fn access(&mut self, value: usize) -> usize {
        match self.reg.index_of(value) {
            Some(index) => {
                self.reg.inc_key(index);
                COST_ACCESS
            },
            None => {
                if self.reg.is_full() {
                    let (key, old_value) = self.reg.pop();
                    self.update_key_for_value(key, old_value);
                }
                self.reg.push(self.get_elem_with_value(value));
                COST_FAULT
            },
        }
    }
}


// Random Markup Algorithm
#[derive(Clone)]
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

    fn print(&self) {
        self.reg.print();
    }

    fn clear(&mut self) {
        self.reg.clear();
    }

    fn change_len(&mut self, new_len: usize, new_n: usize) {
        self.reg = Register::new(new_len);
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




// #[cfg(test)]
// mod tests {
//     use crate::cache::{Register, Cache, FIFO, LRU, LFU};
//     use crate::cache::{NAME_FIFO, NAME_LRU, NAME_LFU};
//     use crate::cache::{COST_ACCESS, COST_FAULT};

//     #[test]
//     fn test_register_basics() {
//         let max_length: usize = 5;
//         let mut reg = Register::new(max_length);
//         assert!(reg.is_empty());
//         assert!(!reg.is_full());
//         assert_eq!(reg.max_len(), max_length);

//         let middle_length: usize = 3;
//         for elem in 1..=middle_length {
//             reg.push(elem);
//         }
//         assert!(!reg.is_empty());
//         assert!(!reg.is_full());
//         assert_eq!(reg.len(), middle_length);

//         for elem in (middle_length + 1)..=max_length {
//             reg.push(elem);
//         }
//         assert!(!reg.is_empty());
//         assert!(reg.is_full());

//         reg.remove(0);
//         assert!(!reg.is_empty());
//         assert!(!reg.is_full());
//         assert_eq!(reg.len(), max_length - 1);

//         let special_elem: usize = 69;
//         reg.insert(special_elem, 0);
//         assert!(!reg.is_empty());
//         assert!(reg.is_full());
//         assert!(reg.contains(special_elem));
//         assert_eq!(reg.index_of(special_elem), Some(0));

//         reg.remove(0);
//         assert!(!reg.is_empty());
//         assert!(!reg.is_full());
//         assert!(!reg.contains(special_elem));

//         reg.clear();
//         assert!(reg.is_empty());
//         assert!(!reg.is_full());
//     }

//     #[test]
//     fn test_register_adding_over_limit() {
//         let max_length: usize = 5;
//         let mut reg = Register::new(max_length);
//         for elem in 1..=max_length {
//             reg.push(elem);
//         }
//         assert!(reg.is_full());

//         let elem_over_limit = max_length + 1;
//         reg.push(elem_over_limit);
//         assert!(reg.is_full());
//         assert!(!reg.contains(elem_over_limit));

//         reg.insert(elem_over_limit, 0);
//         assert!(reg.is_full());
//         assert!(!reg.contains(elem_over_limit));

//         reg.clear();
//         reg.push(elem_over_limit);
//         assert!(reg.contains(elem_over_limit));
//     }

//     #[test]
//     fn test_register_removing_out_of_bounds() {
//         let max_length: usize = 5;
//         let mut reg = Register::new(max_length);
//         assert!(reg.is_empty());
//         reg.remove(0);
//         assert!(reg.is_empty());
//         assert_eq!(reg.max_len(), max_length);
//         reg.remove(1);
//         assert!(reg.is_empty());
//         assert_eq!(reg.max_len(), max_length);

//         reg.push(1);
//         assert!(!reg.is_empty());
//         reg.remove(1);
//         assert!(!reg.is_empty());

//         reg.clear();
//         for elem in 0..max_length {
//             reg.push(elem);
//         }
//         reg.remove(0);
//         assert!(!reg.is_empty());
//         assert!(!reg.is_full());
//         let current_length = reg.len();
//         reg.remove(max_length);
//         assert_eq!(reg.len(), current_length);
//         reg.remove(current_length);
//         assert_eq!(reg.len(), current_length);
//     }

//     #[test]
//     fn test_register_fn_len() {
//         let max_length: usize = 5;
//         let mut reg = Register::new(max_length);
//         assert_eq!(reg.len(), 0);
//         for i in 1..=max_length {
//             reg.push(i);
//             assert_eq!(reg.len(), i);
//         }
//         assert_eq!(reg.len(), max_length);
//     }

//     // HEAP TESTS
//     // #[test]
//     // fn test_heap_

//     // FIFO TESTS
//     #[test]
//     fn test_fifo_new() {
//         let max_length: usize = 5;
//         let fifo = FIFO::new(max_length);
//         assert_eq!(fifo.name(), NAME_FIFO);
//     }

//     #[test]
//     fn test_fifo_access_adding() {
//         let max_length: usize = 5;
//         let mut fifo = FIFO::new(max_length);
//         let mut total: usize = 0;
//         let mut cost: usize;
//         for i in 1..=max_length {
//             cost = fifo.access(i);
//             assert_eq!(cost, COST_FAULT);
//             total += cost;
//         }
//         assert_eq!(total, max_length);
//     }

//     #[test]
//     fn test_fifo_access_existing_elems() {
//         let max_length: usize = 5;
//         let mut fifo = FIFO::new(max_length);
//         for i in 1..=max_length {
//             fifo.access(i);
//         }
//         let mut cost: usize;
//         for i in 1..=max_length {
//             cost = fifo.access(i);
//             assert_eq!(cost, COST_ACCESS);
//         }
//     }

//     #[test]
//     fn test_fifo_access_fault() {
//         let max_length: usize = 5;
//         let mut fifo = FIFO::new(max_length);
//         for i in 1..=max_length {
//             fifo.access(i);
//         }
//         // 1 2 3 4 5
//         let new_elem = max_length + 1;  // 6
//         let mut cost: usize = fifo.access(new_elem);
//         // 2 3 4 5 6
//         assert_eq!(cost, COST_FAULT);
//         for i in 2..new_elem {
//             cost = fifo.access(i);
//             assert_eq!(cost, COST_ACCESS);
//         }
//         cost = fifo.access(1);
//         // 3 4 5 6 1
//         assert_eq!(cost, COST_FAULT);
//         cost = fifo.access(1);
//         assert_eq!(cost, COST_ACCESS);
//         for i in 3..new_elem {
//             cost = fifo.access(i);
//             assert_eq!(cost, COST_ACCESS);
//         }
//     }

//     // LRU TESTS
//     #[test]
//     fn test_lru_new() {
//         let max_length = 5;
//         let lru = LRU::new(max_length);
//         assert_eq!(lru.name(), NAME_LRU);
//     }

//     #[test]
//     fn test_lru_access_adding() {
//         let max_length: usize = 5;
//         let mut lru = LRU::new(max_length);
//         let mut total: usize = 0;
//         let mut cost: usize;
//         for i in 1..=max_length {
//             cost = lru.access(i);
//             assert_eq!(cost, COST_FAULT);
//             total += cost;
//         }
//         assert_eq!(total, max_length);
//     }

//     #[test]
//     fn test_lru_access_existing_elems() {
//         let max_length: usize = 5;
//         let mut lru = LRU::new(max_length);
//         for i in 1..=max_length {
//             lru.access(i);
//         }
//         let mut cost: usize;
//         for i in 1..=max_length {
//             cost = lru.access(i);
//             assert_eq!(cost, COST_ACCESS);
//         }
//     }

//     #[test]
//     fn test_lru_access_fault() {
//         let max_length: usize = 5;
//         let mut lru = LRU::new(max_length);
//         for i in 1..=max_length {
//             lru.access(i);
//         }
//         // 5 4 3 2 1
//         lru.print();
//         let new_elem = max_length + 1;  // 6
//         println!("{}", new_elem);
//         let mut cost: usize = lru.access(new_elem);
//         // 6 5 4 3 2
//         lru.print();
//         assert_eq!(cost, COST_FAULT);
//         for i in (2..new_elem).rev() {
//             cost = lru.access(i);
//             assert_eq!(cost, COST_ACCESS);
//         }
//         // 2 3 4 5 6
//         lru.print();
//         cost = lru.access(1);
//         // 1 2 3 4 5
//         lru.print();
//         assert_eq!(cost, COST_FAULT);
//         cost = lru.access(1);
//         assert_eq!(cost, COST_ACCESS);
//         // for i in 3..max_length {
//         //     cost = fifo.access(i);
//         //     assert_eq!(cost, COST_ACCESS);
//         // }
//     }

//     // LFU TESTS
//     #[test]
//     fn test_lfu_new() {
//         let max_length = 5;
//         let n = 100;
//         let lfu = LFU::new(max_length, n);
//         assert_eq!(lfu.name(), NAME_LFU);
//     }

//     #[test]
//     fn test_lfu_access_adding() {
//         let max_length: usize = 5;
//         let n = 100;
//         let mut lfu = LFU::new(max_length, n);
//         let mut total: usize = 0;
//         let mut cost: usize;
//         for i in 1..=max_length {
//             cost = lfu.access(i);
//             assert_eq!(cost, COST_FAULT);
//             total += cost;
//         }
//         assert_eq!(total, max_length);
//     }

//     #[test]
//     fn test_lfu_access_existing_elems() {
//         let max_length: usize = 5;
//         let n = 100;
//         let mut lfu = LFU::new(max_length, n);
//         for i in 1..=max_length {
//             lfu.access(i);
//         }
//         let mut cost: usize;
//         for i in 1..=max_length {
//             cost = lfu.access(i);
//             assert_eq!(cost, COST_ACCESS);
//         }
//     }

//     #[test]
//     fn test_lfu_access_fault() {
//         let max_length: usize = 5;
//         let n = 100;
//         let mut lfu = LFU::new(max_length, n);
//         for i in 1..=max_length {
//             lfu.access(i);
//         }
//         // // 5 4 3 2 1
//         // lfu.print();
//         // let new_elem = max_length + 1;  // 6
//         // println!("{}", new_elem);
//         // let mut cost: usize = lfu.access(new_elem);
//         // // 6 5 4 3 2
//         // lfu.print();
//         // assert_eq!(cost, COST_FAULT);
//         // for i in (2..new_elem).rev() {
//         //     cost = lfu.access(i);
//         //     assert_eq!(cost, COST_ACCESS);
//         // }
//         // // 2 3 4 5 6
//         // lfu.print();
//         // cost = lfu.access(1);
//         // // 1 2 3 4 5
//         // lfu.print();
//         // assert_eq!(cost, COST_FAULT);
//         // cost = lfu.access(1);
//         // assert_eq!(cost, COST_ACCESS);
//         // // for i in 3..max_length {
//         // //     cost = fifo.access(i);
//         // //     assert_eq!(cost, COST_ACCESS);
//         // // }
//     }

//     // MISC
//     // #[test]
//     // fn test_div_negative() {
//     //     let zero: usize = 0;
//     //     let result: usize = (zero - 1) / 2;
//     //     println!("(0 - 1)/2 = {}", result);
//     //     assert!(false);
//     // }
// }
