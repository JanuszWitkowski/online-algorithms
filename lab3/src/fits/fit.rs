const LIMIT:    f64 = 1.0;

pub struct Bin {
    storage:    f64,
}

impl Bin {
    pub fn new() -> Self where Self: Sized {
        Bin{ storage: 0.0 }
    }

    // fn is_full(&self) -> bool {
    //     self.storage >= LIMIT 
    // }

    fn can_store(&self, elem: f64) -> bool {
        self.storage + elem <= LIMIT
    }

    fn unprotected_add(&self, elem: f64) {
        self.storage += elem;
    }

    pub fn add(&self, elem: f64) -> bool {
        match self.can_store(elem) {
            true => {
                self.unprotected_add(elem);
                true
            },
            false => false
        }
    }
}


pub trait Fit {
    fn new() -> Self where Self: Sized;
    fn name(&self) -> &'static str;
    fn add(&self, elem: f64);
    fn bins_number(&self) -> usize;
}
