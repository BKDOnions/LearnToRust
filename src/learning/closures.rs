//! expansive calculations
//!
//! Using closures to create abstract of behavior
//! This function is using [std::thread::sleep()] to sleep the thread in order to simulate a expansive calculation
//! ```rust
//! fn simulated_expansive_calculation(intensity: u32) ->u32{
//!     println!("Calculating slowly");
//!     thread::sleep(Duration::from_secs(2));
//!     intensity
//!}
//! ```
//! Using assigned variables as user inputs
//! ```rust
//! fn main() {
//!     let simulated_user_specified_value = 10;
//!     let simulated_random_number = 7;
//!
//!     generate_workout(
//!         simulated_user_specified_value,
//!         simulated_random_number
//!     );
//! }
//! ```
//! function generate workout details
//! ```rust
//! fn generate_workout(intensity: u32, random_number: u32) {
//!     if intensity < 25 {
//!         println!(
//!             "Today, do {} pushups!",
//!             simulated_expensive_calculation(intensity)
//!         );
//!         println!(
//!             "Next, do {} situps!",
//!             simulated_expensive_calculation(intensity)
//!         );
//!     } else {
//!         if random_number == 3 {
//!             println!("Take a break today! Remember to stay hydrated!");
//!         } else {
//!             println!(
//!                 "Today, run for {} minutes!",
//!                simulated_expensive_calculation(intensity)
//!             );
//!         }
//!     }
//! }
//! ```
//! Using functions to reconstruct generate workout detail :
//! ```rust
//!
//! #![allow(unused)]
//! fn main() {
//!     use std::thread;
//!     use std::time::Duration;
//!
//!     fn simulated_expensive_calculation(num: u32) -> u32 {
//!         println!("calculating slowly...");
//!         thread::sleep(Duration::from_secs(2));
//!         num
//!     }
//!
//!     fn generate_workout(intensity: u32, random_number: u32) {
//!         let expensive_result =
//!             simulated_expensive_calculation(intensity);
//!
//!         if intensity < 25 {
//!             println!(
//!                 "Today, do {} pushups!",
//!                 expensive_result
//!             );
//!             println!(
//!                 "Next, do {} situps!",
//!                 expensive_result
//!             );
//!         } else {
//!             if random_number == 3 {
//!                 println!("Take a break today! Remember to stay hydrated!");
//!             } else {
//!                 println!(
//!                     "Today, run for {} minutes!",
//!                     expensive_result
//!                 );
//!             }
//!         }
//!     }
//! }
//! ```
//! Follow up is the example using closure to reconstruct
//!
use std::thread;
use std::time::Duration;

pub fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_calculation_closure = |num| {
        println!("Calculating slowly");
        thread::sleep(Duration::from_secs(2));
        num
    };
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_calculation_closure(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_calculation_closure(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_calculation_closure(intensity)
            );
        }
    }
}

/// Closure type assigning and deducing
///
/// Type assigning :
///
/// ```rust
/// let expensive_closure = |num: u32| -> u32 {
///     println!("calculating slowly...");
///     thread::sleep(Duration::from_secs(2));
///     num
/// };
/// ```
/// type deducing
/// ```rust
/// fn  add_one_v1   (x: u32) -> u32 { x + 1 }  //normal function like
/// let add_one_v2 = |x: u32| -> u32 { x + 1 }; // closure w/ type assigned
/// let add_one_v3 = |x|             { x + 1 };
/// let add_one_v4 = |x|               x + 1  ;
/// ```
/// ```rust
/// let example = |x| x;
/// let s = example(String::from("str")); // The example closure is String type
/// let s = example(1); // panic bc example is not a string type
/// ```
pub fn get_args() {}
