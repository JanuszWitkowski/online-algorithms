use crate::cache::{Cache, FIFO, FWF, LRU, LFU, RAND, RMA};
use crate::distribution::{Distribution, Uniform, Geometric, Harmonic, Diharmonic};
use std::fs::{File, create_dir_all};
use std::io::Write;

const K_DIV_UPPER: usize = 10;
const K_DIV_LOWER: usize = 5;


pub fn get_ks(n: usize, k_div_upper: usize, k_div_lower: usize) -> Vec<usize> {
    let mut ks: Vec<usize> = Vec::new();
    let k_left = n / k_div_upper;
    let k_right = n / k_div_lower;
    for k in k_left..=k_right {
        ks.push(k);
    }
    ks
}


pub fn average_request_cost (cache: &mut dyn Cache, distribution: &dyn Distribution, 
        n_of_requests: usize, n_of_experiments: usize) -> f64 {
    let mut sum: f64 = 0.0;
    for _ in 0..n_of_experiments {
        cache.clear();
        for _ in 0..n_of_requests {
            sum += cache.access(distribution.get()) as f64;
        }
    }
    return sum / ((n_of_requests as f64) * (n_of_experiments as f64));
}

pub fn check_all_caches_and_distributions (ns: &[usize], n_of_requests: usize, n_of_experiments: usize) {
    // Write set of ns to file, for future data analysis.
    match create_dir_all("results") {
        Err(e) => panic!("Could not create directory! {:?}", e),
        _ => (),
    };
    let mut filename = format!("results/ns.csv");
    let output = File::create(filename);
    let mut output = match output {
        Ok(file) => file,
        Err(error) => panic!("Problem creating ns file : {:?}", error),
    };
    for n in ns {
        write!(output, "{}\n", *n).unwrap();
    }

    let mut avg_cost: f64;
    for n in ns {
        let ds_uniform = Uniform::new(*n);
        let ds_geometric = Geometric::new(*n);
        let ds_harmonic = Harmonic::new(*n);
        let ds_diharmonic = Diharmonic::new(*n);
        let dists: [&dyn Distribution; 4] = [&ds_uniform, &ds_geometric, &ds_harmonic, &ds_diharmonic];
        let ks = get_ks(*n, K_DIV_UPPER, K_DIV_LOWER);

        let mut c_fifo = FIFO::new(10);
        let mut c_fwf = FWF::new(10);
        let mut c_lru = LRU::new(10);
        let mut c_lfu = LFU::new(10, *n);
        let mut c_rand = RAND::new(10);
        let mut c_rma = RMA::new(10);
        let caches: [&mut dyn Cache; 6] = [
            &mut c_fifo,
            &mut c_fwf,
            &mut c_lru,
            &mut c_lfu,
            &mut c_rand,
            &mut c_rma
        ];

        for cache in caches {
            for dist in dists {
                filename = format!("results/data_r{}_n{}_{}_{}.csv", n_of_requests, *n, cache.name(), dist.name());
                println!("Writing {}...", filename);
                let output = File::create(filename);
                let mut output = match output {
                    Ok(file) => file,
                    Err(error) => panic!("Problem creating data file : {:?}", error),
                };
                for k in &ks {
                    cache.change_len(*k);
                    avg_cost = average_request_cost(cache, dist, n_of_requests, n_of_experiments);
                    write!(output, "{};{}\n", k, avg_cost).unwrap();
                    print!(" k{}", k);
                }
                println!("Done.\n");
            }
        }
    }
    println!("Done with all experiments! (for {} requests)", n_of_requests);
}





// pub fn _check_lfu() {
//     let mut lfu = LFU::new(5, 10);
//     for i in 1..=5 {
//         lfu.access(i);
//     }
//     lfu.print();
//     // lfu.print_usage();
//     for i in 1..=5 {
//         lfu.access(i);
//     }
//     lfu.print();
//     // lfu.print_usage();
//     lfu.access(6);
//     lfu.print();
//     // lfu.print_usage();
//     for _ in 0..3 {
//         lfu.access(6);
//     }
//     lfu.print();
//     // lfu.print_usage();
//     lfu.access(2);
//     lfu.print();
//     // lfu.print_usage();
//     lfu.access(7);
//     lfu.print();
//     lfu.print_usage();
//     lfu.clear();
//     lfu.print();
//     lfu.print_usage();
// }
