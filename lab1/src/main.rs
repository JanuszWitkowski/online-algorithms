mod update_list;

fn main() {
    let ul = update_list::ULClassic::new();
    println!("Hello, world! {}", update_list::ul::access(5));
}
