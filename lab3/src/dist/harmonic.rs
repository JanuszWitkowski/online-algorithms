use crate::dist::distribution::{Distribution, sanitise_bounds, random_uniform};

const NAME_HARMONIC: &str = "harmonic";

#[derive(Clone)]
pub struct Harmonic {
    limit: usize,
    hs: Vec<f64>,
}

impl Distribution for Harmonic {
    fn new(limit: usize) -> Self {
        let limit_sanitised = sanitise_bounds(limit);
        let hs = calculate_harmonic_cdf(limit);
        Harmonic { limit: limit_sanitised, hs }
    }

    fn get(&self) -> usize {
        let x: f64 = random_uniform();
        let mut idx: usize = 0;
        while idx < self.limit && self.hs[idx] < x {
            idx += 1;
        }
        idx + 1
    }

    fn change_limit(&mut self, new_limit: usize) {
        self.limit = sanitise_bounds(new_limit);
        self.hs = calculate_harmonic_cdf(self.limit);
    }
    
    fn name(&self) -> &'static str {
        NAME_HARMONIC
    }
}

// Here ranges will be used, instead of inverse of CDF.
fn calculate_harmonic_cdf (n: usize) -> Vec<f64> {
    let mut hs: Vec<f64> = vec![1.0];
    for i in 1..n {
        hs.push(hs[i-1] + (1.0 / (i+1) as f64));
    }
    let hn = hs[n-1];
    for h in hs.iter_mut() {
        *h /= hn;
    }
    hs
}
