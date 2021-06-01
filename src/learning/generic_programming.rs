extern crate num_traits;

use std::borrow::Borrow;
use std::num;

use self::num_traits::Num;

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
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    pub fn get_x(&self) -> &T {
        &self.x
    }
    pub fn get_y(&self) -> &T {
        &self.y
    }
    // pub fn distance<T>(&self, &point: Point<T>) -> Point<T>{
    //
    // }
    pub fn from(x: T, y: T) -> Point<T> {
        Point {
            x,
            y,
        }
    }
}

