// NOTE: There is a library for distributions:
// use rand::distributions::{Distribution, Standard, Uniform};
// It is not used here - implemented them myself, just for fun.
use rand::Rng; // 0.8.5
// static mut rng: rand::prelude::ThreadRng = rand::thread_rng();

// TRAIT
pub trait Distribution {
    fn new(limit: usize) -> Self where Self: Sized;
    fn gen(&self) -> usize;
    fn change_limit(&mut self, new_limit: usize);
    fn name(&self) -> &'static str;

    fn ev(&self, iter: usize) -> f64 {
        let mut sum: f64 = 0.0;
        for _ in 0..iter {
            sum += self.gen() as f64;
        }
        sum / (iter as f64)
    }

    fn random_sequence(&self, sequence_limit: usize) -> Vec<f64> {
        let mut sequence = Vec::new();
        let mut elem: f64;
        let mut k: usize;
        while sequence.len() < sequence_limit {
            elem = random_uniform();
            k = self.gen();
            for _ in 0..k {
                sequence.push(elem);
            }
        }
        // sequence.slice_from(sequence_limit).to_vec()
        sequence[..=sequence_limit].to_vec()
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
