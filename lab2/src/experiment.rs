use crate::cache::{Cache, LFU};

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
