// mod update_list {
    pub trait UpdateList {
        fn new() -> Self;
        fn access(&mut self, x: u8) -> u8;
    }
    
    // fn access_search(list: &Vec<u8>, x: u8) -> (u8, u8, usize) {
    //     //
    // }
    
    pub struct ULClassic {
        list: Vec<u8>,
    }
    impl UpdateList for ULClassic {
        fn new() -> Self {
            return ULClassic{list: Vec::new()};
        }
        fn access(&mut self, x: u8) -> u8 {
            let mut cost: u8 = 0;
            let mut i = 0;
            let n = self.list.len();
            while i < n && self.list[i] != x {
                cost += 1;
                i += 1;
            }
            if i == n {
                self.list.insert(n, x);
            } else {
                cost += 1;
            }
            return cost;
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
            return 0;
        }
    }
    
    pub struct ULTranspose {
        list: Vec<u8>,
    }
    
    pub struct ULCount {
        list: Vec<(u8, u32)>,
    }
    
// }
