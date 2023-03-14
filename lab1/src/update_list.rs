// mod update_list {
    pub trait UpdateList {
        fn new() -> Self where Self: Sized;
        fn access(&mut self, x: u8) -> u8;
        fn print(&mut self);
    }
    
    fn access_search(list: &mut Vec<u8>, x: u8) -> (u8, usize) {
        let mut cost: u8 = 0;
        let mut i = 0;
        let n = list.len();
        while i < n && list[i] != x {
            cost += 1;
            i += 1;
        }
        if i == n {
            list.insert(n, x);
        } else {
            cost += 1;
        }
        return (cost, i);   // (cost, index of the element found/inserted)
    }

    fn print_list(list: &Vec<u8>) {
        print!("[");
        for el in list {
            print!(" {}", el);
        }
        println!(" ]");
    }
    
    pub struct ULClassic {
        list: Vec<u8>,
    }
    impl UpdateList for ULClassic {
        fn new() -> Self {
            return ULClassic{list: Vec::new()};
        }
        fn access(&mut self, x: u8) -> u8 {
            let (cost, _) = access_search(&mut self.list, x);
            return cost;
        }
        fn print(&mut self) {
            print_list(&self.list);
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
            let (cost, index) = access_search(&mut self.list, x);
            self.list.remove(index);
            self.list.insert(0, x);
            return cost;
        }
        fn print(&mut self) {
            print_list(&self.list);
        }
    }
    
    pub struct ULTranspose {
        list: Vec<u8>,
    }
    
    pub struct ULCount {
        list: Vec<(u8, u32)>,
    }
    
// }
