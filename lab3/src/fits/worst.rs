use crate::fits::fit::{Bin, Fit};

const NAME_WORST: &str = "WorstFit";

pub struct WorstFit {
    bins:   Vec<Bin>,
}

impl Fit for WorstFit {
    fn new() -> Self {
        WorstFit{ bins: Vec::new() }
    }

    fn name(&self) -> &'static str {
        NAME_WORST
    }

    fn add(&mut self, elem: f64) {
        if self.bins.len() == 0 {
            let mut bin = Bin::new();
            bin.add(elem);
            self.bins.push(bin);
        } else {
            let mut bin;
            if self.bins[0].add(elem) {     // If adding an element was successful
                bin = self.bins.remove(0);
            } else {
                bin = Bin::new();
                bin.add(elem);
            }
            // Sorting (ascending by amount of space taken)
            // TODO: Perhaps change it to binary search?
            let content = bin.show();
            let i = 0;
            while i < self.bins.len() && self.bins[i].show() < content {
                i += 1;
            }
            self.bins.insert(i, bin);
            }
    }

    fn bins_number(&self) -> usize {
        self.bins.len()
    }
}
