use std::fmt::Display;

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

/// Trait bound
///
/// They have the same mean
/// ```rust
/// pub fn notify(item: impl Summary) {}
/// pub fn notify<T: Summary>(item: T) {}
/// ```
///
/// Above shows the limitation of parameter item should have implemented Summary
///
/// It seems longer though, it is used for simplify longer expressions
/// ```rust
/// pub fn notify(item1: impl Summary, item2: impl Summary) {}
/// pub fn notify<T: Summary>(item1: T, item2: T) {}
///
/// pub fn notify(item: impl Summary + Display) {}
/// pub fn notify<T: Summary + Display>(item: T) {}
/// ```
///
/// Using where can also simplify trait bound like:
/// ```rust
/// fn some_fn<T: Display + Clone, U:Clone + Debug>(t: T, u: U) -> i32 {}
/// fn some_fn<T,U>(t: T, u: U) -> i32
///     where T: Display + Clone,
///           U: Clone + Debug
/// {}
/// ```
// In order to compare with each other, you have to impl PartialOrd
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

pub fn fixed_comparison() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

/// Using trait bound to implement functions with limitations
///
struct Pair<T> {
    x: T,
    y: T,
}

// Generic implement
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// Only if T has implemented Display + PartialOrd can compare and display
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
