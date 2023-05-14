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

pub fn run_bit_packings(
                    fit: &mut dyn Fit, 
                    dist: &dyn Distribution, 
                    seq_limit: usize, 
                    iter_experiments: usize) {
    let mut sequence;
    let (mut cost, mut opt);
    let (mut cost_sum, mut opt_sum);
    let (mut ratio, competitive);
    // println!("{}_{}", fit.name(), dist.name());
    let filename = format!("data_{}_{}.csv", fit.name(), dist.name());
    let mut output = File::create(filename).unwrap();
    println!("Writing results for {}_{}...", fit.name(), dist.name());
    writeln!(output, "online;opt;konk").unwrap();
    cost_sum = 0;
    opt_sum = 0;
    for _ in 0..iter_experiments {
        sequence = dist.random_sequence(seq_limit);
        cost = single_bin_pack(fit, &sequence);
        opt = estimated_optimum(&sequence);
        ratio = (cost as f64) / (opt as f64);
        cost_sum += cost;
        opt_sum += opt;
        // println!("\t({:?}, {})", sequence, cost);
        writeln!(output, "{};{};{}", cost, opt, ratio).unwrap();
    }
    competitive = (cost_sum as f64) / (opt_sum as f64);
    // writeln!(output, "]").unwrap();
    let filename = format!("konk_{}_{}.txt", fit.name(), dist.name());
    let mut output = File::create(filename).unwrap();
    writeln!(output, "{}", competitive).unwrap();
}

pub fn multiple_bit_packings(
                    fit: &mut dyn Fit, 
                    dist: &dyn Distribution, 
                    seq_limit: usize, 
                    iter_experiments: usize, 
                    iter_fit: usize) {
    let mut sequence;
    let (mut cost, mut opt);
    let (mut cost_sum, mut opt_sum);
    let (mut ratio, competitive);
    // println!("{}_{}", fit.name(), dist.name());
    let filename = format!("data_{}_{}.csv", fit.name(), dist.name());
    let mut output = File::create(filename).unwrap();
    println!("Writing results for {}_{}...", fit.name(), dist.name());
    writeln!(output, "online;opt;konk").unwrap();
    cost_sum = 0;
    opt_sum = 0;
    for _ in 0..iter_experiments {
        sequence = dist.random_sequence(seq_limit);
        cost = 0;
        for _ in 0..iter_fit {
            cost += single_bin_pack(fit, &sequence);
        }
        cost /= iter_fit;
        opt = estimated_optimum(&sequence);
        ratio = (cost as f64) / (opt as f64);
        cost_sum += cost;
        opt_sum += opt;
        // println!("\t({:?}, {})", sequence, cost);
        writeln!(output, "{};{};{}", cost, opt, ratio).unwrap();
    }
    competitive = (cost_sum as f64) / (opt_sum as f64);
    let filename = format!("konk_{}_{}.txt", fit.name(), dist.name());
    let mut output = File::create(filename).unwrap();
    writeln!(output, "{}", competitive).unwrap();
}

fn estimated_optimum(seq: &[f64]) -> usize {
    seq.iter().sum::<f64>().ceil() as usize
}
