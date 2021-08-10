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

use std::collections::HashMap;

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
        if i == 7 {
            break;
        }
        print!(" {} ", vector[i]);
    }

    vector = vec!["also", "do", "for-each\n"];
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
        if i == string.len() {
            break;
        }
        string.pop();
    }
    let index = "index".to_string();
    string = "and it can access by using ".to_string() + &index[0..index.len()];
    string.clear();
    string = format!(
        "We can format Strings {} {}",
        "like".to_string(),
        "this".to_string()
    );

    /// It's a danger way to access string by using index, for example :
    /// ```
    /// let hello = "Здравствуйте";
    ///
    /// let s = &hello[0..1];
    /// ```
    /// this panics because bytes stored in string have a length in 2, so slices like [0..1] is invalid.
    // below is a valid way to access string by characters or bytes;
    for i in string.chars() {
        println!("By Chars element: {}", i);
    }

    /// HashMap
    /// HashMap is only used for get()ting values by key, mapping values to keys implemented by using Hash function, it cannot be accessed by using indexes
    /// Here are some examples
    let mut team_scores = HashMap::new();
    team_scores.insert(String::from("Yellow"), 50);
    team_scores.insert(String::from("Blue"), 60);
    /// another way to create a hashmap is to use a tuple vector's collect() function;
    /// by using zip() to create a tuple mapping vector
    let teams = vec!["Yellow", "Blue", "Black"];
    let scores = vec![50, 60, 40];
    let _team_score: HashMap<_, _> = teams.iter().zip(scores.iter()).collect();

    /// Updating existing keys
    team_scores.insert("Yellow".to_string(), 90);

    /// Insert only if the key is not exist
    team_scores.entry("Black".to_string()).or_insert(80);
    team_scores.entry("Yellow".to_string()).or_insert(85);
    println!("{:#?}", team_scores);

    // let team = vec!["Gold", "Grey", 10];
    // let score = vec![10, 20, 30];
    // let team_score_map = team.iter().zip(score.iter()).collect();
    //
    // println!("{:?}", team_score_map);
}
