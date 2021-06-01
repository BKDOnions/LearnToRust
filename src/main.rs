use crate::{
    implements::guess_number::guess_num,
    learning::about_enum::{about_if_let, show_about_enums},
    learning::about_ownership::{borrowing_and_referencing, detail_of_ownership},
    learning::about_struct::printing_traits,
    learning::common_collections::details_of_collections,
    learning::data_types::show_data_types,
    learning::exception_handling::exception_handling_detail,
    learning::mutability_details::show_mutability_details,
    // learning::generic_programming::largest
};

mod implements;
mod learning;
mod leet_code;

fn main() {
    // guess_num();
    // show_mutability_details();
    // show_data_types();
    // detail_of_ownership();
    // borrowing_and_referencing();
    // printing_traits();
    // show_about_enums();
    // about_if_let();
    // details_of_collections();
    // exception_handling_detail();
    let x = 32;
    println!("{}", is_number(&x));
}

fn is_number<T>(var: &T) -> i32 {
    match &var {
        i32 => 21,
        _ => 31
    }
}