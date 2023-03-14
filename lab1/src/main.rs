mod update_list;
use update_list::UpdateList;
mod distribution;
use distribution::Distribution;

const _NS: [u32; 7] = [100, 500, 1000, 5000, 10000, 50000, 100000];

fn test_trait (ul: &mut dyn update_list::UpdateList, ds: &dyn distribution::Distribution) {
    ul.access(ds.get());
    ul.print();
}

fn main() {
    // let mut ul = update_list::ULClassic::new();
    // let mut ul = update_list::ULMoveToFront::new();
    // let mut ul = update_list::ULTranspose::new();
    let mut ul = update_list::ULCount::new();
    let ds = distribution::Uniform::new(1, 100);
    ul.print();
    for _ in 0..20 {
        test_trait(&mut ul, &ds);
    }
}
