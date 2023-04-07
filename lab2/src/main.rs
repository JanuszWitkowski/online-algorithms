mod cache;
mod distribution;
mod experiment;
use crate::cache::Cache;

fn main() {
    println!("Hello, world!");
    // experiment::check_lfu();
    experiment::print_constants();
}
