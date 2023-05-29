use crate::dist::distribution::{Distribution, sanitise_bounds, random_uniform};

const NAME_GEOMETRIC: &str = "Geometric";

#[derive(Clone)]
pub struct Geometric {
    limit: usize,
    p: f64,
}

impl Distribution for Geometric {
    fn new(limit: usize) -> Self {
        let limit_sanitised = sanitise_bounds(limit);
        Geometric { limit: limit_sanitised, p: 0.5 }
    }

    fn gen(&self) -> usize {
        let mut x: f64 = 0.0;
        while x <= 0.0 || x >= 1.0 {
            x = random_uniform();
        }
        let v = inv_cdf_geometric(x, self.p);
        if v > self.limit as u32 {
            return self.limit;
        }
        v as usize
    }

    fn change_limit(&mut self, new_limit: usize) {
        self.limit = sanitise_bounds(new_limit);
    }

    fn name(&self) -> &'static str {
        NAME_GEOMETRIC
    }
}

// The inverse of cumulative distribution function for Geometric distribution.
// Randomize x from range (0, 1), then apply this function to get a random value from this distribution.
// Takes x and p from range (0, 1)
fn inv_cdf_geometric (x: f64, p: f64) -> u32 {
    (x.ln() / (1.0 - p).ln()).ceil() as u32
}

