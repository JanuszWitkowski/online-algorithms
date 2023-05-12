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
        if self.bins.is_empty() {
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
            let mut i = 0;
            while i < self.bins.len() && self.bins[i].show() < content {
                i += 1;
            }
            self.bins.insert(i, bin);
            }
    }

    fn reset(&mut self) {
        self.bins.clear();
    }

    fn bins_number(&self) -> usize {
        self.bins.len()
    }
}


#[cfg(test)]
mod tests {
    use crate::fits::fit::{Fit};
    use crate::fits::worst::{WorstFit};
    use crate::fits::worst::{NAME_WORST};

    #[test]
    fn test_worst_new() {
        let wf = WorstFit::new();
        assert_eq!(wf.name(), NAME_WORST);
    }

    #[test]
    fn test_worst_add_to_three_fulls() {
        let mut wf = WorstFit::new();
        // |(0.4+0.4+0.2), (0.4+0.6), (0.4+0.2+0.2+0.2)| == 3
        let seq = [0.4, 0.4, 0.4, 0.6, 0.4, 0.2, 0.2, 0.2, 0.2];
        for elem in seq {
            wf.add(elem);
        }
        assert_eq!(wf.bins_number(), 3);
        assert_eq!(wf.bins[0].show(), 1.0);
        assert_eq!(wf.bins[1].show(), 1.0);
        assert_eq!(wf.bins[2].show(), 1.0);
    }

    #[test]
    fn test_worst_add_not_optimal() {
        let mut wf = WorstFit::new();
        // |(0.8), (0.6), (0.6)| == 3
        // Optimal would be |(1.0), (1.0)| == 2
        let seq = [0.4, 0.4, 0.6, 0.6];
        for elem in seq {
            wf.add(elem);
        }
        assert_eq!(wf.bins_number(), 3);
        assert_eq!(wf.bins[0].show(), 0.6);
        assert_eq!(wf.bins[1].show(), 0.6);
        assert_eq!(wf.bins[2].show(), 0.8);
    }
}
