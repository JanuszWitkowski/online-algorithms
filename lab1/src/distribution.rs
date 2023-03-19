// NOTE: There is a library for distributions:
// use rand::distributions::{Distribution, Standard, Uniform};
// It is not used here to implement it myself, just for fun.
use rand::Rng; // 0.8.5
// const mut rng = rand::thread_rng();

// TRAIT
pub trait Distribution {
    fn new(low: u8, high: u8) -> Self where Self: Sized;
    fn get(&self) -> u8;
    fn name(&self) -> &'static str;
}


const NAME_UNIFORM: &'static str = "uniform";
const NAME_GEOMETRIC: &'static str = "geometric";
const NAME_HARMONIC: &'static str = "harmonic";
const NAME_TWOHARMONIC: &'static str = "twoharmonic";


// HELPER FUNCTIONS
fn sanitise_bounds(low: u8, high: u8) -> (u8, u8) {
    let low_sanitised: u8;
    let high_sanitised: u8;
    if low > high {
        low_sanitised = high;
        high_sanitised = low;
    } else {
        low_sanitised = low;
        high_sanitised = high;
    }
    return (low_sanitised, high_sanitised);
}


// IMPLEMENTATIONS
pub struct Uniform {
    low: u8,
    high: u8,
}
impl Distribution for Uniform {
    fn new(low: u8, high: u8) -> Self {
        let (low_sanitised, high_sanitised) = sanitise_bounds(low, high);
        return Uniform { low: low_sanitised, high: high_sanitised }
    }
    fn get(&self) -> u8 {
        return rand::thread_rng().gen_range(self.low..=self.high);
    }
    fn name(&self) -> &'static str {
        NAME_UNIFORM
    }
}


pub struct Geometric {
    low: u8,
    high: u8,
    p: f64,
}
impl Distribution for Geometric {
    fn new(low: u8, high: u8) -> Self {
        let (low_sanitised, high_sanitised) = sanitise_bounds(low, high);
        return Geometric { low: low_sanitised, high: high_sanitised, p: 0.5 }
    }
    fn get(&self) -> u8 {
        let mut x: f64 = 0.0;
        while x <= 0.0 || x >= 1.0 {
            x = rand::thread_rng().gen();
        }
        let v = inv_cdf_geometric(x, self.p) + (self.low as u32) - 1;
        if v > self.high as u32 {
            return self.high;
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
    low: u8,
    high: u8,
    harm: f64,
}
impl Distribution for Harmonic {
    fn new(low: u8, high: u8) -> Self {
        let h = calculate_harmonic_number(high as usize, 0.0);
        return Harmonic { low: low, high: high, harm: h };
    }
    fn get(&self) -> u8 {   //TODO
        return 0;
    }
    fn name(&self) -> &'static str {
        NAME_HARMONIC
    }
}

fn calculate_harmonic_number (n: usize, acc: f64) -> f64 {
    if n == 0 {
        return acc;
    }
    calculate_harmonic_number(n - 1, acc + (1.0 / (n as f64)))
}


pub struct TwoHarmonic {
    low: u8,
    high: u8,
    harm: f64,
}
impl Distribution for TwoHarmonic {
    fn new(low: u8, high: u8) -> Self {
        let h = calculate_generalized_harmonic_number(high as usize, 2.0, 0.0);
        return TwoHarmonic { low: low, high: high, harm: h };
    }
    fn get(&self) -> u8 {   //TODO
        return 0;
    }
    fn name(&self) -> &'static str {
        NAME_TWOHARMONIC
    }
}

fn calculate_generalized_harmonic_number (n: usize, m: f64, acc: f64) -> f64 {
    if n == 0 {
        return acc;
    }
    calculate_generalized_harmonic_number(n - 1, m, acc + (1.0 / (n as f64).powf(m)))
}
