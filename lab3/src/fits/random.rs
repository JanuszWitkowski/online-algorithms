use crate::fits::fit::{Bin, Fit};
use fastrand;

const NAME_RANDOM: &str = "RandomFit";

pub struct RandomFit {
    bins:   Vec<Bin>,
}

impl Fit for RandomFit {
    fn new() -> Self {
        RandomFit{ bins: Vec::new() }
    }

    fn name(&self) -> &'static str {
        NAME_RANDOM
    }

    fn add(&mut self, elem: f64) {
        let mut indexes_able_to_store: Vec<usize> = Vec::new();
        for i in 0..self.bins.len() {
            if self.bins[i].can_store(elem) {
                indexes_able_to_store.push(i);
            }
        }
        if indexes_able_to_store.is_empty() {
            let mut bin = Bin::new();
            bin.add(elem);
            self.bins.push(bin);
        } else {
            let chosen = fastrand::usize(..indexes_able_to_store.len());
            self.bins[indexes_able_to_store[chosen]].add(elem);
        }
    }

    fn reset(&mut self) {
        self.bins.clear();
    }

    fn bins_number(&self) -> usize {
        self.bins.len()
    }
}
