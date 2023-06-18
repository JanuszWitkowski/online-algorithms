mod graph;
mod dist;
mod experiment;
// use crate::graph::network::*;
// use crate::dist::distribution::*;
// use crate::dist::uniform::*;

fn main() {
    let n = 64;
    let number_of_requests = 65536;
    let number_of_experiments = 100;
    let ds = [16, 32, 64, 128, 256];
    let ps = [0.01, 0.02, 0.05, 0.1, 0.2, 0.5];

    // let mut uniform = Uniform::new(n);
    // let mut network = Network::new(n, ds[0]);

    experiment::run(n, number_of_requests, &ds, &ps, number_of_experiments);

    println!("Hello, world!");
}

