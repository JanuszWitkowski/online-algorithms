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

const SEQUENCE_LIMIT    : usize = 100;
const DIST_LIMIT        : usize = 10;
// const N_OF_EXPERIMENTS  : usize = 1_000_000;
const N_OF_EXPERIMENTS  : usize = 10;

fn main() {
    let fits: [&mut dyn Fit; 4] = [
        &mut NextFit::new(),
        &mut FirstFit::new(),
        &mut BestFit::new(),
        &mut WorstFit::new()
    ];
    let distributions: [&dyn Distribution; 4] = [
        &Uniform::new(DIST_LIMIT),
        &Geometric::new(DIST_LIMIT),
        &Harmonic::new(DIST_LIMIT),
        &Diharmonic::new(DIST_LIMIT)
    ];
    for fit in fits {
        for dist in distributions {
            experiment::run_bit_packings(fit, dist, SEQUENCE_LIMIT, N_OF_EXPERIMENTS);
        }
    }
    println!("Done!");
}
