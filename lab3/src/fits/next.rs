use crate::fits::fit::{Bin, Fit};

const NAME_NEXT: &str = "NextFit";

pub struct NextFit {
    bin:            Bin,
    bins_number:    usize,
}

impl Fit for NextFit {
    fn new() -> Self {
        NextFit{ bin: Bin::new(), bins_number: 1 }
    }

    fn name(&self) -> &'static str {
        NAME_NEXT
    }

    fn add(&mut self, elem: f64) {
        if !self.bin.add(elem) {
            self.bin.reset();
            self.bins_number += 1;
            self.bin.add(elem);
        }
    }

    fn bins_number(&self) -> usize {
        self.bins_number
    }
}

