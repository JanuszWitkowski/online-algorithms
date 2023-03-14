use rand::Rng; // 0.8.5

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
