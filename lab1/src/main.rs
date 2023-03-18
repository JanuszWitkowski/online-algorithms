mod update_list;
use update_list::{UpdateList, UpdateListType};
mod distribution;
use distribution::{Distribution, DistributionType};
use std::fs::File;
use std::io::Write;

// use crate::distribution::distribution_name;
// use crate::update_list::update_list_name;

// Types of structures used in experiments
const ULS: [UpdateListType; 4] = [UpdateListType::Classic, UpdateListType::MoveToFront, UpdateListType::Transpose, UpdateListType::Count];
const DSS: [DistributionType; 4] = [DistributionType::Uniform, DistributionType::Geometric, DistributionType::Harmonic, DistributionType::TwoHarmonic];

// Numbers of operations
const NS: [usize; 7] = [100, 500, 1000, 5000, 10000, 50000, 100000];

// Number of iterations
// const ITER: usize = 1_000_000;
const ITER: usize = 1_000;

// Range used in random number generation in {1, ..., 100}
const LOW: u8 = 1;
const HIGH: u8 = 100;


// pub fn update_list_constructor (list_type: UpdateListType) -> Box<&'static mut dyn UpdateList> {
//     match list_type {
//         UpdateListType::Classic => Box::new(&mut update_list::ULClassic::new()),
//         UpdateListType::MoveToFront => Box::new(&mut update_list::ULMoveToFront::new()),
//         UpdateListType::Transpose => Box::new(&mut update_list::ULTranspose::new()),
//         UpdateListType::Count => Box::new(&mut update_list::ULCount::new()),
//     }
// }
pub fn insert_update_list_to_vector (list_type: UpdateListType, vector: mut Vec<dyn UpdateList>) -> Vec<dyn UpdateList> {
    match list_type {
        UpdateListType::Classic => vector.push(&mut update_list::ULClassic::new()),
        UpdateListType::MoveToFront => vector.push(&mut update_list::ULMoveToFront::new()),
        UpdateListType::Transpose => vector.push(&mut update_list::ULTranspose::new()),
        UpdateListType::Count => vector.push(&mut update_list::ULCount::new()),
    }
    vector
}

// pub fn update_list_name (list_type: UpdateListType) -> &'static str {
//     match list_type {
//         UpdateListType::Classic => update_list::NAME_CLASSIC,
//         UpdateListType::MoveToFront => update_list::NAME_MOVE_TO_FRONT,
//         UpdateListType::Transpose => update_list::NAME_TRANSPOSE,
//         UpdateListType::Count => update_list::NAME_COUNT,
//     }
// }


// pub fn distribution_constructor(dist_type: DistributionType, low: u8, high: u8) -> Box<&'static dyn Distribution> {
//     match dist_type {
//         DistributionType::Uniform => Box::new(&distribution::Uniform::new(low, high)),
//         DistributionType::Geometric => Box::new(&distribution::Geometric::new(low, high)),
//         DistributionType::Harmonic => Box::new(&distribution::Harmonic::new(low, high)),
//         DistributionType::TwoHarmonic => Box::new(&distribution::TwoHarmonic::new(low, high)),
//     }
// }

// pub fn distribution_name(dist_type: DistributionType) -> &'static str {
//     match dist_type {
//         DistributionType::Uniform => "uniform",
//         DistributionType::Geometric => "geometric",
//         DistributionType::Harmonic => "harmonic",
//         DistributionType::TwoHarmonic => "twoharmonic",
//     }
// }


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

fn unbox<T>(value: Box<T>) -> T {
    *value
}

fn main() {
    // for _ in 0..10_000_000 {
    //     ul.access(ds.get());
    // }
    // ul.print();
    
    // for list_type in ULS {
    //     println!("{}", update_list::update_list_name(list_type));
    // }
    // for dist_type in DSS {
    //     println!("{}", distribution::distribution_name(dist_type));
    // }

    // Set up.
    // let mut update_lists = ULS.map(|list_type| update_list_constructor(list_type));
    // let update_lists_names = ULS.map(|list_type| update_list_name(list_type));
    // let distributions = DSS.map(|dist_type| distribution_constructor(dist_type, LOW, HIGH));
    // let distributions_names = DSS.map(|dist_type| distribution_name(dist_type));
    // let mut cost: f64;

    // Write set of ns to file, for future data analysis.
    // let mut filename = format!("ns.txt");
    // let output = File::create(filename);
    // let mut output = match output {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem creating ns file : {:?}", error),
    // };
    // for n in NS {
    //     write!(output, "{}\n", n).expect("Error writing NS file.");
    // }

    // Perform multiple experiments.
    // for idx_ul in 0..update_lists.len() {
    //     for idx_ds in 0..distributions.len() {
    //         filename = format!("{}_{}.txt", update_lists_names[idx_ul], distributions_names[idx_ds]);
    //         let output = File::create(filename);
    //         let mut output = match output {
    //             Ok(file) => file,
    //             Err(error) => panic!("Problem creating data file : {:?}", error),
    //         };
    //         for n in NS {
    //             cost = experiment_average_access_cost(unbox(update_lists[idx_ul]), unbox(distributions[idx_ds]), n, ITER);
    //             cost = update_lists[idx_ul].access(8_u8) as f64;
    //             write!(output, "{}\n", cost).expect("Error writing in data file.");
    //         }
    //     }
    // }
    // println!("Done!");

    let mut ul_vector: Vec<dyn UpdateList> = Vec::new();
    vector = insert_update_list_to_vector(UpdateListType::Classic, vector);
}
