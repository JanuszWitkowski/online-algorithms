mod update_list;
use update_list::{UpdateList, UpdateListType};
mod distribution;
use distribution::{Distribution, DistributionType};

// Types of structures used in experiments
const ULS: [UpdateListType; 4] = [UpdateListType::Classic, UpdateListType::MoveToFront, UpdateListType::Transpose, UpdateListType::Count];
const DSS: [DistributionType; 4] = [DistributionType::Uniform, DistributionType::Geometric, DistributionType::Harmonic, DistributionType::TwoHarmonic];

// Numbers of operations
const NS: [usize; 7] = [100, 500, 1000, 5000, 10000, 50000, 100000];

// Number of iterations
const ITER: usize = 1_000_000;

// Range used in random number generation in {1, ..., 100}
const LOW: u8 = 1;
const HIGH: u8 = 100;


fn experiment_average_access_cost (ul: &mut dyn update_list::UpdateList,
     ds: &dyn distribution::Distribution, n: usize, iter: usize) -> f64 {
    let mut sum: f64 = 0.0;
    for _ in 0..iter {
        ul.clear();
        for _ in 0..n {
            sum += ul.access(ds.get()) as f64;
        }
    }
    return sum / ((n as f64) * (iter as f64));
}

fn main() {
    // let mut ul = update_list::ULClassic::new();
    // let mut ul = update_list::ULMoveToFront::new();
    // let mut ul = update_list::ULTranspose::new();
    let mut ul = update_list::ULCount::new();
    // let ds = distribution::Uniform::new(LOW, HIGH);
    // let ds = distribution::Geometric::new(LOW, HIGH);
    // let ds = distribution::Harmonic::new(LOW, HIGH);
    let ds = distribution::TwoHarmonic::new(LOW, HIGH);

    // for _ in 0..10_000_000 {
    //     ul.access(ds.get());
    // }
    // ul.print();
    
    for list_type in ULS {
        println!("{}", update_list::update_list_name(list_type));
    }
    for dist_type in DSS {
        println!("{}", distribution::distribution_name(dist_type));
    }

    // println!("{}", experiment_average_access_cost(&mut ul, &ds, NS[0], ITER));

    // let ff: f64 = (10.0_f64.powf(13.0) - 5000.0) / 10.0_f64.powf(11.0);
    // println!("f64: {}", ff);
}
