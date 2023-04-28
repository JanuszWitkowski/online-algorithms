mod dist;
use crate::dist::distribution::*;
use crate::dist::uniform::*;

fn main() {
    println!("Hello, world!");
    let u = Uniform::new(10);
    println!("{}", u.name());
}
