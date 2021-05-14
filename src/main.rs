use crate::implements::guess_number::guess_num;
use crate::learning::about_enum::{about_if_let, show_about_enums};
use crate::learning::about_ownership::{borrowing_and_referencing, detail_of_ownership};
use crate::learning::about_struct::printing_traits;
use crate::learning::common_collections::details_of_collections;
use crate::learning::data_types::show_data_types;
use crate::learning::exception_handling::exception_handling_detail;
use crate::learning::mutability_details::show_mutability_details;

mod implements;
mod learning;

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
    exception_handling_detail();
}
