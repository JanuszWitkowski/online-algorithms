use crate::dist::distribution::*;
use crate::fits::fit::*;
use std::fs::{File};
use std::io::Write;

fn single_bin_pack(fit: &mut dyn Fit, sequence: &Vec<f64>) -> usize {
    fit.reset();
    for elem in sequence {
        fit.add(*elem);
    }
    fit.bins_number()
}

pub fn run_bit_packings(fit: &mut dyn Fit, dist: &dyn Distribution, seq_limit: usize, iter: usize) {
    let mut sequence;
    let mut cost;
    // println!("{}_{}", fit.name(), dist.name());
    let filename = format!("data_{}_{}.jl", fit.name(), dist.name());
    let mut output = File::create(filename).unwrap();
    println!("Writing results for {}_{}...", fit.name(), dist.name());
    writeln!(output, "xs = [").unwrap();
    for _ in 0..iter {
        sequence = dist.random_sequence(seq_limit);
        cost = single_bin_pack(fit, &sequence);
        // println!("\t({:?}, {})", sequence, cost);
        writeln!(output, "\t({:?}, {})", sequence, cost).unwrap();
    }
    writeln!(output, "]").unwrap();
}
