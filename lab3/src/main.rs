mod dist;
mod fits;
mod experiment;

use crate::dist::distribution::*;
use crate::dist::uniform::*;

// use crate::fits::fit::*;
// use crate::fits::next::*;

// use crate::experiment;

fn main() {
    println!("Hello, world!");
    let u = Uniform::new(10);
    println!("{}", u.name());
    experiment::test_import();
}
