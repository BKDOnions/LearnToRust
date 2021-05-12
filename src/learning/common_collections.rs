//! Common collections in Rust contains:
//!
//!     Vector:
//!         Vectors allow you to store more than one value in a single data structure that puts `all the values next to each other in memory`. Vectors can only store values of the same type. They are useful when you have a list of items, such as the lines of text in a file or the prices of items in a shopping cart.
//!         Vectors are implemented using generics;
//!
//!    String:
//!         Encapsulated by using `vec<u8>`
//!
//!    HashMap:
//!

pub fn details_of_collections() {
    // Vector
    // new() Vector w/o known type, Error: cannot infer type for type parameter `T` due to Vectors are implemented using generics;
    // let vector = Vec::new();
    // new() Vector:Vec<i64>
    let vector: Vec<i32> = Vec::new();
    println!("{:?}", vector);

    let mut vector = vec!["new()", "vectors", "in", "an", "array", "way"];
    println!("We can {:#?}", vector);

    while !vector.is_empty() {
        vector.pop();
    }

    vector.push("push()");
    vector.push("and");
    vector.push("pop()");
    vector.push("vectors");
    vector.push("in");
    vector.push("a");
    vector.push("stack");
    vector.push("way");

    println!("We can {:#?}", vector);
    print!("We can");
    vector = vec!["read", "vectors", "in", "a", "array", "way", "too"];
    for i in 0..10 {
        if i == 7 { break }
        print!(" {} ", vector[i]);
    }

    vector = vec!["also", "do", "for-each"];
    print!("\nWe can");
    for element in &vector {
        print!(" {} ", element);
    }

    // String :
    //      new(): new a String from nothing
    //      from(): new a String from literal like "foobar"
    let mut string = String::new();
    string = "new() String in a String way".to_string();
    string = String::from("new() String from its literal");
    string.clear();
    string.push_str("push another String");
    string.clear();
    string.push('I');
    string.push('n');
    string.push('a');
    string.push('S');
    string.push('t');
    string.push('a');
    string.push('c');
    string.push('k');
    string.push('W');
    string.push('a');
    string.push('y');
    for i in 0..20 {
        if i == string.len() { break; }
        string.pop();
    }
    let index = "index".to_string();
    string = "and it can access by using ".to_string() + &index[0..index.len()];
    string.clear();
    string = format!("We can format Strings {} {}", "like".to_string(), "this".to_string());
}
