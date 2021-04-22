use std::io;

mod learning;
mod implements;

fn main() {
    println!("Hello, world!");
    learning::data_types::show_data_types();
    implements::guess_number::guess_num();
}
