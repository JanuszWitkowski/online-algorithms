mod update_list;
use update_list::UpdateList;

fn main() {
    // let mut ul: update_list::ULClassic;
    let mut ul = update_list::ULClassic::new();
    ul.access(9_u8);
    let access_cost: u8 = ul.access(5_u8);
    println!("Hello, world! {}", access_cost);
}
