use crate::{
    algorithms::insert_sort,
    implements::guess_number::guess_num,
    learning::about_enum::{about_if_let, show_about_enums},
    learning::about_ownership::{borrowing_and_referencing, detail_of_ownership},
    learning::about_struct::printing_traits,
    learning::command_line_io,
    learning::command_line_io::{get_args, get_file_content},
    learning::common_collections::details_of_collections,
    learning::data_types::show_data_types,
    learning::exception_handling::exception_handling_detail,
    learning::generic_programming::{NewsArticle, Summary, Tweet},
    learning::generic_programming::fixed_comparison,
    learning::mutability_details::show_mutability_details,
    leet_code::{atoi, no14, no48},
};
use crate::algorithms::solution::Solution;
use crate::leet_code::no1095::MountainArray;

mod algorithms;
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
    // get_args();
    // fixed_comparison();
    // get_file_content();
    let arr1 = vec![
        1, 5, 2
    ];
    let mountainArray = MountainArray {
        arr: arr1
    };
    println!("{}", Solution::find_in_mountain_array(3, &mountainArray));
    // println!("{}", Solution::min_pair_sum(nums));
}
