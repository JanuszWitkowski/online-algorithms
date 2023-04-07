use crate::cache::{Cache, LFU};

const NS: [usize; 9] = [20, 30, 40, 50, 60, 70, 80, 90, 100];
const K_DIVS: [usize; 6] = [10, 9, 8, 7, 6, 5];

pub fn print_constants() {
    println!("{:?}", NS);
    println!("{:?}", K_DIVS);
}

pub fn check_lfu() {
    let mut lfu = LFU::new(5, 10);
    for i in 1..=5 {
        lfu.access(i);
    }
    lfu.print();
    // lfu.print_usage();
    for i in 1..=5 {
        lfu.access(i);
    }
    lfu.print();
    // lfu.print_usage();
    lfu.access(6);
    lfu.print();
    // lfu.print_usage();
    for _ in 0..3 {
        lfu.access(6);
    }
    lfu.print();
    // lfu.print_usage();
    lfu.access(2);
    lfu.print();
    // lfu.print_usage();
    lfu.access(7);
    lfu.print();
    // lfu.print_usage();
}
