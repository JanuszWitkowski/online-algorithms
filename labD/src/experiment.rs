use crate::graph::network::*;
use crate::dist::distribution::*;
use crate::dist::uniform::*;
use rand::Rng;
use std::fs::{File, create_dir_all};
use std::io::Write;


// fn get_request_type(p: f64) {
//     //
// }

const DIR_RESULTS:  &str = "results";
const FILE_EXT:     &str = "csv";


pub fn run(n: usize, r: usize, ds: &[usize], ps: &[f64], x: usize) {
    let mut uniform = Uniform::new(n);
    let mut network = Network::new(n, ds[0]);
    let mut rng = rand::thread_rng();

    let mut request_id: usize;
    let mut request_type: RequestType;

    let mut avg_cost: f64;
    let mut max_copies: usize;
    let mut total_avg_cost: f64 = 0.0;
    let mut total_max_copies: usize = 0;
    let mut total_results = Vec::new();

    create_dir_all(DIR_RESULTS).unwrap();

    for d in ds {
        network.set_d_value(*d);
        for p in ps {
            // Open new file
            let filename = format!("{}/data_d{}_p{}_.{}", DIR_RESULTS, d, p, FILE_EXT);
            let mut output = File::create(filename).unwrap();

            println!("Running for d={}, p={} ...", d, p);
            for xth in 1..=x {
                // println!("{}, ", xth);
                network.clear();
                for ri in 0..r {
                    request_id = uniform.gen();
                    request_type = match rng.gen::<f64>() < *p {
                        true => RequestType::WRITE,
                        false => RequestType::READ
                    };
                    network.request(request_type, request_id);
                    // print!("({},{}) ", xi, ri);
                    // print!("|");
                }
                // print!("{} ", xth);
                println!("Done d={}, p={}, {}/{} times.", d, p, xth, x);
                (avg_cost, max_copies, _) = network.results();
                // Write resutls
                writeln!(output, "{};{}", avg_cost, max_copies).unwrap();
                total_avg_cost += avg_cost;
                total_max_copies += max_copies;
            }
            println!();
            total_results.push((d, p, total_avg_cost / (x as f64), (total_max_copies as f64) / (x as f64)));
            total_avg_cost = 0.0;
            total_max_copies = 0;
        }
    }

    let mut total_output = File::create(format!("{}/total.{}", DIR_RESULTS, FILE_EXT)).unwrap();
    for (d, p, c, s) in total_results {
        writeln!(total_output, "{};{};{};{}", d, p, c, s).unwrap();
    }

    // let mut costs_output = File::create(format!("{}/total_costs.{}", DIR_RESULTS, FILE_EXT)).unwrap();
    // for (d, p, c, s) in total_results {
    //     writeln!(costs_output, "{};{};{};{}", d, p, c, s).unwrap();
    // }
    // let mut copies_output = File::create(format!("{}/total_copies.{}", DIR_RESULTS, FILE_EXT)).unwrap();
    // for (d, p, c, s) in total_results {
    //     // writeln!(copies_output, "{};{};{};{}", d, p, c, s).unwrap();
    //     write!(copies_output, "{};", s).unwrap();
    // }

    println!("Done!");
}

