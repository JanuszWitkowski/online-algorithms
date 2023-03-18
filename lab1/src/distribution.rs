// NOTE: There is a library for distributions:
// use rand::distributions::{Distribution, Standard, Uniform};
// It is not used here to implement it myself, just for fun.
use rand::Rng; // 0.8.5
// const mut rng = rand::thread_rng();

// TRAIT
pub trait Distribution {
    fn new(low: u8, high: u8) -> Self where Self: Sized;
    fn get(&self) -> u8;
}


// ENUM
pub enum DistributionType {
    Uniform,
    Geometric,
    Harmonic,
    TwoHarmonic,
}

// pub fn distribution_constructor(dist_type: DistributionType, low: u8, high: u8) -> Box<&'static dyn Distribution> {
//     match dist_type {
//         DistributionType::Uniform => Box::new(&Uniform::new(low, high)),
//         DistributionType::Geometric => Box::new(&Geometric::new(low, high)),
//         DistributionType::Harmonic => Box::new(&Harmonic::new(low, high)),
//         DistributionType::TwoHarmonic => Box::new(&TwoHarmonic::new(low, high)),
//     }
// }

// pub fn distribution_name(dist_type: DistributionType) -> &'static str {
//     match dist_type {
//         DistributionType::Uniform => "uniform",
//         DistributionType::Geometric => "geometric",
//         DistributionType::Harmonic => "harmonic",
//         DistributionType::TwoHarmonic => "twoharmonic",
//     }
// }


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
}

fn calculate_generalized_harmonic_number (n: usize, m: f64, acc: f64) -> f64 {
    if n == 0 {
        return acc;
    }
    calculate_generalized_harmonic_number(n - 1, m, acc + (1.0 / (n as f64).powf(m)))
}
