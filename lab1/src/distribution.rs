// NOTE: There is a library for distributions:
// use rand::distributions::{Distribution, Standard, Uniform};
// It is not used here - implemented them myself, just for fun.
use rand::Rng; // 0.8.5
// const mut rng = rand::thread_rng();

// TRAIT
pub trait Distribution {
    fn new(limit: u8) -> Self where Self: Sized;
    fn get(&self) -> u8;
    fn name(&self) -> &'static str;
    fn ev(&self, iter: usize) -> f64 {
        let mut sum: f64 = 0.0;
        for _ in 0..iter {
            sum += self.get() as f64;
        }
        return sum / (iter as f64);
    }
}

// Names for file naming.
const NAME_UNIFORM: &'static str = "uniform";
const NAME_GEOMETRIC: &'static str = "geometric";
const NAME_HARMONIC: &'static str = "harmonic";
const NAME_DIHARMONIC: &'static str = "diharmonic";


// HELPER FUNCTIONS
fn sanitise_bounds(limit: u8) -> u8 {
    if limit < 1 {
        return 1;
    }
    limit
}


// IMPLEMENTATIONS
pub struct Uniform {
    limit: u8,
}
impl Distribution for Uniform {
    fn new(limit: u8) -> Self {
        let limit_sanitised = sanitise_bounds(limit);
        return Uniform { limit: limit_sanitised }
    }
    fn get(&self) -> u8 {
        return rand::thread_rng().gen_range(1..=self.limit);
    }
    fn name(&self) -> &'static str {
        NAME_UNIFORM
    }
}


pub struct Geometric {
    limit: u8,
    p: f64,
}
impl Distribution for Geometric {
    fn new(limit: u8) -> Self {
        let limit_sanitised = sanitise_bounds(limit);
        return Geometric { limit: limit_sanitised, p: 0.5 }
    }
    fn get(&self) -> u8 {
        let mut x: f64 = 0.0;
        while x <= 0.0 || x >= 1.0 {
            x = rand::thread_rng().gen();
        }
        let v = inv_cdf_geometric(x, self.p);
        if v > self.limit as u32 {
            return self.limit;
        }
        return v as u8;
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


pub struct Harmonic {
    limit: u8,
    hs: Vec<f64>,
}
impl Distribution for Harmonic {
    fn new(limit: u8) -> Self {
        let limit_sanitised = sanitise_bounds(limit);
        let hs = calculate_harmonic_cdf(limit as usize);
        return Harmonic { limit: limit_sanitised, hs: hs };
    }
    fn get(&self) -> u8 {
        let x: f64 = rand::thread_rng().gen();
        let mut idx: usize = 0;
        while idx < self.limit as usize && self.hs[idx] < x {
            idx += 1;
        }
        return idx as u8 + 1;
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
    for i in 0..n {
        hs[i] /= hn;
    }
    return hs;
}


pub struct Diharmonic {
    limit: u8,
    hs: Vec<f64>,
}
impl Distribution for Diharmonic {
    fn new(limit: u8) -> Self {
        let limit_sanitised = sanitise_bounds(limit);
        let hs = calculate_generalized_harmonic_cdf(limit as usize, 2.0);
        return Diharmonic { limit: limit_sanitised, hs: hs };
    }
    fn get(&self) -> u8 {
        let x: f64 = rand::thread_rng().gen();
        let mut idx: usize = 0;
        while idx < self.limit as usize && self.hs[idx] < x {
            idx += 1;
        }
        return idx as u8 + 1;
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
    for i in 0..n {
        hs[i] /= hn;
    }
    return hs;
}

