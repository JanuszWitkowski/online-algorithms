use distribution::Distribution;

const NAME_UNIFORM: &'static str = "uniform";

#[derive(Clone)]
pub struct Uniform {
    limit: usize,
}
impl Distribution for Uniform {
    fn new(limit: usize) -> Self {
        let limit_sanitised = sanitise_bounds(limit);
        return Uniform { limit: limit_sanitised }
    }
    fn get(&self) -> usize {
        return rand::thread_rng().gen_range(1..=self.limit);
    }
    fn change_limit(&mut self, new_limit: usize) {
        self.limit = sanitise_bounds(new_limit);
    }
    fn name(&self) -> &'static str {
        NAME_UNIFORM
    }
}
