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


#[cfg(test)]
mod tests {
    use crate::fits::fit::{Fit};
    use crate::fits::best::{BestFit};
    use crate::fits::best::{NAME_BEST};

    #[test]
    fn test_best_new() {
        let bf = BestFit::new();
        assert_eq!(bf.name(), NAME_BEST);
    }

    #[test]
    fn test_best_add_to_three_fulls() {
        let mut bf = BestFit::new();
        // |(0.4+0.4+0.2), (0.4+0.6), (0.4+0.2+0.2+0.2)| == 3
        let seq = [0.4, 0.4, 0.4, 0.6, 0.4, 0.2, 0.2, 0.2, 0.2];
        for elem in seq {
            bf.add(elem);
        }
        assert_eq!(bf.bins_number(), 3);
        assert_eq!(bf.bins[0].show(), 1.0);
        assert_eq!(bf.bins[1].show(), 1.0);
        assert_eq!(bf.bins[2].show(), 1.0);
    }

    #[test]
    fn test_best_add_not_optimal() {
        let mut bf = BestFit::new();
        // |(0.8), (0.6), (0.6)| == 3
        // Optimal would be |(1.0), (1.0)| == 2
        let seq = [0.4, 0.4, 0.6, 0.6];
        for elem in seq {
            bf.add(elem);
        }
        assert_eq!(bf.bins_number(), 3);
        assert_eq!(bf.bins[0].show(), 0.6);
        assert_eq!(bf.bins[1].show(), 0.6);
        assert_eq!(bf.bins[2].show(), 0.8);
    }

    // #[test]
    // fn test_best_sorting() {
    //     let mut bf = BestFit::new();
    //     let seq = [.6, .6, ];
    //     for elem in seq {
    //         bf.add(elem);
    //     }
    //     assert_eq!(bf.bins_number(), 3);
    // }
}
