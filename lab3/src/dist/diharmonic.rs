use crate::dist::distribution::{Distribution, sanitise_bounds, random_uniform};

const NAME_DIHARMONIC: &str = "diharmonic";

#[derive(Clone)]
pub struct Diharmonic {
    limit: usize,
    hs: Vec<f64>,
}

impl Distribution for Diharmonic {
    fn new(limit: usize) -> Self {
        let limit_sanitised = sanitise_bounds(limit);
        let hs = calculate_generalized_harmonic_cdf(limit, 2.0);
        Diharmonic { limit: limit_sanitised, hs }
    }

    fn gen(&self) -> usize {
        let x: f64 = random_uniform();
        let mut idx: usize = 0;
        while idx < self.limit && self.hs[idx] < x {
            idx += 1;
        }
        idx + 1
    }

    fn change_limit(&mut self, new_limit: usize) {
        self.limit = sanitise_bounds(new_limit);
        self.hs = calculate_generalized_harmonic_cdf(self.limit, 2.0);
    }

    fn name(&self) -> &'static str {
        NAME_DIHARMONIC
    }
}

fn calculate_generalized_harmonic_cdf (n: usize, e: f64) -> Vec<f64> {
    let mut hs: Vec<f64> = vec![1.0];
    for i in 1..n {
        hs.push(hs[i-1] + (1.0 / ((i+1) as f64).powf(e)));
    }
    let hn = hs[n-1];
    for h in hs.iter_mut() {
        *h /= hn;
    }
    hs
}
