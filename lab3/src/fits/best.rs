use crate::fits::fit::{Bin, Fit};

const NAME_BEST: &str = "BestFit";

pub struct BestFit {
    bins:   Vec<Bin>,
}

impl Fit for BestFit {
    fn new() -> Self {
        BestFit{ bins: Vec::new() }
    }

    fn name(&self) -> &'static str {
        NAME_BEST
    }

    fn add(&mut self, elem: f64) {
        let mut i = 0;
        while i < self.bins.len() && !self.bins[i].add(elem) {
            i += 1;
        }
        let mut bin: Bin;
        if i == self.bins.len() {   // If there is no fitting box
            bin = Bin::new();
            bin.add(elem);      // The element has not been added yet
        } else {
            bin = self.bins.remove(i);
        }
        // Sorting (descending by amount of space taken)
        // TODO: Perhaps change it to binary search?
        let content = bin.show();
        while i > 0 && self.bins[i-1].show() > content {
            i -= 1;
        }
        self.bins.insert(i, bin);
    }

    fn bins_number(&self) -> usize {
        self.bins.len()
    }
}
