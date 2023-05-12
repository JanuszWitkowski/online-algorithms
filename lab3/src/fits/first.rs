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
    use crate::fits::first::{FirstFit};
    use crate::fits::first::{NAME_FIRST};

    #[test]
    fn test_first_new() {
        let ff = FirstFit::new();
        assert_eq!(ff.name(), NAME_FIRST);
    }

    #[test]
    fn test_first_add_to_two_fulls() {
        let mut ff = FirstFit::new();
        let seq = [0.3, 0.4, 0.5, 0.3, 0.2, 0.3];   // |(0.3+0.4+0.3), (0.5+0.2+0.3)| == 2
        for elem in seq {
            ff.add(elem);
        }
        assert_eq!(ff.bins_number(), 2);
        assert_eq!(ff.bins[0].show(), 1.0);
        assert_eq!(ff.bins[1].show(), 1.0);
    }

    #[test]
    fn test_first_add_not_optimal() {
        let mut ff = FirstFit::new();
        let seq = [0.3, 0.9, 0.1, 0.7]; // |(0.3+0.1), (0.9), (0.7)| == 3
        // Optimal would be |(0.3+0.7), (0.9+0.1)| == 2
        for elem in seq {
            ff.add(elem);
        }
        assert_eq!(ff.bins_number(), 3);
        assert_eq!(ff.bins[0].show(), 0.4);
        assert_eq!(ff.bins[1].show(), 0.9);
        assert_eq!(ff.bins[2].show(), 0.7);
    }
}
