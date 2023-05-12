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


#[cfg(test)]
mod tests {
    use crate::fits::fit::{Fit};
    use crate::fits::next::{NextFit};
    use crate::fits::next::{NAME_NEXT};

    #[test]
    fn test_next_new() {
        let nf = NextFit::new();
        assert_eq!(nf.name(), NAME_NEXT);
    }

    #[test]
    fn test_next_add_to_full() {
        let mut nf = NextFit::new();
        let seq = [0.1, 0.2, 0.3, 0.4]; // sum == 1.0
        for elem in seq {
            nf.add(elem);
        }
        assert_eq!(nf.bins_number(), 1);
    }

    #[test]
    fn test_next_add_not_fitting() {
        let mut nf = NextFit::new();
        let seq = [0.3, 0.3, 0.5];  // |(0.6, 0.5)| == 2
        for elem in seq {
            nf.add(elem);
        }
        assert_eq!(nf.bins_number(), 2);
    }

    #[test]
    fn test_next_add_not_optimal() {
        let mut nf = NextFit::new();
        let seq = [0.5, 0.6, 0.5, 0.4]; // |(0.5, 0.6, 0.9)| == 3; optimal is |(1.0, 1.0)| == 2
        for elem in seq {
            nf.add(elem);
        }
        assert_eq!(nf.bins_number(), 3);
    }

    #[test]
    fn test_next_bins_number() {
        let mut nf = NextFit::new();
        let seq = [0.6; 1_000];
        for elem in seq {
            nf.add(elem);
        }
        assert_eq!(nf.bins_number(), 1_000);
    }

    #[test]
    fn test_next_bins_number_not_optimal() {
        let mut nf = NextFit::new();
        let seq1 = [0.6; 2_000];
        let seq2 = [0.4; 2_000];
        // 2_000 x (0.6) & 1_000 x (0.4+0.4)
        // Optimal would be 2_000 x (0.6 + 0.4)
        for elem in seq1 {
            nf.add(elem);
        }
        for elem in seq2 {
            nf.add(elem);
        }
        assert_eq!(nf.bins_number(), 3_000);
    }
}
