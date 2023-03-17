// NOTE: There is a library for distributions:
// use rand::distributions::{Distribution, Standard, Uniform};
// It is not used here to implement it myself, just for fun.
use rand::Rng; // 0.8.5
// const mut rng = rand::thread_rng();

pub trait Distribution {
    fn new(low: u8, high: u8) -> Self where Self: Sized;
    fn get(&self) -> u8;
}

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
}

impl Distribution for Geometric {
    fn new(low: u8, high: u8) -> Self {
        let (low_sanitised, high_sanitised) = sanitise_bounds(low, high);
        return Geometric { low: low_sanitised, high: high_sanitised }
    }
    fn get(&self) -> u8 {
        let mut x: f64 = 0.0;
        while x <= 0.0 || x >= 1.0 {
            x = rand::thread_rng().gen();
        }
        let v = inv_cdf_geometric(x, 0.5) + (self.low as u32) - 1;
        if v > self.high as u32 {
            return self.high;
        }
        return v as u8;
    }
}

// Takes x and p from range (0, 1)
fn inv_cdf_geometric (x: f64, p: f64) -> u32 {
    (x.ln() / (1.0 - p).ln()).ceil() as u32
}
