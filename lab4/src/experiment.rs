use crate::graf::graph::Graph;
use crate::graf::torus::Torus;
use crate::graf::hypercube::Hypercube;

use crate::algs::alg::Alg;
use crate::algs::mtm::MoveToMin;
use crate::algs::flip::Flip;

use crate::dist::distribution::*;
use crate::dist::uniform::*;
use crate::dist::harmonic::*;
use crate::dist::diharmonic::*;

use std::fs::{File, create_dir_all};
use std::io::Write;


const DIR_RESULTS:  &str = "results";
const FILE_EXT:     &str = ".csv";


pub fn run(
        number_of_nodes:    usize, 
        migration_cost:     usize, 
        number_of_requests: usize, 
        number_of_experiments:  usize
) {
    let mut graphs: [&mut dyn Graph; 2] = [
        &mut Torus::new(number_of_nodes),
        &mut Hypercube::new(number_of_nodes)
    ];
    let mut algs: [&mut dyn Alg; 2] = [
        &mut MoveToMin::new(migration_cost),
        &mut Flip::new(migration_cost)
    ];
    let dists: [&mut dyn Distribution; 3] = [
        &mut Uniform::new(number_of_nodes),
        &mut Harmonic::new(number_of_nodes),
        &mut Diharmonic::new(number_of_nodes)
    ];
    let mut sum_of_totals:  f64;
    let (mut total, mut avg);

    println!("Generating random sequences...");
    let mut sequences: Vec<Vec<Vec<usize>>> = Vec::new();
    for i in 0..dists.len() {
        sequences.push(Vec::new());
        for _ in 0..number_of_experiments {
            let mut sequence = Vec::new();
            for _ in 0..number_of_requests {
                sequence.push(dists[i].gen());
            }
            sequences[i].push(sequence);
        }
        println!("Done for {}", dists[i].name());
    }

    create_dir_all(DIR_RESULTS).unwrap();
    
    println!("Beginning experiments...");
    // for dist in &dists {
    //     for i in 0..number_of_experiments {
    //         let mut sequence = Vec::new();
    //         for _ in 0..number_of_requests {
    //             sequence.push(dist.gen());
    //         }
    //         for graph in &mut graphs.iter_mut() {
    //             for alg in &mut algs.iter_mut() {
    //                 for request in &sequence {
    //                     alg.request(*graph, *request);
    //                 }
    //                 // total_cost
    //             }
    //         }
    //         println!("Done for {}, {}/{}", dist.name(), i, number_of_experiments);
    //     }
    // }
    for graph in &mut graphs.iter_mut() {
        for i in 0..dists.len() {
            for alg in &mut algs.iter_mut() {
                sum_of_totals = 0.0;

                let filename = format!("{}/data_total_{}_{}_{}.{}", DIR_RESULTS, graph.name(), alg.name(), dists[i].name(), FILE_EXT);
                let mut output_totals = File::create(filename).unwrap();
                let filename = format!("{}/data_avg_{}_{}_{}.{}", DIR_RESULTS, graph.name(), alg.name(), dists[i].name(), FILE_EXT);
                let mut output_avgs = File::create(filename).unwrap();

                for j in 0..sequences[i].len() {
                    alg.clear();
                    for request in &sequences[i][j] {
                        alg.request(*graph, *request);
                    }

                    total = alg.total_cost();
                    writeln!(output_totals, "{}", total).unwrap();
                    sum_of_totals += total as f64;

                    avg = total as f64 / number_of_requests as f64;
                    writeln!(output_avgs, "{}", avg).unwrap();

                    println!("Done for {} in {} using {}, {}/{}", dists[i].name(), graph.name(), alg.name(), j, number_of_experiments);
                }

                let filename = format!("{}/expv_total_{}_{}_{}.{}", DIR_RESULTS, graph.name(), alg.name(), dists[i].name(), FILE_EXT);
                let mut output_totals = File::create(filename).unwrap();
                let filename = format!("{}/expv_avg_{}_{}_{}.{}", DIR_RESULTS, graph.name(), alg.name(), dists[i].name(), FILE_EXT);
                let mut output_avgs = File::create(filename).unwrap();
                writeln!(output_totals, "{}", sum_of_totals).unwrap();
                writeln!(output_avgs, "{}", sum_of_totals / number_of_requests as f64 / number_of_experiments as f64).unwrap();
            }
        }
    }
    println!("Done with all experiments!");
}



/*
torus:
uniform-mtm;uniform-flip;harmonic-mtm;harmonic-flip;diharmonic-mtm;diharmonic-flip
um;uf;hm;hf;dm;df
 */
