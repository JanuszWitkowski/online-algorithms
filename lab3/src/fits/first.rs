use crate::fits::fit::{Bin, Fit};

const NAME_FIRST: &str = "FirstFit";

pub struct FirstFit {
    bins:   Vec<Bin>,
}

impl Fit for FirstFit {
    fn new() -> Self {
        FirstFit{ bins: Vec::new() }
    }

    fn name(&self) -> &'static str {
        NAME_FIRST
    }

    fn add(&mut self, elem: f64) {
        let mut i = 0;
        while i < self.bins.len() && !self.bins[i].add(elem) {
            i += 1;
        }
        if i == self.bins.len() {
            self.bins.push(Bin::new());
            self.bins[i].add(elem);
        }
    }

    fn bins_number(&self) -> usize {
        self.bins.len()
    }
}
