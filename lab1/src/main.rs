mod update_list;
use update_list::UpdateList;

fn test_trait (ul: &mut dyn update_list::UpdateList) {
    ul.print();
}

fn main() {
    // let mut ul = update_list::ULClassic::new();
    // let mut ul = update_list::ULMoveToFront::new();
    // let mut ul = update_list::ULTranspose::new();
    let mut ul = update_list::ULCount::new();
    test_trait(&mut ul);
    ul.access(1_u8);
    test_trait(&mut ul);
    ul.access(2_u8);
    test_trait(&mut ul);
    ul.access(2_u8);
    test_trait(&mut ul);
    ul.access(2_u8);
    test_trait(&mut ul);
    let access_cost: u8 = ul.access(3_u8);
    println!("Hello, world! {}", access_cost);
    test_trait(&mut ul);
}
