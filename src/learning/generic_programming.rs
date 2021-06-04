use std::borrow::Borrow;
use std::num;

use num_traits::real::Real;

/// using generic programming in functions:
///
/// ```rust
/// fn largest_i32(list: &[i32]) -> i32 {
///     let mut largest = list[0];
///
///     for &item in list.iter() {
///         if item > largest {
///             largest = item;
///         }
///     }
///
///     largest
/// }
///
/// fn largest_char(list: &[char]) -> char {
///     let mut largest = list[0];
///
///     for &item in list.iter() {
///         if item > largest {
///             largest = item;
///         }
///     }
///
///     largest
/// }
///
/// fn main() {
///     let number_list = vec![34, 50, 25, 100, 65];
///
///     let result = largest_i32(&number_list);
///     println!("The largest number is {}", result);
///
///     let char_list = vec!['y', 'm', 'a', 'q'];
///
///     let result = largest_char(&char_list);
///     println!("The largest char is {}", result);
/// }
/// ```
/// like above: different signature of largest
///
/// actually we can do like `fn largest<T>(list:&<T>)->T{}`
///


// todo!("https://stackoverflow.com/questions/59555044/how-to-define-different-implementations-for-an-associated-function-to-a-struct-t");
// todo!("https://doc.rust-lang.org/book/ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types");
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.content)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}