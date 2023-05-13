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
        // let mut able_to_store: Vec<&mut Bin> = self.bins.iter().filter(|bin| bin.can_store(elem)).collect();
        let mut able_to_store: Vec<*mut Bin> = Vec::new();
        for mut bin in &mut *self.bins {
            if bin.can_store(elem) {
                able_to_store.push(bin);
            }
        }
        if able_to_store.len() == 0 {
            let mut bin = Bin::new();
            bin.add(elem);
            self.bins.push(bin);
        } else {
            let chosen = fastrand::usize(..able_to_store.len());
            &able_to_store[chosen].add(elem);
        }
        // if self.bins.is_empty() {
        //     let mut bin = Bin::new();
        //     bin.add(elem);
        //     self.bins.push(bin);
        // } else {
        //     let mut bin;
        //     if self.bins[0].add(elem) {     // If adding an element was successful
        //         bin = self.bins.remove(0);
        //     } else {
        //         bin = Bin::new();
        //         bin.add(elem);
        //     }
        //     // Sorting (ascending by amount of space taken)
        //     // TODO: Perhaps change it to binary search?
        //     let content = bin.show();
        //     let mut i = 0;
        //     while i < self.bins.len() && self.bins[i].show() < content {
        //         i += 1;
        //     }
        //     self.bins.insert(i, bin);
        //     }
    }

    fn reset(&mut self) {
        self.bins.clear();
    }

    fn bins_number(&self) -> usize {
        self.bins.len()
    }
}
