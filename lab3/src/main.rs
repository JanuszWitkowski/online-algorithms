mod dist;
mod fits;
mod experiment;

use crate::dist::distribution::*;
use crate::dist::uniform::*;
use crate::dist::geometric::*;
use crate::dist::harmonic::*;
use crate::dist::diharmonic::*;

use crate::fits::fit::*;
use crate::fits::next::*;
use crate::fits::first::*;
use crate::fits::best::*;
use crate::fits::worst::*;

// use crate::experiment;

const SEQUENCE_LIMIT    : usize = 100;
const DIST_LIMIT        : usize = 10;
const N_OF_EXPERIMENTS  : usize = 1_000;

fn main() {
    println!("Hello, world!");
    let u = Uniform::new(DIST_LIMIT);
    let g = Geometric::new(DIST_LIMIT);
    let h = Harmonic::new(DIST_LIMIT);
    let d = Diharmonic::new(DIST_LIMIT);
    println!("{} {}", u.name(), u.ev(1_000_000));
    println!("{} {}", g.name(), g.ev(1_000_000));
    println!("{} {}", h.name(), h.ev(1_000_000));
    println!("{} {}", d.name(), d.ev(1_000_000));

    let mut nf = NextFit::new();
    let mut ff = FirstFit::new();
    let mut bf = BestFit::new();
    println!("{}", nf.name());
    println!("{}", ff.name());
    println!("{}", bf.name());

    experiment::test_import();

    let seq = u.random_sequence(SEQUENCE_LIMIT);
    println!("{:?}\n{}", seq, seq.len());
    // let seq = g.random_sequence(SEQUENCE_LIMIT);
    // println!("{:?}\n{}", seq, seq.len());
    // let seq = h.random_sequence(SEQUENCE_LIMIT);
    // println!("{:?}\n{}", seq, seq.len());
    // let seq = d.random_sequence(SEQUENCE_LIMIT);
    // println!("{:?}\n{}", seq, seq.len());

    experiment::run_bit_packings(&mut nf, &g, SEQUENCE_LIMIT, N_OF_EXPERIMENTS);
}
