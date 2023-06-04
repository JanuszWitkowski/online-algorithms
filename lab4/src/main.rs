mod dist;
mod graf;
mod algs;
mod experiment;

const NUMBER_OF_VERTICES:   usize = 64;
const MIGRATION_COST_MULT:  usize = 32;
const NUMBER_OF_REQUESTS:   usize = 1024;
const NUMBER_OF_EXPERIMENTS:    usize = 1_000_000;

fn main() {
    let timer = std::time::Instant::now();
    experiment::run(NUMBER_OF_VERTICES, MIGRATION_COST_MULT, NUMBER_OF_REQUESTS, NUMBER_OF_EXPERIMENTS);
    let elapsed = timer.elapsed();
    println!("Time spent on experiments: {:?}", elapsed);
}
