// NOTE: There is a library for distributions:
// use rand::distributions::{Distribution, Standard, Uniform};
// It is not used here - implemented them myself, just for fun.
use rand::Rng; // 0.8.5
// const mut rng = rand::thread_rng();

// TRAIT
pub trait Distribution {
    fn new(limit: usize) -> Self where Self: Sized;
    fn get(&self) -> usize;
    fn change_limit(&mut self, new_limit: usize);
    fn name(&self) -> &'static str;
    fn ev(&self, iter: usize) -> f64 {
        let mut sum: f64 = 0.0;
        for _ in 0..iter {
            sum += self.get() as f64;
        }
        sum / (iter as f64)
    }
}

// HELPER FUNCTIONS
pub fn sanitise_bounds(limit: usize) -> usize {
    if limit < 1 {
        return 1;
    }
    limit
}

pub fn random_uniform() -> f64 {
    rand::thread_rng().gen()
}

pub fn random_uniform_in_range(limit: usize) -> usize {
    rand::thread_rng().gen_range(1..=limit)
}
