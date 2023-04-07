mod cache;
mod distribution;
mod experiment;

const NS: [usize; 9] = [20, 30, 40, 50, 60, 70, 80, 90, 100];

fn main() {
    let ns_of_requests: Vec<usize> = (1..=10).map(|x| 10_000*x).collect::<Vec<usize>>();
    let n_of_experiments = 10_000;
    // println!("{:?}", ns_of_requests);
    println!("Hello, world!");
    // experiment::check_lfu();
    // experiment::print_constants();
    // println!("{:?}", experiment::get_ks(100, 10, 5));
    // experiment::check_all_caches_and_distributions(10_000, 10_000);
    // experiment::check_all_caches_and_distributions();
    for r in ns_of_requests {
        experiment::check_all_caches_and_distributions(&NS, r, n_of_experiments);
    }
}
