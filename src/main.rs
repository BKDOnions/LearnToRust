mod learning;
mod implements;

fn main() {
    println!("Hello, world!");
    implements::guess_number::guess_num();
    learning::mutability_details::show_mutability_details();
    learning::data_types::show_data_types();
}
