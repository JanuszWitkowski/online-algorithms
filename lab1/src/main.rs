mod update_list;
use update_list::UpdateList;
mod distribution;
use distribution::Distribution;
use std::fs::{File, create_dir_all};
use std::io::Write;

// Numbers of operations
const NS: [usize; 7] = [100, 500, 1000, 5000, 10000, 50000, 100000];

// Number of iterations
const ITER: usize = 1_000_000;
// const ITER: usize = 1;

// Range used in random number generation in {1, ..., 100}
const LIMIT: u8 = 100;


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

fn run_experiments (ns: &[usize], iter: usize, limit: u8) {
    // Set up.
    let mut ul_classic = update_list::ULClassic::new();
    let mut ul_move_to_front = update_list::ULMoveToFront::new();
    let mut ul_transpose = update_list::ULTranspose::new();
    let mut ul_count = update_list::ULCount::new();
    let ds_uniform = distribution::Uniform::new(limit);
    let ds_geometric = distribution::Geometric::new(limit);
    let ds_harmonic = distribution::Harmonic::new(limit);
    let ds_two_harmonic = distribution::TwoHarmonic::new(limit);
    let update_lists: [&mut dyn UpdateList; 4] = [&mut ul_classic, &mut ul_move_to_front, &mut ul_transpose, &mut ul_count];
    let distributions: [&dyn Distribution; 4] = [&ds_uniform, &ds_geometric, &ds_harmonic, &ds_two_harmonic];
    let mut cost: f64;

    // Write set of ns to file, for future data analysis.
    match create_dir_all("results") {
        Err(e) => panic!("Could not create directory! {:?}", e),
        _ => (),
    };
    let mut filename = format!("results/ns.txt");
    let output = File::create(filename);
    let mut output = match output {
        Ok(file) => file,
        Err(error) => panic!("Problem creating ns file : {:?}", error),
    };
    for n in ns {
        write!(output, "{}\n", n).expect("Error writing NS file.");
    }

    // Perform multiple experiments.
    for ul in update_lists {
        for ds in distributions {
            filename = format!("results/data_{}_{}.txt", ul.name(), ds.name());
            println!("Writing {}...", filename);
            let output = File::create(filename);
            let mut output = match output {
                Ok(file) => file,
                Err(error) => panic!("Problem creating data file : {:?}", error),
            };
            for n in ns {
                cost = experiment_average_access_cost(ul, ds, *n, iter);
                write!(output, "{}\n", cost).expect("Error writing in data file.");
            }
        }
    }
    println!("Done!");
}


fn expected_value (ds: &dyn distribution::Distribution, iter: usize) -> f64 {
    let mut sum: f64 = 0.0;
    for _ in 0..iter {
        sum += ds.get() as f64;
    }
    return sum / (iter as f64);
}

fn run_ev (iter: usize, limit: u8) {
    let ds_uniform = distribution::Uniform::new(limit);
    let ds_geometric = distribution::Geometric::new(limit);
    let ds_harmonic = distribution::Harmonic::new(limit);
    let ds_two_harmonic = distribution::TwoHarmonic::new(limit);
    let ev_uniform = expected_value(&ds_uniform, iter);
    let ev_geometric = expected_value(&ds_geometric, iter);
    let ev_harmonic = expected_value(&ds_harmonic, iter);
    let ev_twoharmonic = expected_value(&ds_two_harmonic, iter);
    println!("uni: {}", ev_uniform);
    println!("geo: {}", ev_geometric);
    println!("harm: {}", ev_harmonic);
    println!("2harm: {}", ev_twoharmonic);
}


fn main() -> std::io::Result<()> {
    run_ev(100_000, LIMIT);
    run_experiments(&NS, ITER, LIMIT);
    Ok(())
}
