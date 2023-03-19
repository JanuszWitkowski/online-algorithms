// TRAIT
// Each UpdateList has to implement Access function. Clear is for tests. Print is for debugging.
pub trait UpdateList {
    fn new() -> Self where Self: Sized;
    fn access(&mut self, x: u8) -> u8;
    fn print(&mut self);
    fn clear(&mut self);
    fn name(&mut self) -> &'static str;
}


const NAME_CLASSIC: &'static str = "classic";
const NAME_MOVE_TO_FRONT: &'static str = "move-to-front";
const NAME_TRANSPOSE: &'static str = "transpose";
const NAME_COUNT: &'static str = "count";


// HELPER FUNCTIONS
fn access_search(list: &mut Vec<u8>, x: u8) -> (u8, usize, bool) {
    let mut cost: u8 = 0;
    let mut i = 0;
    let inserted: bool;
    let n = list.len();
    while i < n && list[i] != x {
        cost += 1;
        i += 1;
    }
    if i == n {
        list.insert(n, x);
        inserted = true;
    } else {
        inserted = false;
        cost += 1;
    }
    return (cost, i, inserted);   // (cost, index of the element found/inserted)
}

fn print_list(list: &Vec<u8>) {
    print!("[");
    for el in list {
        print!(" {}", el);
    }
    println!(" ]");
}


// IMPLEMENTATIONS
pub struct ULClassic {
    list: Vec<u8>,
}
impl UpdateList for ULClassic {
    fn new() -> Self {
        return ULClassic{list: Vec::new()};
    }
    fn access(&mut self, x: u8) -> u8 {
        let (cost, _, _) = access_search(&mut self.list, x);
        return cost;
    }
    fn print(&mut self) {
        print_list(&self.list);
    }
    fn clear(&mut self) {
        self.list.clear();
    }
    fn name(&mut self) -> &'static str {
        NAME_CLASSIC
    }
}

pub struct ULMoveToFront {
    list: Vec<u8>,
}
impl UpdateList for ULMoveToFront {
    fn new() -> Self {
        return ULMoveToFront { list: Vec::new() }
    }
    fn access(&mut self, x: u8) -> u8 {
        let (cost, index, _) = access_search(&mut self.list, x);
        self.list.remove(index);
        self.list.insert(0, x);
        return cost;
    }
    fn print(&mut self) {
        print_list(&self.list);
    }
    fn clear(&mut self) {
        self.list.clear();
    }
    fn name(&mut self) -> &'static str {
        NAME_MOVE_TO_FRONT
    }
}

pub struct ULTranspose {
    list: Vec<u8>,
}
impl UpdateList for ULTranspose {
    fn new() -> Self {
        return ULTranspose { list: Vec::new() }
    }
    fn access(&mut self, x: u8) -> u8 {
        let (cost, index, _) = access_search(&mut self.list, x);
        if index > 0 {
            self.list.remove(index);
            self.list.insert(index - 1, x);
        }
        return cost;
    }
    fn print(&mut self) {
        print_list(&self.list);
    }
    fn clear(&mut self) {
        self.list.clear();
    }
    fn name(&mut self) -> &'static str {
        NAME_TRANSPOSE
    }
}

pub struct ULCount {
    list: Vec<u8>,
    freq: Vec<u32>,
}
impl UpdateList for ULCount {
    fn new() -> Self {
        return ULCount { list: Vec::new(), freq: Vec::new() }
    }
    fn access(&mut self, x: u8) -> u8 {
        let (cost, index, inserted) = access_search(&mut self.list, x);
        if inserted {
            self.freq.insert(index, 0);
        }
        let mut i: i8 = (index as i8) - 1;
        let frequency = self.freq[index] + 1;
        self.list.remove(index);
        self.freq.remove(index);
        while i >= 0 && self.freq[i as usize] <= frequency {
            i -= 1;
        }
        self.list.insert((i + 1) as usize, x);
        self.freq.insert((i + 1) as usize, frequency);
        return cost;
    }
    fn print(&mut self) {
        print!("[");
        for i in 0..self.list.len() {
            print!(" {}({})", self.list[i], self.freq[i]);
        }
        println!(" ]");
    }
    fn clear(&mut self) {
        self.list.clear();
        self.freq.clear();
    }
    fn name(&mut self) -> &'static str {
        NAME_COUNT
    }
}

