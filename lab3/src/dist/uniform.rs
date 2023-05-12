use crate::dist::distribution::{Distribution, sanitise_bounds, random_uniform_in_range};

const NAME_UNIFORM: &str = "uniform";

#[derive(Clone)]
pub struct Uniform {
    limit: usize,
}

impl Distribution for Uniform {
    fn new(limit: usize) -> Self {
        let limit_sanitised = sanitise_bounds(limit);
        Uniform { limit: limit_sanitised }
    }

    fn gen(&self) -> usize {
        random_uniform_in_range(self.limit)
    }

    fn change_limit(&mut self, new_limit: usize) {
        self.limit = sanitise_bounds(new_limit);
    }

    fn name(&self) -> &'static str {
        NAME_UNIFORM
    }
}
