trait UpdateList {
    fn new() -> Self;
    fn access(&self, x: u8) -> u8;
}

fn access_search(list: &Vec<u8>, x: u8) -> (u8, u8, usize) {
    //
}

pub struct ULClassic {
    list: Vec<u8>,
}
impl UpdateList for ULClassic {
    pub fn new() {
        return ULClassic{Vec::new()};
    }
    pub fn access(&self, x: u8) -> u8 {
        let cost: u8 = 0;
        let i: u8 = 0;
        let n = self.list.len();
        while i < n && self.list[i] != x {
            cost += 1;
            i += 1;
        }
        if i == n {
            self.list.insert(x);
        } else {
            cost += 1;
        }
        return cost;
    }
}

pub struct ULMoveToFront {
    list: Vec<u8>,
}

pub struct ULTranspose {
    list: Vec<u8>,
}

pub struct ULCount {
    list: Vec<(u8, u32)>,
}
